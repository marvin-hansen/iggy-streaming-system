use crate::{Guarded, MessageConsumer};
use iggy::clients::consumer::IggyConsumer;
use iggy::identifier::Identifier;

impl MessageConsumer {
    /// Returns a reference to the stream identifier.
    #[inline]
    pub const fn stream_id(&self) -> &Identifier {
        &self.stream_id
    }

    /// Returns a reference to the topic identifier.
    #[inline]
    pub const fn topic_id(&self) -> &Identifier {
        &self.topic_id
    }

    /// Returns a reference to the underlying consumer.
    #[inline]
    pub const fn consumer(&self) -> &Guarded<IggyConsumer> {
        &self.consumer
    }

    /// Returns a mutable reference to the underlying consumer.
    #[inline]
    pub fn consumer_mut(&mut self) -> &mut Guarded<IggyConsumer> {
        &mut self.consumer
    }
}
