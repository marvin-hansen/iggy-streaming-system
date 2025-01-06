use binance_core_data_integration::{
    ImsBinanceDataIntegration, COIN_M_API_BASE_URL, COIN_M_API_WSS_URL,
    COIN_M_TESTNET_API_BASE_URL, COIN_M_TESTNET_API_WSS_URL,
};
use binance_data_integration_macro::{
    BinanceImsOhlcvDataIntegration, BinanceImsSymbolIntegration, BinanceImsTradeDataIntegration,
};
use common_data_bar::TimeResolution;
use common_errors::MessageProcessingError;
use std::collections::HashSet;
use std::sync::Arc;
use trait_data_integration::{
    EventProcessor, ImsOhlcvDataIntegration, ImsSymbolIntegration, ImsTradeDataIntegration,
};

// All integration traits implementations are generated using the `binance_data_integration_macro` macro
// See the `binance_data_integration_macro` documentation for more details.
#[derive(
    BinanceImsSymbolIntegration,
    BinanceImsTradeDataIntegration,
    BinanceImsOhlcvDataIntegration,
    Default,
)]
pub struct ImsBinanceCoinFuturesDataIntegration {
    integration: ImsBinanceDataIntegration,
}

impl ImsBinanceCoinFuturesDataIntegration {
    pub fn new() -> Self {
        Self {
            integration: ImsBinanceDataIntegration::new(COIN_M_API_BASE_URL, COIN_M_API_WSS_URL),
        }
    }

    pub fn testnet() -> Self {
        Self {
            integration: ImsBinanceDataIntegration::new(
                COIN_M_TESTNET_API_BASE_URL,
                COIN_M_TESTNET_API_WSS_URL,
            ),
        }
    }
}
