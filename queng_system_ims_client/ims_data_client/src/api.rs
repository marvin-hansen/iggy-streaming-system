use crate::error::ImsClientError;
use crate::ImsDataClient;

impl ImsDataClient {
    pub async fn login(&self) -> Result<(), ImsClientError> {
        self.client_login().await
    }

    pub async fn logout(&self) -> Result<(), ImsClientError> {
        self.client_logout().await
    }
}