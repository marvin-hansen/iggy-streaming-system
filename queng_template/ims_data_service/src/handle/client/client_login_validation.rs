use crate::service::Service;
use common_errors::MessageProcessingError;
use sbe_types::ClientErrorType;
use trait_data_integration::ImsDataIntegration;

impl<Integration: ImsDataIntegration> Service<Integration> {
    /// Checks if a client is allowed to log in.
    ///
    /// # Parameters
    ///
    /// - `client_id`: The ID of the client to check
    ///
    /// # Errors
    ///
    /// - `ClientNotAuthorized` if the client is not allowed to log in
    /// - `ClientAlreadyLoggedIn` if the client is already logged in
    /// - `ClientNotAuthorized` if the client is not allowed to stream data
    /// - `ClientLogInError` if any of the checks fail due to an internal error
    /// - `ClientLogInError` if any of the checks fail due to an internal error
    ///
    pub(crate) async fn validate_client_login_request(
        &self,
        client_id: u16,
    ) -> Result<(), (ClientErrorType, MessageProcessingError)> {
        self.dbg_print(&format!(
            "Checking if client with id {} is allowed to log in",
            client_id
        ));

        if !self.check_valid_client_id(client_id) {
            return Err((
                ClientErrorType::ClientNotAuthorized,
                MessageProcessingError(format!(
                    "Client with id {} not allowed to log in",
                    client_id
                )),
            ));
        }

        self.dbg_print(&format!(
            "Checking if client with id {} is already logged in",
            client_id
        ));
        let already_logged_in = match self.check_client_login(client_id).await {
            Ok(exists) => exists,
            Err(err) => {
                return Err((
                    ClientErrorType::ClientLogInError,
                    MessageProcessingError(format!(
                        "Failed to check if client with id {} is logged in due to error: { }",
                        client_id, err
                    )),
                ));
            }
        };

        if already_logged_in {
            self.dbg_print(&format!("Client with id {} already logged in", client_id));
            return Err((
                ClientErrorType::ClientAlreadyLoggedIn,
                MessageProcessingError(format!("Client with id {} already logged in", client_id)),
            ));
        }

        self.dbg_print(&format!(
            "Checking if client with id {} is authorized to stream data",
            client_id
        ));

        if !self.check_if_client_id_authorized(client_id) {
            return Err((
                ClientErrorType::ClientNotAuthorized,
                MessageProcessingError(format!(
                    "Client with id {} not authorized to stream data",
                    client_id
                )),
            ));
        }

        Ok(())
    }
}
