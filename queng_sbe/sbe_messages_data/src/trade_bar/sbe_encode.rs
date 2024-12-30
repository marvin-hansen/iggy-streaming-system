use common_data_bar::TradeBar;
use rust_decimal::prelude::ToPrimitive;

use sbe_bindings::{
    message_type::MessageType as SbeMessageType, trade_bar_codec::TradeBarEncoder, Encoder,
    WriteBuf,
};

use sbe_bindings::message_header_codec::ENCODED_LENGTH;

use sbe_types::SbeEncodeError;

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
/// # Process
///
/// - Create 46 byte buffer
/// - Create default `TradeBarEncoder`
/// - Wrap buffer in `WriteBuf`
/// - Encode header
/// - Encode `message_type`
/// - Encode `symbol_id`
/// - Encode `date_time` as timestamp
/// - Convert price to SBE decimal
/// - Encode price
/// - Convert volume to SBE decimal
/// - Encode volume
/// - Return encoded size and buffer
///
#[inline]
pub(crate) fn encode_trade_bar_message(bar: TradeBar) -> Result<(usize, Vec<u8>), SbeEncodeError> {
    let mut buffer = vec![0u8; 56];

    let mut csg = TradeBarEncoder::default();

    csg = csg.wrap(WriteBuf::new(buffer.as_mut_slice()), ENCODED_LENGTH);

    csg = csg.header(0).parent().expect("Failed to encode header");

    let value = SbeMessageType::TradeBar;
    csg.message_type(value);

    // Convert string symbol id into fixed sized char [u8; 20]
    let mut byte_array = [0u8; 20];
    byte_array[..bar.symbol_id().len()].copy_from_slice(bar.symbol_id().as_bytes());
    csg.symbol_id(&byte_array);

    let date_time = bar.date_time().timestamp_micros();
    csg.date_time(date_time);

    let mut price_encoder = csg.price_encoder();
    price_encoder.num(
        bar.price()
            .mantissa()
            .to_i64()
            .expect("Failed to convert price decimal to i64"),
    );
    price_encoder.scale(bar.price().scale() as u8);
    csg = price_encoder
        .parent()
        .expect("Failed to encode trade price");

    let mut volume_encoder = csg.volume_encoder();

    volume_encoder.num(
        bar.volume()
            .mantissa()
            .to_i64()
            .expect("Failed to convert volume decimal to i64"),
    );
    volume_encoder.scale(bar.volume().scale() as u8);
    csg = volume_encoder
        .parent()
        .expect("Failed to encode trade volume");

    let limit = csg.get_limit();
    Ok((limit, buffer))
}
