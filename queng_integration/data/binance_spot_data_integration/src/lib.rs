use binance_core_data_integration::{ImsBinanceDataIntegration, SPOT_API_BASE_URL, SPOT_API_WSS_URL, SPOT_TESTNET_API_BASE_URL, SPOT_TESTNET_API_WSS_URL};
use binance_data_integration_macro::{
    BinanceImsSymbolIntegration, BinanceImsOhlcvDataIntegration, BinanceImsTradeDataIntegration,
};
use common_data_bar::TimeResolution;
use common_errors::MessageProcessingError;
use std::collections::HashSet;
use std::sync::Arc;
use trait_data_integration::{
    EventProcessor, ImsSymbolIntegration, ImsOhlcvDataIntegration, ImsTradeDataIntegration,
};

// All integration traits implementations are generated using the `binance_data_integration_macro` macro
// See the `binance_data_integration_macro` documentation for more details.
#[derive(
    BinanceImsSymbolIntegration,
    BinanceImsTradeDataIntegration,
    BinanceImsOhlcvDataIntegration,
    Default,
)]
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
            integration: ImsBinanceDataIntegration::new(SPOT_TESTNET_API_BASE_URL, SPOT_TESTNET_API_WSS_URL),
        }
    }
}
