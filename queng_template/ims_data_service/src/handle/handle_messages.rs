use crate::service::Service;
use common_errors::MessageProcessingError;
use iggy::models::messages::PolledMessage;
use sbe_messages_client::{ClientLoginMessage, ClientLogoutMessage};
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
        //
        let message = polled_message.payload.to_vec();
        let raw_message = message.as_slice();
        let message_type = MessageType::from(u16::from(raw_message[2]));

        match message_type {
            MessageType::ClientLogin => {
                let client_login_msg = ClientLoginMessage::from(raw_message);
                self.handle_client_login(&client_login_msg).await
            }
            MessageType::ClientLogout => {
                let client_logout_msg = ClientLogoutMessage::from(raw_message);
                self.handle_client_logout(&client_logout_msg).await
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
