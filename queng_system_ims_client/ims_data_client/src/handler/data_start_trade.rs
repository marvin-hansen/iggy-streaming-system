use crate::{ImsClientError, ImsDataClient};
use common_data_bar::TimeResolution;
use sbe_messages_control::StartDataMessage;
use sbe_types::DataType;

impl ImsDataClient {
    pub(crate) async fn client_start_trade_data(
        &self,
        symbol_id: String,
    ) -> Result<(), ImsClientError> {
        self.dbg_print("start_trade_data");

        self.dbg_print("Construct start_trade_data message");
        let start_data_message = StartDataMessage::new(
            self.client_id,
            self.exchange_id,
            symbol_id,
            TimeResolution::NoValue,
            DataType::TradeData,
        );

        self.dbg_print("Encode SBE message");
        let (_, message) = match start_data_message.encode() {
            Ok(res) => res,
            Err(err) => {
                return Err(ImsClientError::FailedToEncodeControlMessage(format!(
                    "[ImsDataClient/start_data]: Failed to encode start_trade_data message: {err}"
                )))
            }
        };

        self.dbg_print("Send start_data message");
        match self.send_one_message(message.as_slice()).await {
            Ok(_) => {}
            Err(err) => {
                return Err(ImsClientError::FailedToSendControlMessageToIggyServer(
                    format!(
                    "[ImsDataClient/start_data]: Failed to send start_trade_data message: {err}"
                ),
                ))
            }
        }

        Ok(())
    }
}
