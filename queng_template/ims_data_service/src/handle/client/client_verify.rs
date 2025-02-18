use crate::service::Service;
use common_errors::MessageProcessingError;
use trait_data_integration::ImsDataIntegration;

impl<Integration: ImsDataIntegration> Service<Integration> {
    //
    /// Checks if a client with the specified ID is logged in.
    ///
    /// This method checks the client producers map to verify if a client with the given ID
    /// has an active producer, indicating they are logged in.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The unique identifier of the client to check
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing:
    /// * `Ok(true)` if the client is logged in
    /// * `Ok(false)` if the client is not logged in
    ///
    /// # Errors
    ///
    /// Returns a `MessageProcessingError` if:
    /// * Failed to acquire read lock on client producers map
    /// * Lock is poisoned due to a panic in another thread
    pub(crate) async fn check_client_login(
        &self,
        client_id: u16,
    ) -> Result<bool, MessageProcessingError> {
        let client_db = self.client_producers().read().await;

        Ok(client_db.contains_key(&client_id))
    }

    /// Checks if a client has a valid client id that is allowed to log in.
    ///
    /// This function should be overriden by the implementation of the service.
    /// The default implementation allows clients with an ID > 99.
    ///
    /// # Parameters
    ///
    /// - `client_id`: The ID of the client to check
    ///
    /// # Returns
    ///
    /// `true` if the client is allowed, `false` otherwise
    pub(crate) fn check_valid_client_id(&self, client_id: u16) -> bool {
        client_id >= 100
    }

    /// Checks if a client with the specified ID is allowed to stream data
    pub(crate) fn check_if_client_id_authorized(&self, _client_id: u16) -> bool {
        true
    }
}
