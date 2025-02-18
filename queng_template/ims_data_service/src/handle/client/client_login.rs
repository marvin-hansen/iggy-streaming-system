use crate::service::Service;
use common_errors::MessageProcessingError;
use iggy::stream_builder::IggyStreamProducer;
use ims_iggy_config;
use sbe_types::ClientErrorType;
use std::sync::Arc;
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
    /// - `ClientNotAuthorized` if the client is not allowed to log in
    /// - `ClientAlreadyLoggedIn` if the client is already logged in
    /// - `ClientNotAuthorized` if the client is not allowed to stream data
    /// - `ClientLogInError` if there was an issue creating the message stream or logging in the client
    ///
    pub(crate) async fn client_login(
        &self,
        client_id: u16,
    ) -> Result<(), (ClientErrorType, MessageProcessingError)> {
        //
        self.dbg_print(&format!("Validate login for client id {}", client_id));
        self.validate_client_login_request(client_id).await?;

        self.dbg_print(&format!(
            "Create a new stream config for client with id {}",
            client_id
        ));
        let exchange_id = self.exchange_id();
        let client_data_stream_config =
            ims_iggy_config::ims_data_iggy_config(client_id, exchange_id)
                .producer_config()
                .to_owned();

        self.dbg_print(&format!(
            "Create a new message producer for client with id {}",
            client_id
        ));
        // RW lock the iggy client
        let client_guard = self.iggy_client().write().await;
        let res = IggyStreamProducer::build(&client_guard, &client_data_stream_config).await;
        drop(client_guard); // Unlock the iggy client
        let producer = match res {
            // The producer creates stream and topic if it doesn't exist
            Ok(producer) => producer,
            Err(err) => {
                return Err((
                    ClientErrorType::ClientLogInError,
                    MessageProcessingError(format!(
                        "Failed to create a new message producer for client with id {} due to error {}",
                        client_id,
                        err.as_string()
                    )),
                ));
            }
        };

        self.dbg_print(&format!("Login in client with id {}", client_id));
        // RW lock the client_data_producers hashmap
        let mut client_producers_guard = self.client_producers().write().await;
        let res = client_producers_guard.insert(client_id, Arc::new(producer));
        drop(client_producers_guard); // Unlock the client_data_producers hashmap
        if res.is_none() {
            return Err((
                ClientErrorType::ClientLogInError,
                MessageProcessingError(format!("Failed to login client with id {}", client_id,)),
            ));
        };

        // RW lock the client_configs hashmap
        let mut client_configs_guard = self.client_configs().write().await;
        let res = client_configs_guard.insert(client_id, client_data_stream_config);
        drop(client_configs_guard); // Unlock the client_configs hashmap
        if res.is_none() {
            return Err((
                ClientErrorType::ClientLogInError,
                MessageProcessingError(format!(
                    "Failed to store config for client with id {}",
                    client_id
                )),
            ));
        };

        self.dbg_print(&format!(
            "Client login successful for client with id {}",
            client_id
        ));

        Ok(())
    }
}
