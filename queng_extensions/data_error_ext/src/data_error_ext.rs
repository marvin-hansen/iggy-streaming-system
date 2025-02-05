use common_sbe_errors::DataError;
use sbe_types::{SbeDecodeError, SbeEncodeError};

/// Provides an extension trait for `DataError` to encode and decode into an SBE message.
pub trait SbeDataErrorExtension {
    /// Encodes `self` into an SBE message.
    ///
    /// # Parameters
    ///
    /// * `self` - The `DataError` to encode
    ///
    /// # Returns
    ///
    /// Returns a `Result` with a tuple containing the size of the encoded message and the encoded message as a `Vec<u8>`.
    /// If encoding fails a `SbeEncodeError` is returned.
    ///
    fn encode_to_sbe(self) -> Result<(usize, Vec<u8>), SbeEncodeError>;

    /// Decodes a `DataError` from an SBE message.
    ///
    /// # Parameters
    ///
    /// * `buffer` - The SBE message to decode
    ///
    /// # Returns
    ///
    /// Returns a `Result` with the decoded `DataError` if successful, otherwise returns a `SbeDecodeError`.
    ///
    fn decode_from_sbe(buffer: &[u8]) -> Result<DataError, SbeDecodeError>;
}

impl SbeDataErrorExtension for DataError {
    fn encode_to_sbe(self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        sbe_messages_data::encode_data_error_message(self)
    }

    fn decode_from_sbe(buffer: &[u8]) -> Result<DataError, SbeDecodeError> {
        sbe_messages_data::decode_data_error_message(buffer)
    }
}
