use crate::error::ImsClientError;
use crate::ImsDataClient;
use iggy::client::Client;

impl ImsDataClient {
    /// Shutdown the IMS data client.
    pub(crate) async fn client_shutdown(&self) -> Result<(), ImsClientError> {
        // self.dbg_print("Delete data topic");
        // let data_stream_id = &self.data_producer().stream();
        // let data_topic_id = &self.data_producer.topic();
        // match &self
        //     .iggy_client_data
        //     .client()
        //     .read()
        //     .await
        //     .delete_topic(data_stream_id, data_topic_id)
        //     .await
        // {
        //     Ok(_) => (),
        //     Err(err) => return Err(ImsClientError::FailedToDeleteIggyTopic(err.to_string())),
        // }
        //
        // self.dbg_print("Delete data stream");
        // match &self
        //     .iggy_client_data
        //     .client()
        //     .read()
        //     .await
        //     .delete_stream(data_stream_id)
        //     .await
        // {
        //     Ok(_) => (),
        //     Err(err) => return Err(ImsClientError::FailedToDeleteIggyStream(err.to_string())),
        // }

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
