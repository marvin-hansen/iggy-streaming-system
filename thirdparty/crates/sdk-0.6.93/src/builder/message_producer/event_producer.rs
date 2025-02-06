use crate::builder::event_producer::EventProducer;
use crate::builder::message_producer::MessageProducer;
use async_trait::async_trait;
use iggy::error::IggyError;
use iggy::messages::send_messages::Message;
use tracing::error;

#[async_trait]
impl EventProducer for MessageProducer {
    /// Sends a single event to the target topic.
    ///
    /// # Arguments
    ///
    /// * `message`: The event to be sent.
    ///
    /// # Errors
    ///
    /// This function will return an error if the underlying producer fails to send the message.
    ///
    async fn send_one_event(&self, message: Message) -> Result<(), IggyError> {
        match self.producer.send_one(message).await {
            Ok(()) => Ok(()),
            Err(err) => {
                error!("Failed to send one event due to error: {err}");
                Err(err)
            }
        }
    }

    /// Sends a batch of events to the target topic.
    ///
    /// # Arguments
    ///
    /// * `messages`: The events to be sent.
    ///
    /// # Errors
    ///
    /// This function will return an error if the underlying producer fails to send the messages.
    ///
    async fn send_event_batch(&self, messages: Vec<Message>) -> Result<(), IggyError> {
        match self.producer.send(messages).await {
            Ok(()) => Ok(()),
            Err(err) => {
                error!("Failed to send event batch due to error: {err}");
                Err(err)
            }
        }
    }
}
