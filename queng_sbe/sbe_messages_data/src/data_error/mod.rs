use common_sbe_errors::DataError;
use sbe_types::{SbeDecodeError, SbeEncodeError};

mod sbe_decode;
mod sbe_encode;

#[inline]
pub fn encode_data_error_message(data: DataError) -> Result<(usize, Vec<u8>), SbeEncodeError> {
    sbe_encode::encode_data_error_message(data)
}

#[inline]
pub fn decode_data_error_message(buffer: &[u8]) -> Result<DataError, SbeDecodeError> {
    sbe_decode::decode_data_error_message(buffer)
}
