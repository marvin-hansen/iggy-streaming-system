use crate::{ImsClientError, ImsDataClient};
use bytes::Bytes;
use iggy::messages::send_messages::Message;

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
    pub(crate) async fn send_control_message(&self, data: &[u8]) -> Result<(), ImsClientError> {
        self.dbg_print("Build iggy Message");
        let message = Message::new(None, Bytes::copy_from_slice(data), None);

        self.dbg_print("Send message via iggy");
        match self.control_producer.send_one(message).await {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Failed to send control message: {e}");
                Err(ImsClientError::FailedToSendControlMessageToIggyServer(
                    e.to_string(),
                ))
            }
        }
    }
}
