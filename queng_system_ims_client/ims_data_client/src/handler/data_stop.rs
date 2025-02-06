use crate::{ImsClientError, ImsDataClient};
use sbe_messages_control::StopDataMessage;
use sbe_types::DataType;

impl ImsDataClient {
    pub(crate) async fn client_stop_data(
        &self,
        symbol_id: String,
        data_type_id: DataType,
    ) -> Result<(), ImsClientError> {
        self.dbg_print("stop_data");

        self.dbg_print("Construct stop_data message");
        let stop_data_message =
            StopDataMessage::new(self.client_id, self.exchange_id, symbol_id, data_type_id);

        self.dbg_print("Encode SBE message");
        let (_, message) = match stop_data_message.encode() {
            Ok(res) => res,
            Err(err) => {
                return Err(ImsClientError::FailedToEncodeControlMessage(format!(
                    "[ImsDataClient/stop_data]: Failed to encode stop_data message: {err}"
                )))
            }
        };

        self.dbg_print("Send stop_data message");
        match self.send_one_message(message.as_slice()).await {
            Ok(_) => {}
            Err(err) => {
                return Err(ImsClientError::FailedToSendControlMessageToIggyServer(
                    format!("[ImsDataClient/stop_data]: Failed to send stop_data message: {err}"),
                ))
            }
        };
        Ok(())
    }
}
