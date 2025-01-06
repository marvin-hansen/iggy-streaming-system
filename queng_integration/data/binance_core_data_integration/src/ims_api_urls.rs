// SPOT
//
// LIVE API
pub const SPOT_API_BASE_URL: &str = "https://api.binance.com/api/v3";
pub const SPOT_API_WSS_URL: &str = "wss://stream.binance.com:9443/ws";

// TESTNET API
// https://www.binance.com/en/support/faq/how-to-test-my-functions-on-binance-testnet-ab78f9a1b8824cf0a106b4229c76496d
pub const SPOT_TESTNET_API_BASE_URL: &str = "https://testnet.binance.vision/api/v3";
pub const SPOT_TESTNET_API_WSS_URL: &str = "wss://testnet.binance.vision/ws";

// Binance Coin-M Futures
//
// LIVE API https://binance-docs.github.io/apidocs/delivery/en/#basis
pub const COIN_M_API_BASE_URL: &str = "https://dapi.binance.com/dapi/v1";
pub const COIN_M_API_WSS_URL: &str = "wss://dstream.binance.com/ws";

// TESTNET API
// https://developers.binance.com/docs/derivatives/coin-margined-futures/general-info
// Somehow the TESTNET API does not deliver symbols. Using the LIVE API instead
pub const COIN_M_TESTNET_API_BASE_URL: &str = "https://dapi.binance.com/dapi/v1"; // "https://testnet.binancefuture.com/api/v3";
pub const COIN_M_TESTNET_API_WSS_URL: &str = "wss://dstream.binancefuture.com";

// Binance USD-M Futures API endpoints
//
// LIVE API
// https://binance-docs.github.io/apidocs/futures/en/#general-info
pub const USD_M_API_BASE_URL: &str = "https://fapi.binance.com/fapi/v1";
pub const USD_M_API_WSS_URL: &str = "wss://fstream.binance.com/ws";

// TESTNET API
// https://developers.binance.com/docs/derivatives/usds-margined-futures/general-info
// Somehow the TESTNET API does not deliver symbols. Using the LIVE API instead
pub const USD_M_TESTNET_API_BASE_URL: &str = "https://fapi.binance.com/fapi/v1"; //"https://testnet.binancefuture.com/api/v3";
pub const USD_M_TESTNET_API_WSS_URL: &str = "wss://stream.binancefuture.com";
