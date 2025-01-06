use common_errors::MessageProcessingError;
use std::collections::HashSet;

#[allow(dead_code)] // Clippy can't see that the trait is used
#[trait_variant::make(ImsSymbolIntegration: Send)]
pub trait LocalImsSymbolIntegration {
    /// Retrieves and caches the list of valid trading symbols from Binance.
    ///
    /// # Returns
    /// - `Ok(HashSet<String>)`: Set of valid trading symbols
    /// - `Err(MessageProcessingError)`: If API call fails or response is invalid
    ///
    async fn get_exchange_symbols(&self) -> Result<HashSet<String>, MessageProcessingError>;

    /// Validate a list of symbols.
    ///
    /// This method is used to check if all symbols in the given list are valid
    /// for the exchange.
    ///
    /// The method takes a `&[String]` of symbols to validate and returns a
    /// `Result` of `bool`. The `Result` is `Ok` if all symbols are valid,
    /// otherwise it is `Err` with an `MessageProcessingError`.
    async fn validate_symbols(&self, symbols: &[String]) -> Result<bool, MessageProcessingError>;
}
