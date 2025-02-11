use crate::{ImsClientError, ImsDataClient};
use sbe_messages_client::ClientLogoutMessage;

impl ImsDataClient {
    /// Logs out the client via control channel.
    ///
    /// # Errors
    ///
    /// If the message fails to send, it will return an `ImsDataClientError` with the error message.
    ///
    pub(crate) async fn client_logout(&self) -> Result<(), ImsClientError> {
        self.dbg_print("logout");

        self.dbg_print("Construct logout message");
        let logout_message = ClientLogoutMessage::new(self.client_id);

        self.dbg_print("Encode SBE message");
        let (_, message) = match logout_message.encode() {
            Ok(message) => message,
            Err(err) => {
                return Err(ImsClientError::FailedToEncodeControlMessage(format!(
                    "[ImsDataClient/logout]: Failed to encode logout_message message: {err}"
                )))
            }
        };

        self.dbg_print("Send logout message");
        match self.send_control_message(message.as_slice()).await {
            Ok(_) => {}
            Err(err) => return Err(ImsClientError::FailedToSendControlMessageToIggyServer(
                format!(
                    "[ImsDataClient/logout]: Failed to send logout message to control channel: {err}"
                ),
            )),
        };
        Ok(())
    }
}
