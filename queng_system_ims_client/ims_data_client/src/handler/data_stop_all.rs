use crate::{ImsClientError, ImsDataClient};
use sbe_messages_control::StopAllDataMessage;

impl ImsDataClient {
    pub(crate) async fn client_stop_all_data(&self) -> Result<(), ImsClientError> {
        self.dbg_print("stop_all_data");

        self.dbg_print("Construct stop_all_data message");
        let stop_all_data_message = StopAllDataMessage::new(self.client_id, self.exchange_id);

        self.dbg_print("Encode SBE message");
        let (_, message) = match stop_all_data_message.encode() {
            Ok(res) => res,
            Err(err) => {
                return Err(ImsClientError::FailedToEncodeControlMessage(format!(
                    "[ImsDataClient/stop_all]: Failed to encode stop_all_data message: {err}"
                )))
            }
        };

        self.dbg_print("Send stop_all_data message");
        match self.send_one_message(message.as_slice()).await {
            Ok(_) => {}
            Err(err) => {
                return Err(ImsClientError::FailedToSendControlMessageToIggyServer(
                    format!(
                        "[ImsDataClient/stop_all]: Failed to send stop_all_data message: {err}"
                    ),
                ))
            }
        };

        Ok(())
    }
}
