use crate::service::Service;
use common_errors::MessageProcessingError;
use sbe_types::ClientErrorType;
use trait_data_integration::ImsDataIntegration;

impl<Integration: ImsDataIntegration> Service<Integration> {
    //
    /// Logs a client out by shutting down the message stream and removing the client from the hashmap of client data producers.
    ///
    /// Checks if the client is already logged in, and if so, shuts down the message stream and removes the client from the hashmap of client data producers.
    /// If the client is not logged in, a `ClientNotLoggedIn` error is returned.
    ///
    /// # Parameters
    ///
    /// * `client_id`: The ID of the client to log out.
    ///
    /// # Returns
    ///
    /// A `Result` with no value if the client was logged out successfully,
    /// or a `(ClientErrorType, MessageProcessingError)` if an error occurred.
    ///
    /// # Errors
    ///
    /// - `ClientErrorType::ClientNotLoggedIn` if the client is not logged in
    /// - `ClientErrorType::ClientShutdownError` if there was an issue shutting down the message stream
    /// - `ClientErrorType::ClientLogOutError` if there was an issue logging out the client
    pub(crate) async fn client_logout(
        &self,
        client_id: u16,
    ) -> Result<(), (ClientErrorType, MessageProcessingError)> {
        self.dbg_print(&format!(
            "Checking if client with id {} is logged in",
            client_id
        ));
        let is_logged_in = match self.check_client_login(client_id).await {
            Ok(exists) => exists,
            Err(err) => {
                return Err((
                    ClientErrorType::ClientLogOutError,
                    MessageProcessingError(format!(
                        "Failed to check if client with id {} is logged in due to error: { }",
                        client_id, err
                    )),
                ))
            }
        };

        if !is_logged_in {
            self.dbg_print(&format!("Client with id {} is not logged in", client_id));
            return Err((
                ClientErrorType::ClientNotLoggedIn,
                MessageProcessingError(format!("Client with id {} is not logged in", client_id)),
            ));
        }

        // lock the client_data_producers hashmap
        let client_data_producers = self.client_producers().write().await;

        //
        // Re-write
        //
        // // get the client data producer from the hashmap
        // match client_data_producers.get(&client_id) {
        //     Some(message_stream) => {
        //         self.dbg_print(&format!(
        //             "Stopping the message stream for client with id {}",
        //             client_id
        //         ));
        //         match message_stream.iggy_client().shutdown().await {
        //             Ok(_) => {}
        //             Err(e) => {
        //                 MessageProcessingError(format!(
        //                     "Failed to shutdown client with id {} due to error: {}",
        //                     client_id, e
        //                 ));
        //             }
        //         }
        //
        //         self.dbg_print(&format!("Logout client with id {}", client_id));
        //         match client_data_producers.remove(&client_id) {
        //             Some(_) => {}
        //             None => {
        //                 return Err((
        //                     ClientErrorType::ClientLogOutError,
        //                     MessageProcessingError(format!(
        //                         "Failed to logout client with id {}",
        //                         client_id,
        //                     )),
        //                 ))
        //             }
        //         };
        //     }
        //
        //     None => {
        //         return Err((
        //             ClientErrorType::ClientLogOutError,
        //             MessageProcessingError(format!(
        //                 "Failed to logout client with id {}",
        //                 client_id,
        //             )),
        //         ))
        //     }
        // };

        // Unlock the client_data_producers hashmap
        drop(client_data_producers);

        Ok(())
    }
}
