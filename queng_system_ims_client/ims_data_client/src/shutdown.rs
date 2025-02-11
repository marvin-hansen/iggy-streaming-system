use crate::error::ImsClientError;
use crate::ImsDataClient;
use iggy::client::Client;

impl ImsDataClient {
    /// Shutdown the IMS data client.
    pub(crate) async fn client_shutdown(&self) -> Result<(), ImsClientError> {
        // Send cancellation signals to consumers
        if let Some(tx_control) = self.tx_control_consumer.write().await.take() {
            let _ = tx_control.send(());
            self.dbg_print("Sent cancellation signal to control consumer");
        }

        if let Some(tx_data) = self.tx_data_consumer.write().await.take() {
            let _ = tx_data.send(());
            self.dbg_print("Sent cancellation signal to data consumer");
        }

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
