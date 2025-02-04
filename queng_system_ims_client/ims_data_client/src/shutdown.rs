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

        // Delete control stream
        let control_stream_id = &self.control_producer().stream_id();
        let control_topic_id = &self.control_producer().topic_id();
        message_shared::cleanup(
            &self.iggy_client_control,
            control_stream_id,
            control_topic_id,
        )
            .await
            .expect("Failed to delete control stream");

        // Delete data topic
        let data_stream_id = &self.data_producer().stream_id();
        let data_topic_id = &self.data_producer.topic_id();
        message_shared::cleanup(&self.iggy_client_control, data_stream_id, data_topic_id)
            .await
            .expect("Failed to delete data stream");

        // Shutdown iggy client for control stream
        match &self.iggy_client_control.shutdown().await {
            Ok(_) => {}
            Err(err) => return Err(ImsClientError::FailedToShutdownIggyClient(err.to_string())),
        }

        // Shutdown iggy client for data stream
        match &self.iggy_client_data.shutdown().await {
            Ok(_) => {}
            Err(err) => return Err(ImsClientError::FailedToShutdownIggyClient(err.to_string())),
        }

        Ok(())
    }
}
