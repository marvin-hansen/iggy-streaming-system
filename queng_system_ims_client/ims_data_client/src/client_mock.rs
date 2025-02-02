use crate::error::ImsClientError;
use crate::{ImsDataClientTrait, ImsDataMockClient};
use async_trait::async_trait;
use common_data_bar::TimeResolution;

#[async_trait]
impl ImsDataClientTrait for ImsDataMockClient {
    async fn login(&self) -> Result<(), ImsClientError> {
        Ok(())
    }

    async fn logout(&self) -> Result<(), ImsClientError> {
        Ok(())
    }

    async fn start_trade_data(&self, _symbol_id: String) -> Result<(), ImsClientError> {
        Ok(())
    }

    async fn start_ohlcv_data(
        &self,
        _symbol_id: String,
        _time_resolution: TimeResolution,
    ) -> Result<(), ImsClientError> {
        Ok(())
    }

    async fn stop_trade_data(&self, _symbol_id: String) -> Result<(), ImsClientError> {
        Ok(())
    }

    async fn stop_ohlcv_data(&self, _symbol_id: String) -> Result<(), ImsClientError> {
        Ok(())
    }

    async fn stop_all_data(&self) -> Result<(), ImsClientError> {
        Ok(())
    }

    async fn shutdown(&self) -> Result<(), ImsClientError> {
        Ok(())
    }
}
