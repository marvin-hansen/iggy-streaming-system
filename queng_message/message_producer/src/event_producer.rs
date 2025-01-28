use crate::MessageProducer;
use bytes::Bytes;
use iggy::messages::send_messages::Message;
use trait_event_processor::{EventProcessor, EventProcessorError};

impl EventProcessor for MessageProducer {
    /// Send a single byte message.
    ///
    /// The message is provided as a `Vec<u8>`.
    ///
    /// # Errors
    ///
    /// Returns an error if the message cannot be sent.
    ///
    async fn process_one_event(&self, bytes: Vec<u8>) -> Result<(), EventProcessorError> {
        self.dbg_print("process_one_event");

        self.dbg_print("converting SBE message to iggy message");
        let message = Message::new(None, Bytes::from(bytes), None);

        self.dbg_print("sending iggy message");
        match self.producer.send_one(message).await {
            Ok(()) => Ok(()),
            Err(e) => Err(EventProcessorError::new(e.to_string())),
        }
    }

    /// Send a collection of byte messages.
    ///
    /// The messages are provided as a `Vec` of `Vec<u8>`.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the messages cannot be sent.
    ///
    async fn process_event_batch(
        &self,
        bytes_batch: &[Vec<u8>],
    ) -> Result<(), EventProcessorError> {
        self.dbg_print("process_event_batch");

        self.dbg_print("converting SBE message batch to iggy messages");
        let messages: Vec<Message> = bytes_batch
            .iter()
            // Convert the SBE bytes into a new message with auto-generated ID, payload, and no headers.
            .map(|bytes| Message::new(None, Bytes::from(bytes.to_owned()), None))
            .collect();

        self.dbg_print("sending iggy message batch");
        match self.producer.send(messages).await {
            Ok(()) => Ok(()),
            Err(e) => Err(EventProcessorError::new(e.to_string())),
        }
    }
}
