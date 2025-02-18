use crate::service::Service;
use common_errors::MessageProcessingError;
use trait_data_integration::ImsDataIntegration;

impl<Integration: ImsDataIntegration> Service<Integration> {
    /// Handles a client logout message.
    ///
    /// Logs out the client by shutting down the message stream and removing the client from the hashmap of client data producers.
    ///
    /// # Parameters
    ///
    /// * `client_logout_msg`: The `ClientLogoutMessage` to handle
    ///
    /// # Returns
    ///
    /// A `Result` with no value if the client was logged out successfully,
    /// or a `MessageProcessingError` if an error occurred.
    ///
    pub(crate) async fn handle_client_logout(
        &self,
        client_id: u16,
    ) -> Result<(), MessageProcessingError> {
        self.dbg_print("handle_client_logout");

        match self.client_logout(client_id).await {
            Ok(_) => {}
            Err((client_error_type, err)) => {
                // Print error
                println!(
                    "[handle_client_logout]: ClientLogOutError: {:?}",
                    err.to_string()
                );

                match self.send_client_error(client_id, client_error_type).await {
                    Ok(_) => {}
                    Err(err) => {
                        println!(
                            "[handle_client_logout]: ClientLogOutError: {:?}",
                            err.to_string()
                        );

                        return Err(err);
                    }
                }
            }
        }

        Ok(())
    }
}
