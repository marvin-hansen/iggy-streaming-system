use binance_core_data_integration::*;
use common_data_bar::TimeResolution;
use common_errors::MessageProcessingError;
use std::collections::HashSet;
use std::future::Future;
use std::sync::Arc;
use trait_data_integration::*;
use data_integration_macro::ImsDataIntegrationImpl;

// All integration traits implementations are generated using the `ImsDataIntegrationImpl` macro
#[derive(Default, ImsDataIntegrationImpl)]
pub struct ImsBinanceUsdFuturesDataIntegration {
    integration: ImsBinanceDataIntegration,
}

impl ImsBinanceUsdFuturesDataIntegration {
    pub fn new() -> Self {
        Self {
            integration: ImsBinanceDataIntegration::new(USD_M_API_BASE_URL, USD_M_API_WSS_URL),
        }
    }

    pub fn testnet() -> Self {
        Self {
            integration: ImsBinanceDataIntegration::new(
                USD_M_TESTNET_API_BASE_URL,
                USD_M_TESTNET_API_WSS_URL,
            ),
        }
    }
}
