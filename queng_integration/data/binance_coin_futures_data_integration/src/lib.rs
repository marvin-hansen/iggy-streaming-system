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
// Binance Coin-M Futures API endpoints
// https://binance-docs.github.io/apidocs/delivery/en/#basis
const API_BASE_URL: &str = "https://dapi.binance.com/dapi/v1";
const API_WSS_URL: &str = "wss://dstream.binance.com/ws";

// TESTNET API
// https://developers.binance.com/docs/derivatives/coin-margined-futures/general-info
const TESTNET_API_BASE_URL: &str = "https://testnet.binancefuture.com/api/v3";
const TESTNET_API_WSS_URL: &str = "wss://dstream.binancefuture.com";

// All integration traits implementations are generated using the `binance_data_integration_macro` macro
// See the `binance_data_integration_macro` documentation for more details.
#[derive(
    BinanceImsDataIntegration,
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
            integration: ImsBinanceDataIntegration::new(API_BASE_URL, API_WSS_URL),
        }
    }

    pub fn testnet() -> Self {
        Self {
            integration: ImsBinanceDataIntegration::new(TESTNET_API_BASE_URL, TESTNET_API_WSS_URL),
        }
    }
}
