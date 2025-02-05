use common_sbe_errors::ClientError;
use sbe_types::{SbeDecodeError, SbeEncodeError};

/// Provides an extension trait for `ClientError` to encode and decode into an SBE message.

pub trait SbeClientErrorExtension {
    /// Encodes a `ClientError` to a byte buffer.
    ///
    /// # Arguments
    ///
    /// * `data` - `ClientError` to encode
    ///
    /// # Returns
    ///
    /// A `Result` containing the number of bytes written (`usize`) and the encoded byte buffer (`Vec<u8>`) on success
    ///
    /// # Errors
    ///
    /// Returns `Err` if encoding fails, with the error type `SbeEncodeError`
    ///
    fn encode_to_sbe(self) -> Result<(usize, Vec<u8>), SbeEncodeError>;

    /// Decodes a `ClientError` from a byte buffer.
    ///
    /// # Arguments
    ///
    /// * `buffer` - Byte buffer containing encoded `ClientError`
    ///
    /// # Returns
    ///
    /// Decoded `ClientError` on success
    ///
    /// # Errors
    ///
    /// Returns `Err` if decoding fails, with the error type `SbeDecodeError`
    ///
    fn decode_from_sbe(buffer: &[u8]) -> Result<ClientError, SbeDecodeError>;
}

impl SbeClientErrorExtension for ClientError {
    fn encode_to_sbe(self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        sbe_messages_client::encode_client_error_message(self)
    }

    fn decode_from_sbe(buffer: &[u8]) -> Result<ClientError, SbeDecodeError> {
        sbe_messages_client::decode_client_error_message(buffer)
    }
}
