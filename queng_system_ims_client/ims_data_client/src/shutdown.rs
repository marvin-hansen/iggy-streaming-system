use crate::error::ImsClientError;
use crate::ImsDataClient;
use iggy::client::Client;

impl ImsDataClient {
    pub async fn shutdown(self) -> Result<(), ImsClientError> {
        // Shutdown iggy client
        let iggy_client = self.control_client;

        match iggy_client.shutdown().await {
            Ok(_) => {}
            Err(err) => return Err(ImsClientError::FailedToShutdownIggyClient(err.to_string())),
        }

        Ok(())
    }
}
