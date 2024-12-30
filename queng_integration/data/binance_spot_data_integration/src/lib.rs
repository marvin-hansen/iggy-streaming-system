use binance_core_data_integration::ImsBinanceDataIntegration;
use binance_data_integration_macro::{
    BinanceImsDataIntegration, BinanceImsOhlcvDataIntegration, BinanceImsTradeDataIntegration,
};
use common_data_bar::TimeResolution;
use common_errors::MessageProcessingError;
use std::collections::HashSet;
use std::sync::Arc;
use trait_data_integration::{
    EventProcessor, ImsDataIntegration, ImsOhlcvDataIntegration, ImsTradeDataIntegration,
};

// LIVE API
const API_BASE_URL: &str = "https://api.binance.com/api/v3";
const API_WSS_URL: &str = "wss://stream.binance.com:9443/ws";

// TESTNET API
// https://www.binance.com/en/support/faq/how-to-test-my-functions-on-binance-testnet-ab78f9a1b8824cf0a106b4229c76496d
const TESTNET_API_BASE_URL: &str = "https://testnet.binance.vision/api/v3";
const TESTNET_API_WSS_URL: &str = "wss://testnet.binance.vision/ws";

// All integration traits implementations are generated using the `binance_data_integration_macro` macro
// See the `binance_data_integration_macro` documentation for more details.
#[derive(
    BinanceImsDataIntegration,
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
            integration: ImsBinanceDataIntegration::new(API_BASE_URL, API_WSS_URL),
        }
    }

    pub fn testnet() -> Self {
        Self {
            integration: ImsBinanceDataIntegration::new(TESTNET_API_BASE_URL, TESTNET_API_WSS_URL),
        }
    }
}
