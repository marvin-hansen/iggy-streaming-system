use crate::StartDataMessage;
use std::fmt;

impl fmt::Display for StartDataMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "StartDataMessage[message_type: {}, client_id: {}, exchange_id: {}, symbol_id: {} time_resolution: {} data_type: {}]",
            self.message_type,
            self.client_id,
            self.exchange_id,
            self.symbol_id,
            self.time_resolution,
            self.data_type_id
        )
    }
}
