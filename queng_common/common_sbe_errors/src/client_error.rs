use crate::client_error_types::ClientErrorType;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientError {
    client_id: u16,
    client_error_type: ClientErrorType,
}

impl ClientError {
    pub fn new(client_id: u16, client_error_type: u8) -> Self {
        let client_error_type = ClientErrorType::from(client_error_type);
        Self {
            client_id,
            client_error_type,
        }
    }
}

impl ClientError {
    pub fn client_id(&self) -> u16 {
        self.client_id
    }

    pub fn client_error_type(&self) -> ClientErrorType {
        self.client_error_type
    }
}

impl Default for ClientError {
    fn default() -> Self {
        Self {
            client_id: 0,
            client_error_type: ClientErrorType::UnknownClientError,
        }
    }
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ClientError {{ client_id: {}, client_error_type: {} }}",
            self.client_id, self.client_error_type
        )
    }
}
