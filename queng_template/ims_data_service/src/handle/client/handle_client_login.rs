use crate::service::Service;
use common_errors::MessageProcessingError;
use trait_data_integration::ImsDataIntegration;

impl<Integration: ImsDataIntegration> Service<Integration> {
    /// Handles a client login message by validating the client ID and logging them in.
    ///
    /// Gets the client's control channel, checks if they are not already logged in, and logs them in if allowed.
    /// Sends back any errors over the control channel.
    ///
    /// # Parameters
    ///
    /// - `client_login_msg`: The incoming ClientLoginMessage from the client.
    ///
    /// # Returns
    ///
    /// Result with no value if successful, or a MessageProcessingError if an error occurs.
    ///
    /// # Errors
    ///
    /// - MessageProcessingError if there is an issue checking the client's login status or logging them in.
    ///
    pub(crate) async fn handle_client_login(
        &self,
        client_id: u16,
    ) -> Result<(), MessageProcessingError> {
        self.dbg_print("handle_client_login");

        match self.client_login(client_id).await {
            Ok(_) => {}
            Err((client_error_type, err)) => {
                eprintln!(
                    "[handle_client_login] ClientLogInError: {:?}",
                    err.to_string()
                );

                match self.send_client_error(client_id, client_error_type).await {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!(
                            "[handle_client_login] ClientLogInError: {:?}",
                            err.to_string()
                        );
                    }
                }
            }
        }

        Ok(())
    }
}
