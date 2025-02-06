use crate::builder::event_consumer::EventConsumerError;
use async_trait::async_trait;

/// Trait for event consumer
#[async_trait]
pub trait EventConsumer {
    /// Consume a event from the message bus.
    ///
    /// # Arguments
    ///
    /// * `data` - The event data
    ///
    /// # Errors
    ///
    /// * `EventConsumerError` - If the event consumer fails to consume the event
    async fn consume(&self, data: Vec<u8>) -> Result<(), EventConsumerError>;
}

// Default implementation for `&T`
// https://users.rust-lang.org/t/hashmap-get-dereferenced/33558
#[async_trait]
impl<T: EventConsumer + Send + Sync> EventConsumer for &T {
    /// Consume a event from the message bus.
    ///
    /// # Arguments
    ///
    /// * `data` - The event data
    ///
    /// # Errors
    ///
    /// * `EventConsumerError` - If the event consumer fails to consume the event
    async fn consume(&self, data: Vec<u8>) -> Result<(), EventConsumerError> {
        (**self).consume(data).await
    }
}
