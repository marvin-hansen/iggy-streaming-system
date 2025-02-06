use binance_core_data_integration::*;
use common_data_bar::TimeResolution;
use data_integration_macro::ImsDataIntegrationImpl;
use sdk::builder::EventProducer;
use std::collections::HashSet;
use std::future::Future;
use std::sync::Arc;
use trait_data_integration::*;

// All integration traits implementations are generated using the `ImsDataIntegrationImpl` macro
#[derive(Default, ImsDataIntegrationImpl)]
pub struct ImsBinanceSpotTestnetDataIntegration {
    integration: ImsBinanceDataIntegration,
}

impl ImsBinanceSpotTestnetDataIntegration {
    /// Create new instance of integration for the live environment
    pub fn new() -> Self {
        Self {
            integration: ImsBinanceDataIntegration::new(
                SPOT_TESTNET_API_BASE_URL,
                SPOT_TESTNET_API_WSS_URL,
            ),
        }
    }
}
