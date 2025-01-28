use crate::ImsBinanceDataIntegration;
use trait_data_integration::{
    ImsDataIntegrationError, ImsOhlcvDataIntegration, ImsShutdownIntegration,
    ImsTradeDataIntegration,
};

impl ImsShutdownIntegration for ImsBinanceDataIntegration {
    ///
    /// Shutdown the integration service by stopping all active OHLCV and trade data streams.
    ///
    /// This method is idempotent and can be called multiple times without causing any issues.
    ///
    /// # Process Flow
    /// 1. Stops all active OHLCV data streams
    /// 2. Stops all active trade data streams
    ///
    /// # Returns
    /// - `Ok(())`: If all streams are stopped successfully
    /// - `Err(ImsDataIntegrationError)`: If an error occurs during shutdown
    ///
    async fn shutdown(&self) -> Result<(), ImsDataIntegrationError> {
        let ohlcv_handlers = self.ohlcv_handlers.read().await;
        let trade_handlers = self.trade_handlers.read().await;

        // If nothing has been started, then nothing to stop
        if ohlcv_handlers.is_empty() && trade_handlers.is_empty() {
            return Ok(());
        }

        if !ohlcv_handlers.is_empty() {
            self.stop_all_ohlcv_data()
                .await
                .expect("Failed to start OHLCV data");
        }

        if !trade_handlers.is_empty() {
            self.stop_all_trade_data()
                .await
                .expect("Failed to stop all trade data");
        }

        drop(ohlcv_handlers);
        drop(trade_handlers);

        Ok(())
    }
}
