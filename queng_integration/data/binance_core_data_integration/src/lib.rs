mod ims_integration;
mod ohlcv_data_integration;
mod trade_data_integration;
mod utils;
mod utils_connect;

use reqwest::Client;
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio::time::Instant;

/// Duration between symbol cache refreshes.
///
/// This value determines how often the symbol cache is refreshed from the Binance API.
/// The cache is used to validate symbols and map them to their corresponding OHLCV and trade
/// data streams.
pub(crate) const SYMBOL_CACHE_DURATION: Duration = Duration::from_secs(7200); // 120 minutes

/// Maximum time to wait between reconnect attempts.
///
/// When the WebSocket connection is lost, the integration will attempt to reconnect to the Binance
/// API. If the reconnection fails, it will wait for the specified duration before trying again.
pub(crate) const RECONNECT_INTERVAL: Duration = Duration::from_secs(12 * 3600); // 12 hours

/// Maximum number of reconnect attempts.
///
/// When the WebSocket connection is lost and the reconnection fails, the integration will attempt to
/// reconnect up to the specified number of times. If the maximum number of attempts is reached, the
/// integration will stop.
pub(crate) const MAX_RECONNECT_ATTEMPTS: u32 = 5;

/// Time to wait between reconnect attempts.
///
/// When the WebSocket connection is lost, the integration will wait for the specified duration
/// before attempting to reconnect.
pub(crate) const RECONNECT_DELAY: Duration = Duration::from_secs(5);

/// A Binance data integration implementation that provides real-time trade and OHLCV data streams.
///
/// This struct implements the `ImsDataIntegration` trait for the Binance cryptocurrency exchange.
/// It manages WebSocket connections for trade and OHLCV data streams, handles symbol validation,
/// and provides efficient caching of exchange information.
///
/// # Features
/// - Real-time trade data streaming via WebSocket
/// - Real-time OHLCV (candlestick) data streaming
/// - Symbol validation with caching
/// - Thread-safe connection management
/// - Automatic cleanup of terminated connections
///
#[derive(Default)]
pub struct ImsBinanceDataIntegration {
    api_base_url: String,
    api_wss_url: String,
    http_client: Client,
    symbols_active_trade: RwLock<Vec<String>>,
    symbols_active_ohlcv: RwLock<Vec<String>>,
    symbol_cache: RwLock<Option<(HashSet<String>, Instant)>>,
    trade_handlers: RwLock<HashMap<String, JoinHandle<()>>>,
    ohlcv_handlers: RwLock<HashMap<String, JoinHandle<()>>>,
}

impl ImsBinanceDataIntegration {
    pub fn new(api_base_url: &str, api_wss_url: &str) -> Self {
        Self {
            api_base_url: api_base_url.to_string(),
            api_wss_url: api_wss_url.to_string(),
            http_client: Client::new(),
            symbols_active_trade: RwLock::new(Vec::with_capacity(50)),
            symbols_active_ohlcv: RwLock::new(Vec::with_capacity(50)),
            symbol_cache: RwLock::new(None),
            trade_handlers: RwLock::new(HashMap::with_capacity(50)),
            ohlcv_handlers: RwLock::new(HashMap::with_capacity(50)),
        }
    }
}
