use async_trait::async_trait;
use iggy::error::IggyError;
use iggy::messages::send_messages::Message;

/// Event producer interface
#[async_trait]
pub trait EventProducer {
    /// Send a single iggy message.
    ///
    /// The message is provided as an iggy `Message`.
    ///
    /// # Errors
    ///
    /// Returns an error if the message cannot be sent.
    ///
    async fn send_one_event(&self, message: Message) -> Result<(), IggyError>;

    /// Send a batch of iggy messages.
    ///
    /// The messages are provided as a `Vec` of `Message`.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the messages cannot be sent.
    async fn send_event_batch(&self, messages: Vec<Message>) -> Result<(), IggyError>;
}

// Default implementation for `&T`
// https://users.rust-lang.org/t/hashmap-get-dereferenced/33558
#[async_trait]
impl<T: EventProducer + Send + Sync> EventProducer for &T {
    async fn send_one_event(&self, message: Message) -> Result<(), IggyError> {
        (**self).send_one_event(message).await
    }

    async fn send_event_batch(&self, messages: Vec<Message>) -> Result<(), IggyError> {
        (**self).send_event_batch(messages).await
    }
}
