use crate::StopAllDataMessage;
use common_exchange::ExchangeID;
use sbe_types::MessageType;

impl StopAllDataMessage {
    #[must_use]
    pub const fn message_type(&self) -> &MessageType {
        &self.message_type
    }
    #[must_use]
    pub const fn client_id(&self) -> &u16 {
        &self.client_id
    }
    #[must_use]
    pub const fn exchange_id(&self) -> &ExchangeID {
        &self.exchange_id
    }
}
