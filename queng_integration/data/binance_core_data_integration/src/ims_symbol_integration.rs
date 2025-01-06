use crate::{ImsBinanceDataIntegration, SYMBOL_CACHE_DURATION};
use common_errors::MessageProcessingError;
use serde_json::Value;
use std::collections::HashSet;
use tokio::time::Instant;
use trait_data_integration::{ImsSymbolIntegration};

impl ImsSymbolIntegration for ImsBinanceDataIntegration {
    /// Retrieves and caches the list of valid trading symbols from Binance.
    ///
    /// This method:
    /// 1. Checks the cache first, returning cached symbols if they're still valid
    /// 2. If cache is stale or empty, fetches fresh symbols from Binance API
    /// 3. Updates the cache with new symbols and timestamp
    ///
    /// The cache duration is set to 120 minutes (2 hours) to balance API rate limits
    /// with symbol list accuracy.
    ///
    /// # Returns
    /// - `Ok(HashSet<String>)`: Set of valid trading symbols
    /// - `Err(MessageProcessingError)`: If API call fails or response is invalid
    ///
    async fn get_exchange_symbols(&self) -> Result<HashSet<String>, MessageProcessingError> {
        // Check cache first
        if let Some((symbols, timestamp)) = &*self.symbol_cache.read().await {
            if timestamp.elapsed() < SYMBOL_CACHE_DURATION {
                return Ok(symbols.clone());
            }
        }

        // Cache is stale or doesn't exist, fetch from API
        let url = format!("{}/exchangeInfo", self.api_base_url);
        let response =
            self.http_client.get(&url).send().await.map_err(|e| {
                MessageProcessingError::new(format!("Failed to fetch symbols: {}", e))
            })?;

        let data: Value = response
            .json()
            .await
            .map_err(|e| MessageProcessingError::new(format!("Failed to parse response: {}", e)))?;

        let symbols = data["symbols"]
            .as_array()
            .ok_or_else(|| MessageProcessingError::new("Invalid response format".to_string()))?
            .iter()
            .filter_map(|s| s["symbol"].as_str().map(String::from))
            .collect::<HashSet<_>>();

        // Update cache
        *self.symbol_cache.write().await = Some((symbols.clone(), Instant::now()));

        Ok(symbols)
    }

    /// Validates a list of trading symbols against Binance's supported symbols.
    ///
    /// This method:
    /// 1. Retrieves the current list of valid symbols (using cache when possible)
    /// 2. Checks each input symbol against the valid symbol list
    /// 3. Returns an error if any symbols are invalid
    ///
    /// # Arguments
    /// * `symbols` - List of symbols to validate
    ///
    /// # Returns
    /// - `Ok(true)`: If all symbols are valid
    /// - `Err(MessageProcessingError)`: If any symbols are invalid, with error message listing invalid symbols
    ///
    async fn validate_symbols(&self, symbols: &[String]) -> Result<bool, MessageProcessingError> {
        let valid_symbols = self.get_exchange_symbols().await?;

        let invalid_symbols: Vec<_> = symbols
            .iter()
            .filter(|s| !valid_symbols.contains(*s))
            .collect();

        if !invalid_symbols.is_empty() {
            return Err(MessageProcessingError::new(format!(
                "The following symbols are not traded on Binance: {:?}",
                invalid_symbols
            )));
        }

        Ok(true)
    }
}
