use std::fmt::{Display, Formatter};

/// Enumeration of possible data error types.
///
/// The variants represent the following error conditions:
///
/// * `UnknownDataError` - Default error when the specific cause is unknown.
/// * `DataTypeNotKnownError` - The requested data type is not recognized.
/// * `DataUnavailableError` - The requested data is not available.
/// * `DataEncodingError` - Error encoding the data.
/// * `DataTableNotFound` - The requested data table does not exist.
/// * `DataSendError` - Error sending the requested data.
/// * `DataChannelError` - Error getting the clients data channel.
/// * `DataWrongExchangeError` - Error getting the clients data exchange.
/// * `DataClientNotLoggedInError` - The client is not logged in.
///
/// The enum variants are represented as `u8` values for serialization.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum DataErrorType {
    #[default]
    UnknownDataError = 0_u8,
    DataTypeNotKnownError = 1_u8,
    DataUnavailableError = 2_u8,
    DataEncodingError = 3_u8,
    DataTableNotFound = 4_u8,
    DataSendError = 5_u8,
    DataChannelError = 6_u8,
    DataWrongExchangeError = 7_u8,
    DataClientNotLoggedInError = 8_u8,
    DataStartError = 9_u8,
    DataStopError = 10_u8,
    DataStopAllError = 11_u8,
}

impl From<u8> for DataErrorType {
    /// Implements conversion from `u8` to `DataErrorType`.
    ///
    /// Maps the `u8` value to the corresponding `DataErrorType` variant:
    ///
    /// * 0 -> `UnknownDataError`
    /// * 1 -> `DataTypeNotKnownError`
    /// * 2 -> `DataUnavailableError`
    /// * 3 -> `DataEncodingError`
    /// * 4 -> `DataTableNotFound`
    /// * 5 -> `DataSendError`
    /// * 6 -> `DataChannelError`
    /// * 7 -> `DataWrongExchangeError`
    /// * 8 -> `DataClientNotLoggedInError`
    ///
    /// Any other `u8` value maps to `UnknownDataError`.
    ///
    /// This allows deserializing a `u8` into a `DataErrorType`.
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0_u8 => Self::UnknownDataError,
            1_u8 => Self::DataTypeNotKnownError,
            2_u8 => Self::DataUnavailableError,
            3_u8 => Self::DataEncodingError,
            4_u8 => Self::DataTableNotFound,
            5_u8 => Self::DataSendError,
            6_u8 => Self::DataChannelError,
            7_u8 => Self::DataWrongExchangeError,
            8_u8 => Self::DataClientNotLoggedInError,
            9_u8 => Self::DataStartError,
            10_u8 => Self::DataStopError,
            11_u8 => Self::DataStopAllError,
            _ => Self::UnknownDataError,
        }
    }
}

impl Display for DataErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
