use chrono::{TimeZone, Utc};
use common_data_bar::{OHLCVBar, TradeBar};
use rust_decimal::Decimal;

/// Extracts an OHLCVBar from a Binance WebSocket kline JSON message
///
/// # Arguments
/// * `json_text` - The JSON message string from the Binance WebSocket
/// * `symbol` - The symbol of the OHLCVBar
///
/// # Returns
/// An `Option<OHLCVBar>` containing the parsed bar if successful, or `None` if parsing fails
///
pub(crate) async fn extract_ohlcv_bar_from_json(json_text: &str, symbol: &str) -> Option<OHLCVBar> {
    // JSON Format
    // {
    //   "e": "kline",         // Event type
    //   "E": 1672515782136,   // Event time
    //   "s": "BNBBTC",        // Symbol
    //   "k": {
    //     "t": 1672515780000, // Kline start time
    //     "T": 1672515839999, // Kline close time
    //     "s": "BNBBTC",      // Symbol
    //     "i": "1m",          // Interval
    //     "f": 100,           // First trade ID
    //     "L": 200,           // Last trade ID
    //     "o": "0.0010",      // Open price
    //     "c": "0.0020",      // Close price
    //     "h": "0.0025",      // High price
    //     "l": "0.0015",      // Low price
    //     "v": "1000",        // Base asset volume
    //     "n": 100,           // Number of trades
    //     "x": false,         // Is this kline closed?
    //     "q": "1.0000",      // Quote asset volume
    //     "V": "500",         // Taker buy base asset volume
    //     "Q": "0.500",       // Taker buy quote asset volume
    //     "B": "123456"       // Ignore
    //   }

    // Parse the JSON message
    let json = match serde_json::from_str::<serde_json::Value>(json_text) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            return None;
        }
    };

    // Extract kline data
    let kline = json.get("k")?;

    // Check if the kline is closed
    let is_closed = kline.get("x").and_then(|x| x.as_bool()).unwrap_or(false);

    // Only process closed klines
    if !is_closed {
        return None;
    }

    // Extract numeric values
    let open = parse_decimal(kline.get("o"));
    let high = parse_decimal(kline.get("h"));
    let low = parse_decimal(kline.get("l"));
    let close = parse_decimal(kline.get("c"));
    let volume = parse_decimal(kline.get("v"));

    // Extract start time of the kline
    let start_time = kline
        .get("t")
        .and_then(|t| t.as_u64())
        .map(|ms| Utc.timestamp_millis_opt(ms as i64).unwrap())
        .unwrap_or_else(Utc::now);

    // Generate a unique symbol ID
    let symbol_id = symbol.to_string();

    // Create and return the OHLCVBar
    Some(OHLCVBar::new(
        symbol_id, start_time, open, high, low, close, volume,
    ))
}
/// Extracts a TradeBar from a Binance WebSocket trade JSON message
///
/// # Arguments
/// * `json_text` - The JSON message string from the Binance WebSocket
/// * `symbol` - The symbol of the TradeBar
///
/// # Returns
/// An `Option<TradeBar>` containing the parsed bar if successful, or `None` if parsing fails
///
pub(crate) async fn extract_trade_bar_from_json(json_text: &str, symbol: &str) -> Option<TradeBar> {
    // JSON format
    // {
    //   "e": "trade",       // Event type
    //   "E": 1672515782136, // Event time
    //   "s": "BNBBTC",      // Symbol
    //   "t": 12345,         // Trade ID
    //   "p": "0.001",       // Price
    //   "q": "100",         // Quantity
    //   "T": 1672515782136, // Trade time
    //   "m": true,          // Is the buyer the market maker?
    //   "M": true           // Ignore
    // }

    // Parse the JSON message
    let json = match serde_json::from_str::<serde_json::Value>(json_text) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Failed to parse trade JSON: {}", e);
            return None;
        }
    };

    // Extract trade details
    let price = parse_decimal(json.get("p"));
    let volume = parse_decimal(json.get("q"));

    // Extract trade time
    let trade_time = json
        .get("T")
        .and_then(|t| t.as_u64())
        .map(|ms| Utc.timestamp_millis_opt(ms as i64).unwrap())
        .unwrap_or_else(Utc::now);

    // Generate a unique trade bar ID
    let symbol_id = symbol.to_string();

    // Create and return the TradeBar
    Some(TradeBar::new(symbol_id, trade_time, price, volume))
}

/// Safely converts a JSON value to a Decimal, with a fallback to zero.
///
/// # Arguments
/// * `value` - The JSON value to parse
///
/// # Returns
/// The parsed Decimal, or zero if parsing fails
fn parse_decimal(value: Option<&serde_json::Value>) -> Decimal {
    value
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<Decimal>().ok())
        .unwrap_or_default()
}
