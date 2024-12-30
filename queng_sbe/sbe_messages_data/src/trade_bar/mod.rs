use common_data_bar::TradeBar;
use sbe_types::{SbeDecodeError, SbeEncodeError};

mod sbe_decode;
mod sbe_encode;

/// Encodes a `TradeBar` message to a byte buffer.
///
/// # Arguments
///
/// * `bar` - `TradeBar` to encode
///
/// # Returns
///
/// (usize, `Vec<u8>`) - Tuple containing encoded size and byte buffer
///
/// # Errors
///
/// Returns Err if encoding fails
///
/// # Remarks
///
/// Calls `sbe_encode::encode_trade_bar_message` to perform encoding
///
#[inline]
pub fn encode_trade_bar_message(bar: TradeBar) -> Result<(usize, Vec<u8>), SbeEncodeError> {
    sbe_encode::encode_trade_bar_message(bar)
}

/// Decodes a `TradeBar` message from a byte buffer.
///
/// # Arguments
///
/// * `buffer` - Byte buffer containing encoded `TradeBar` message
///
/// # Returns
///
/// Decoded `TradeBar` on success
///
/// # Errors
///
/// Returns Err if decoding fails
///
/// # Remarks
///
/// Calls `sbe_decode::decode_trade_bar_message` to perform decoding
///
#[inline]
pub fn decode_trade_bar_message(buffer: &[u8]) -> Result<TradeBar, SbeDecodeError> {
    sbe_decode::decode_trade_bar_message(buffer)
}
