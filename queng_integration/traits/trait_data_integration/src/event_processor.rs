use common_errors::MessageProcessingError;

/// Trait to define an `EventProcessor` that can be used to process events
/// in a local context.
///
/// The `process` method is a callback that is called with the data fetched from
/// the exchange. The method takes a `&[Vec<u8>]` of data as input and returns a
/// `Result` of `()`.
#[allow(dead_code)] // Clippy can't see that the trait is used
#[trait_variant::make(EventProcessor: Send)]
pub trait LocalEventProcessor {
    async fn process(&self, data: &[Vec<u8>]) -> Result<(), MessageProcessingError>;
}
