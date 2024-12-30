use common_data_bar::TimeResolution;
use common_errors::MessageProcessingError;
use std::sync::Arc;

#[allow(dead_code)] // Clippy can't see that the trait is used
#[trait_variant::make(ImsOhlcvDataIntegration: Send)]
pub trait LocalImsOhlcvDataIntegration {
    /// Start fetching OHLCV data from the exchange.
    ///
    /// This method is used to start fetching OHLCV data from an exchange for the given symbols.
    ///
    /// The method takes a `&[String]` of symbols to fetch data for and an `EventProcessor` that
    /// will be called with the data.
    ///
    /// The `EventProcessor` is `Send` and `Sync` so that it can be safely accessed
    /// from multiple threads.
    ///
    /// The `EventProcessor` is also `Send` so that it can be safely moved to a different thread.
    ///
    /// The `EventProcessor` is also `Sync` so that it can be safely accessed
    /// from multiple threads.
    ///
    /// The method returns a `Result` of `()`. If the method is successful, the
    /// `Result` is `Ok`, otherwise it is `Err` with a `MessageProcessingError`.
    ///
    async fn start_ohlcv_data<P>(
        &self,
        symbols: &[String],
        time_resolution: TimeResolution,
        processor: Arc<P>,
    ) -> Result<(), MessageProcessingError>
    where
        P: crate::EventProcessor + Send + Sync + 'static;

    /// Stop fetching OHLCV data from a list of symbols.
    ///
    /// This method is used to stop fetching OHLCV data from an exchange for a given list of symbols.
    ///
    /// The method takes a `&[String]` of symbols to stop fetching data for.
    ///
    /// The method returns a `Result` of `()`. If the method is successful, the
    /// `Result` is `Ok`, otherwise it is `Err` with a `MessageProcessingError`.
    ///
    async fn stop_ohlcv_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError>;

    /// Stop fetching OHLCV data from all symbols.
    ///
    /// This method is used to stop fetching OHLCV data from an exchange for all symbols.
    ///
    /// The method returns a `Result` of `()`. If the method is successful, the
    /// `Result` is `Ok`, otherwise it is `Err` with a `MessageProcessingError`.
    ///
    async fn stop_all_ohlcv_data(&self) -> Result<(), MessageProcessingError>;
}
