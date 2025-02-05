use crate::MessageClientBuilder;
use message_consumer::MessageConsumer;
use message_producer::MessageProducer;

impl MessageClientBuilder {
    /// Returns a reference to the `MessageProducer` created for this client.
    pub fn iggy_producer(&self) -> &MessageProducer {
        &self.iggy_producer
    }
    /// Returns the `MessageConsumer` created for this client.
    ///
    /// Note that this method consumes `self`.
    pub fn iggy_consumer(self) -> MessageConsumer {
        self.iggy_consumer
    }
}
