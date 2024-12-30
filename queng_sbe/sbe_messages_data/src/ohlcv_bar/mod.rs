use common_data_bar::OHLCVBar;
use sbe_types::{SbeDecodeError, SbeEncodeError};

pub mod sbe_decoder;
pub mod sbe_encoder;

/// Encodes an `OHLCVBar` to a byte buffer.
///
/// # Parameters
///
/// - `bar` - The `OHLCVBar` to encode
///
/// # Returns
///
/// A Result containing the number of bytes written and the encoded byte buffer or an encoding error.
#[inline]
pub fn encode_data_bar_message(bar: OHLCVBar) -> Result<(usize, Vec<u8>), SbeEncodeError> {
    sbe_encoder::encode_data_bar_message(bar)
}

/// Decodes an SBE message buffer into an `OHLCVBar`.
///
/// # Parameters
///
/// - `buffer` - The SBE encoded message buffer
///
/// # Returns
///
/// A Result containing the decoded `OHLCVBar` or a decoding error.
#[inline]
pub fn decode_data_bar_message(buffer: &[u8]) -> Result<OHLCVBar, SbeDecodeError> {
    sbe_decoder::decode_data_bar_message(buffer)
}
