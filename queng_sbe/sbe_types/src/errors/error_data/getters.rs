use crate::{DataErrorMessage, DataErrorType, MessageType};

impl DataErrorMessage {
    #[must_use]
    pub const fn message_type(&self) -> MessageType {
        self.message_type
    }
    #[must_use]
    pub const fn client_id(&self) -> u16 {
        self.client_id
    }
    #[must_use]
    pub const fn data_error_type(&self) -> DataErrorType {
        self.data_error_type
    }
}
