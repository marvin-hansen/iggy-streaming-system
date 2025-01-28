use crate::EventProcessorError;

/// Trait to define an `EventProcessor` that can be used to process events
/// in a local context.
///
/// The `process` method is a callback that is called with the data fetched from
/// the exchange. The method takes a `&[Vec<u8>]` of data as input and returns a
/// `Result` of `()`.
#[allow(dead_code)] // Clippy can't see that the trait is used
#[trait_variant::make(EventProcessor: Send)]
pub trait LocalEventProcessor {
    /// Send a single byte message.
    ///
    /// The message is provided as a `Vec<u8>`.
    ///
    /// # Errors
    ///
    /// Returns an error if the message cannot be sent.
    async fn process_one_event(&self, bytes: Vec<u8>) -> Result<(), EventProcessorError>;
    /// Send a batch of byte messages.
    ///
    /// The messages are provided as a `Vec` of `Vec<u8>`.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the messages cannot be sent.
    async fn process_event_batch(&self, bytes_batch: &[Vec<u8>])
        -> Result<(), EventProcessorError>;
}

// Default implementation for `&T`
// https://users.rust-lang.org/t/hashmap-get-dereferenced/33558
impl<T: EventProcessor + Send + Sync> EventProcessor for &T {
    async fn process_one_event(&self, bytes: Vec<u8>) -> Result<(), EventProcessorError> {
        (**self).process_one_event(bytes).await
    }

    async fn process_event_batch(
        &self,
        bytes_batch: &[Vec<u8>],
    ) -> Result<(), EventProcessorError> {
        (**self).process_event_batch(bytes_batch).await
    }
}
