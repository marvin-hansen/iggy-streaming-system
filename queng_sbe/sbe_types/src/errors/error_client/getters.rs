use crate::{ClientErrorMessage, ClientErrorType, MessageType};

impl ClientErrorMessage {
    #[must_use]
    pub const fn message_type(&self) -> MessageType {
        self.message_type
    }
    #[must_use]
    pub const fn client_id(&self) -> u16 {
        self.client_id
    }
    #[must_use]
    pub const fn client_error_type(&self) -> ClientErrorType {
        self.client_error_type
    }
}
