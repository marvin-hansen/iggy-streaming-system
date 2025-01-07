use binance_core_data_integration::*;
use common_data_bar::TimeResolution;
use common_errors::MessageProcessingError;
use data_integration_macro::ImsDataIntegrationImpl;
use std::collections::HashSet;
use std::future::Future;
use std::sync::Arc;
use trait_data_integration::*;

// All integration traits implementations are generated using the `ImsDataIntegrationImpl` macro
#[derive(Default, ImsDataIntegrationImpl)]
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
