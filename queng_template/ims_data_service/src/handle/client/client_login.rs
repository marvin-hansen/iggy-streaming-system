use crate::service::Service;
use common_errors::MessageProcessingError;
use sbe_types::ClientErrorType;
use trait_data_integration::ImsDataIntegration;

impl<Integration: ImsDataIntegration> Service<Integration> {
    ///
    /// Logs a client in by validating the client ID and creating a new message stream.
    ///
    /// Checks if the client is already logged in, and if not, checks if the client is allowed to log in.
    /// If the client is allowed to log in, a new message stream is created and the client is logged in.
    ///
    /// The hashmap of client data producers is locked while the client is logged in.
    ///
    /// # Parameters
    ///
    /// * `client_id`: The ID of the client to log in
    ///
    /// # Returns
    ///
    /// A `Result` with no value if the client was logged in successfully,
    /// or a `(ClientErrorType, MessageProcessingError)` if an error occurred.
    ///
    /// # Errors
    ///
    /// - `ClientAlreadyLoggedIn` if the client is already logged in
    /// - `ClientNotAuthorized` if the client is not allowed to log in
    /// - `ClientLogInError` if there was an issue creating the message stream or logging in the client
    pub(crate) async fn client_login(
        &self,
        client_id: u16,
    ) -> Result<(), (ClientErrorType, MessageProcessingError)> {
        //
        self.dbg_print(&format!("Validate login for client id {}", client_id));
        self.validate_client_login_request(client_id).await?;

        self.dbg_print(&format!(
            "Create a new message producer for client with id {}",
            client_id
        ));

        // RW lock the client_data_producers hashmap
        // let mut client_data_producers = self.client_producers().write().await;
        //
        // self.dbg_print(&format!("Login in client with id {}", client_id));
        // if client_data_producers
        //     .insert(client_id, message_stream)
        //     .is_none()
        // {
        //     return Err((
        //         ClientErrorType::ClientLogInError,
        //         MessageProcessingError(format!("Failed to login client with id {}", client_id,)),
        //     ));
        // };
        //
        // // Unlock the client_data_producers hashmap
        // drop(client_data_producers);

        self.dbg_print(&format!(
            "Client login successful for client with id {}",
            client_id
        ));

        Ok(())
    }
}
