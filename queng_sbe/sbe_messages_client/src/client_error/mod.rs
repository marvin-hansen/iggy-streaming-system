use common_sbe_errors::ClientError;
use sbe_types::{SbeDecodeError, SbeEncodeError};

mod sbe_decode;
mod sbe_encode;

#[inline]
pub fn encode_client_error_message(data: ClientError) -> Result<(usize, Vec<u8>), SbeEncodeError> {
    sbe_encode::encode_client_error_message(data)
}

#[inline]
pub fn decode_client_error_message(buffer: &[u8]) -> Result<ClientError, SbeDecodeError> {
    sbe_decode::decode_client_error_message(buffer)
}
