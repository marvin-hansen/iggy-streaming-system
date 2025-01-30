use crate::{ImsClientError, ImsDataClient};
use sbe_messages_client::ClientLoginMessage;

impl ImsDataClient {
    /// Logs in the client via control channel.
    ///
    /// # Errors
    ///
    /// If the message fails to send, it will return an `ImsDataClientError` with the error message.
    ///
    pub(crate) async fn client_login(&self) -> Result<(), ImsClientError> {
        self.dbg_print("login");

        self.dbg_print("Construct login message");
        let login_message = ClientLoginMessage::new(self.client_id);

        self.dbg_print("Encode SBE message");
        let (_, message) = match login_message.encode() {
            Ok(message) => message,
            Err(err) => {
                return Err(ImsClientError::FailedToEncodeControlMessage(format!(
                    "[ImsDataClient/login]: Failed to encode login message: {err}"
                )))
            }
        };

        self.dbg_print("Send login message");
        match self.send_one_message(message).await {
            Ok(_) => {}
            Err(err) => return Err(ImsClientError::FailedToSendControlMessageToIggyServer(
                format!(
                    "[ImsDataClient/login]: Failed to send login message to control channel: {err}"
                ),
            )),
        };

        Ok(())
    }
}
