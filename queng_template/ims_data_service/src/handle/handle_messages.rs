use crate::service::Service;
use common_errors::MessageProcessingError;
use iggy::models::messages::PolledMessage;
use sbe_messages_control::{StartDataMessage, StopAllDataMessage, StopDataMessage};
use sbe_types::MessageType;
use trait_data_integration::ImsDataIntegration;

impl<Integration: ImsDataIntegration + 'static> Service<Integration> {
    /// Handles a single message by processing it and sending it to the appropriate
    /// manager for further processing.
    ///
    /// This method takes a message payload, processes it by calling the `process_message`
    /// method, and sends it to the appropriate handler for further processing.
    ///
    /// # Parameters
    ///
    /// * `self` - The Server instance
    /// * `message` - The message payload to be processed
    ///
    /// # Returns
    /// * Ok on success,
    /// * Err on any processing error
    ///
    pub(crate) async fn handle_message(
        &self,
        polled_message: PolledMessage,
    ) -> Result<(), MessageProcessingError> {
        // payload is a sliceable chunk of contiguous memory; thus we can direct access its contents.
        let raw_message = polled_message.payload.as_ref();
        // The payload is encoded as fixed sized SBE with a fixed header structure.
        // At position 2 is the integer encoded message type, which then can be cast directly into the MessageType enum.
        let message_type = MessageType::from(u16::from(raw_message[2]));

        match message_type {
            MessageType::ClientLogin => {
                // We know that ClientLogin message is 12 bytes long (see ClientLoginMessage SBE definition)
                // And we also know that the client ID is at position 11. Therefore, we can extract
                // the client ID directly from the message without deserializing it.
                let client_id = raw_message[11] as u16;
                // Alternatively, we could deserialize the message into a ClientLoginMessage struct
                // let client_login_msg = ClientLoginMessage::from(raw_message);
                self.handle_client_login(client_id).await
            }
            MessageType::ClientLogout => {
                // We know that ClientLogoutMessage message is also 12 bytes long (see ClientLogoutMessage SBE definition)
                // And we also know that the client ID is at position 11. Therefore, we can extract
                // the client ID directly from the message without deserializing it.
                let client_id = raw_message[11] as u16;
                // Alternatively, we could deserialize the message into a ClientLogoutMessage struct
                // let client_logout_msg = ClientLogoutMessage::from(raw_message);
                self.handle_client_logout(client_id).await
            }
            MessageType::StartData => {
                let start_data_msg = StartDataMessage::from(raw_message);
                self.handle_start_data(&start_data_msg).await
            }

            MessageType::StopData => {
                let stop_data_msg = StopDataMessage::from(raw_message);
                self.handle_stop_data(&stop_data_msg).await
            }

            MessageType::StopAllData => {
                let stop_all_data_msg = StopAllDataMessage::from(raw_message);
                self.handle_stop_all_data(&stop_all_data_msg).await
            }

            _ => Err(MessageProcessingError(
                "[handle::handle_message]: Unknown message type. Abort processing".to_string(),
            )),
        }
    }
}
