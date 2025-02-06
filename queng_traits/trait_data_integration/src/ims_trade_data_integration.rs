use crate::ImsDataIntegrationError;
use sdk::builder::EventProducer;
use std::sync::Arc;

#[allow(dead_code)] // Clippy can't see that the trait is used
#[trait_variant::make(ImsTradeDataIntegration: Send)]
pub trait LocalImsTradeDataIntegration {
    /// Start fetching trade data from the exchange.
    ///
    /// This method is used to start fetching trade data from an exchange for the given symbols.
    ///
    /// The method takes a `&[String]` of symbols to fetch data for and an `EventProducer` that
    /// will be called with the data.
    ///
    /// The `EventProducer` is `Send` and `Sync` so that it can be safely accessed
    /// from multiple threads.
    ///
    /// The `EventProducer` is also `Send` so that it can be safely moved to a different thread.
    ///
    /// The `EventProducer` is also `Sync` so that it can be safely accessed
    /// from multiple threads.
    ///
    /// The method returns a `Result` of `()`. If the method is successful, the
    /// `Result` is `Ok`, otherwise it is `Err` with a `ImsDataIntegrationError`.
    ///
    async fn start_trade_data<P>(
        &self,
        symbols: &[String],
        processor: &Arc<P>,
    ) -> Result<(), ImsDataIntegrationError>
    where
        P: EventProducer + Send + Sync + 'static;

    /// Stop fetching trade data from the specified symbols.
    ///
    /// This method is used to stop fetching trade data from an exchange for the given symbols.
    ///
    /// The method takes a `&[String]` of symbols to stop fetching data for.
    ///
    /// The method returns a `Result` of `()`. If the method is successful, the
    /// `Result` is `Ok`, otherwise it is `Err` with a `ImsDataIntegrationError`.
    ///
    async fn stop_trade_data(&self, symbols: &[String]) -> Result<(), ImsDataIntegrationError>;

    /// Stop fetching trade data from all symbols.
    ///
    /// This method is used to stop fetching trade data from an exchange for all symbols.
    ///
    /// The method returns a `Result` of `()`. If the method is successful, the
    /// `Result` is `Ok`, otherwise it is `Err` with a `ImsDataIntegrationError`.
    ///
    async fn stop_all_trade_data(&self) -> Result<(), ImsDataIntegrationError>;
}
