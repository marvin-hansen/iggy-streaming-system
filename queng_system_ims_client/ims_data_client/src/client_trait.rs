use crate::error::ImsClientError;
use async_trait::async_trait;
use common_data_bar::TimeResolution;
use enum_dispatch::enum_dispatch;

#[async_trait]
#[enum_dispatch(ImsDataClientSelector)]
pub trait ImsDataClientTrait {
    /// Login to the IMS Data client
    ///
    /// This function will perform a login to the ImsDataClient.
    /// The username and password are taken from the configuration.
    ///
    /// Errors:
    /// * `ImsClientError`: A generic error type for theImsDataClient
    ///
    async fn login(&self) -> Result<(), ImsClientError>;
    /// Logout from the IMS Data client
    ///
    /// This function will perform a logout from the  IMS Data client
    ///
    /// Errors:
    /// * `ImsClientError`: A generic error type for the  IMS Data client
    ///
    async fn logout(&self) -> Result<(), ImsClientError>;
    /// Start receiving trade data for the given symbol
    ///
    /// Parameters:
    /// * `symbol_id`: The id of the symbol for which to start receiving trade data
    ///
    /// Errors:
    /// * `ImsClientError`: A generic error type for the  IMS Data client
    ///
    async fn start_trade_data(&self, symbol_id: String) -> Result<(), ImsClientError>;
    /// Start receiving OHLCV data for the given symbol
    ///
    /// Parameters:
    /// * `symbol_id`: The id of the symbol for which to start receiving OHLCV data
    /// * `time_resolution`: The time resolution of the OHLCV data
    ///
    /// Errors:
    /// * `ImsClientError`: A generic error type for the  IMS Data client
    ///
    async fn start_ohlcv_data(
        &self,
        symbol_id: String,
        time_resolution: TimeResolution,
    ) -> Result<(), ImsClientError>;
    /// Stop receiving trade data for the given symbol
    ///
    /// Parameters:
    /// * `symbol_id`: The id of the symbol for which to stop receiving trade data
    ///
    /// Errors:
    /// * `ImsClientError`: A generic error type for the  IMS Data client
    ///
    async fn stop_trade_data(&self, symbol_id: String) -> Result<(), ImsClientError>;
    /// Stop receiving OHLCV data for the given symbol
    ///
    /// Parameters:
    /// * `symbol_id`: The id of the symbol for which to stop receiving OHLCV data
    ///
    /// Errors:
    /// * `ImsClientError`: A generic error type for the  IMS Data client
    ///
    async fn stop_ohlcv_data(&self, symbol_id: String) -> Result<(), ImsClientError>;
    /// Stop all data streams
    ///
    /// Errors:
    /// * `ImsClientError`: A generic error type for the  IMS Data client
    ///
    async fn stop_all_data(&self) -> Result<(), ImsClientError>;
}
