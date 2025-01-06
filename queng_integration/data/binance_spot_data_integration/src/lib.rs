use binance_core_data_integration::*;
use common_data_bar::TimeResolution;
use common_errors::MessageProcessingError;
use std::collections::HashSet;
use std::future::Future;
use std::sync::Arc;
use trait_data_integration::*;

#[derive(Default)]
pub struct ImsBinanceSpotDataIntegration {
    integration: ImsBinanceDataIntegration,
}

impl ImsBinanceSpotDataIntegration {
    pub fn new() -> Self {
        Self {
            integration: ImsBinanceDataIntegration::new(SPOT_API_BASE_URL, SPOT_API_WSS_URL),
        }
    }

    pub fn testnet() -> Self {
        Self {
            integration: ImsBinanceDataIntegration::new(
                SPOT_TESTNET_API_BASE_URL,
                SPOT_TESTNET_API_WSS_URL,
            ),
        }
    }
}

impl ImsDataIntegration for ImsBinanceSpotDataIntegration {}

impl ImsSymbolIntegration for ImsBinanceSpotDataIntegration {
    fn get_exchange_symbols(
        &self,
    ) -> impl Future<Output = Result<HashSet<String>, MessageProcessingError>> + Send {
        self.integration.get_exchange_symbols()
    }

    fn validate_symbols(
        &self,
        symbols: &[String],
    ) -> impl Future<Output = Result<bool, MessageProcessingError>> + Send {
        self.integration.validate_symbols(symbols)
    }
}

impl ImsTradeDataIntegration for ImsBinanceSpotDataIntegration {
    fn start_trade_data<P>(
        &self,
        symbols: &[String],
        processor: Arc<P>,
    ) -> impl Future<Output = Result<(), MessageProcessingError>> + Send
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        self.integration.start_trade_data(symbols, processor)
    }

    fn stop_trade_data(
        &self,
        symbols: &[String],
    ) -> impl Future<Output = Result<(), MessageProcessingError>> + Send {
        self.integration.stop_trade_data(symbols)
    }

    fn stop_all_trade_data(
        &self,
    ) -> impl Future<Output = Result<(), MessageProcessingError>> + Send {
        self.integration.stop_all_trade_data()
    }
}

impl ImsOhlcvDataIntegration for ImsBinanceSpotDataIntegration {
    fn start_ohlcv_data<P>(
        &self,
        symbols: &[String],
        time_resolution: TimeResolution,
        processor: Arc<P>,
    ) -> impl Future<Output = Result<(), MessageProcessingError>> + Send
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        self.integration
            .start_ohlcv_data(symbols, time_resolution, processor)
    }

    fn stop_ohlcv_data(
        &self,
        symbols: &[String],
    ) -> impl Future<Output = Result<(), MessageProcessingError>> + Send {
        self.integration.stop_ohlcv_data(symbols)
    }

    fn stop_all_ohlcv_data(
        &self,
    ) -> impl Future<Output = Result<(), MessageProcessingError>> + Send {
        self.integration.stop_all_ohlcv_data()
    }
}
