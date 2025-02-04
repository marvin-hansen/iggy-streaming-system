use crate::error::ImsClientError;
use crate::ImsDataClient;
use iggy::client::Client;

impl ImsDataClient {
    /// Shutdown the IMS data client.
    pub(crate) async fn client_shutdown(&self) -> Result<(), ImsClientError> {

        self.dbg_print("Shutdown iggy control consumer");
        self.tx_control_consumer.cancel();
        self.dbg_print("Shutdown iggy data consumer");
        self.tx_data_consumer.cancel();

        self.dbg_print("Delete control stream and topic");
        let control_stream_id = &self.control_producer().stream_id();
        let control_topic_id = &self.control_producer().topic_id();
        message_shared::cleanup(
            &self.iggy_client_control,
            control_stream_id,
            control_topic_id,
        )
            .await
            .expect("Failed to delete control stream");

        self.dbg_print("Delete data stream and topic");
        let data_stream_id = &self.data_producer().stream_id();
        let data_topic_id = &self.data_producer.topic_id();
        message_shared::cleanup(&self.iggy_client_control, data_stream_id, data_topic_id)
            .await
            .expect("Failed to delete data stream");

        self.dbg_print("Shutdown iggy client for control stream");
        match &self.iggy_client_control.shutdown().await {
            Ok(_) => {}
            Err(err) => return Err(ImsClientError::FailedToShutdownIggyClient(err.to_string())),
        }

        self.dbg_print("Shutdown iggy client for data stream");
        match &self.iggy_client_data.shutdown().await {
            Ok(_) => {}
            Err(err) => return Err(ImsClientError::FailedToShutdownIggyClient(err.to_string())),
        }

        Ok(())
    }
}
