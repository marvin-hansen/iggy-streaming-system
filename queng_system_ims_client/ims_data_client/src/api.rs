use crate::error::ImsClientError;
use crate::ImsDataClient;
use sbe_types::DataType;

impl ImsDataClient {
    pub async fn login(&self) -> Result<(), ImsClientError> {
        self.client_login().await
    }

    pub async fn logout(&self) -> Result<(), ImsClientError> {
        self.client_logout().await
    }

    pub async fn stop_trade_data(&self, symbol_id: String) -> Result<(), ImsClientError> {
        self.client_stop_data(symbol_id, DataType::TradeData).await
    }

    pub async fn stop_ohlcv_data(&self, symbol_id: String) -> Result<(), ImsClientError> {
        self.client_stop_data(symbol_id, DataType::OHLCVData).await
    }

    pub async fn stop_all_data(&self) -> Result<(), ImsClientError> {
        self.client_stop_all_data().await
    }
}
