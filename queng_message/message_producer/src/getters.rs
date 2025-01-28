use crate::MessageProducer;
use iggy::clients::producer::IggyProducer;
use iggy::identifier::Identifier;

impl MessageProducer {
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

    /// Returns a reference to the `IggyProducer`.
    #[inline]
    pub fn producer(&self) -> &IggyProducer {
        &self.producer
    }
}
