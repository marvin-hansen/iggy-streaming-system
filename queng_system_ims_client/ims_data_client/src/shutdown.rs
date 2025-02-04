use crate::error::ImsClientError;
use crate::ImsDataClient;
use iggy::client::Client;

// tokio-oneshot example
// https://github.com/DennisZhangOiler/tokio-oneshot/blob/main/src/main.rs

impl ImsDataClient {
    /// Shutdown the IMS data client.
    ///
    /// This will shut down the control topic consume and the Iggy client.
    pub(crate) async fn client_shutdown(&self) -> Result<(), ImsClientError> {
        // shutdown iggy control consumer
        let guard = {
            let mut guard = self.tx_control_consumer.lock().unwrap();
            guard.take()
        };

        if let Some(tx_control_sender) = guard {
            match tx_control_sender.send(()) {
                Ok(_) => {}
                Err(_) => {
                    return Err(ImsClientError::FailedToShutdownIggyConsumer(
                        "Failed to send shutdown control message consumer".to_string(),
                    ))
                }
            }
        }

        // shutdown iggy data consumer
        let guard = {
            let mut guard = self.tx_data_consumer.lock().unwrap();
            guard.take()
        };

        if let Some(tx_data_sender) = guard {
            match tx_data_sender.send(()) {
                Ok(_) => {}
                Err(_) => {
                    return Err(ImsClientError::FailedToShutdownIggyConsumer(
                        "Failed to send shutdown control data consumer".to_string(),
                    ))
                }
            }
        }

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
