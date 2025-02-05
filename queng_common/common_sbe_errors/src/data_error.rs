use crate::DataErrorType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataError {
    client_id: u16,
    data_error_type: DataErrorType,
}

impl DataError {
    pub fn new(client_id: u16, data_error_type: u8) -> Self {
        let data_error_type = DataErrorType::from(data_error_type);
        Self {
            client_id,
            data_error_type,
        }
    }
}

impl DataError {
    pub fn client_id(&self) -> u16 {
        self.client_id
    }

    pub fn data_error_type(&self) -> DataErrorType {
        self.data_error_type
    }
}

impl Default for DataError {
    fn default() -> Self {
        Self {
            client_id: 0,
            data_error_type: DataErrorType::UnknownDataError,
        }
    }
}

impl std::fmt::Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DataError: client_id={}, data_error_type={}",
            self.client_id, self.data_error_type
        )
    }
}
