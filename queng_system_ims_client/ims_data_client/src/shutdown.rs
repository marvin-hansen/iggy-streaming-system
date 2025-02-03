use crate::error::ImsClientError;
use crate::ImsDataClient;
use iggy::client::Client;

impl ImsDataClient {
    /// Shutdown the IMS data client.
    ///
    /// This will shut down the control topic consume and the Iggy client.
    pub(crate) async fn client_shutdown(&self) -> Result<(), ImsClientError> {
        // shutdown iggy control consumer
        let control_handler = &self.handler_control_consumer;
        control_handler.abort();

        // shutdown iggy data consumer
        let data_handler = &self.handler_data_consumer;
        data_handler.abort();

        // Shutdown iggy client
        match &self.iggy_client.shutdown().await {
            Ok(_) => {}
            Err(err) => return Err(ImsClientError::FailedToShutdownIggyClient(err.to_string())),
        }

        Ok(())
    }
}
