use crate::{ImsClientError, ImsDataClient};
use bytes::Bytes;
use iggy::messages::send_messages::Message;
use sdk::builder::EventProducer;

impl ImsDataClient {
    /// Sends a message to the control topic.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The message to send as a `Vec<u8>`.
    ///
    /// # Errors
    ///
    /// If the message fails to send, it will return an `ImsDataClientError` with the error message.
    ///
    pub(crate) async fn send_one_message(&self, data: &[u8]) -> Result<(), ImsClientError> {
        // Build iggy Message:
        let message = Message::new(None, Bytes::copy_from_slice(data), None);

        // Send message via iggy
        match self.control_producer.send_one_event(message).await {
            Ok(_) => Ok(()),
            Err(e) => Err(ImsClientError::FailedToSendControlMessageToIggyServer(
                e.to_string(),
            )),
        }
    }
}
