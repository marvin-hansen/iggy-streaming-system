use common_data_bar::OHLCVBar;
use rust_decimal::prelude::ToPrimitive;

use sbe_bindings::{
    Encoder, WriteBuf, data_bar_codec::DataBarEncoder, message_header_codec,
    message_type::MessageType as SbeMessageType,
};
use sbe_types::SbeEncodeError;

/// Encodes an `OHLCVBar` to a byte buffer.
///
/// # Arguments
///
/// * `bar` - `OHLCVBar` to encode
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
/// - Create 58 byte buffer
/// - Create default `DataBarEncoder`
/// - Wrap buffer in `WriteBuf`
/// - Encode header
/// - Encode `message_type`
/// - Encode `symbol_id`
/// - Encode `date_time`
/// - Encode and convert `open_price` to SBE Decimal
/// - Encode and convert `high_price` to  SBE Decimal
/// - Encode and convert `low_price` to  SBE Decimal
/// - Encode and convert `close_price` to  SBE Decimal
/// - Encode and convert volume to  SBE Decimal
/// - Return encoded size and buffer
///
#[inline]
pub(crate) fn encode_data_bar_message(bar: OHLCVBar) -> Result<(usize, Vec<u8>), SbeEncodeError> {
    // precise buffer size is 58 bytes for the entire message.
    let mut buffer = vec![0u8; 83];

    let mut csg = DataBarEncoder::default();

    csg = csg.wrap(
        WriteBuf::new(buffer.as_mut_slice()),
        message_header_codec::ENCODED_LENGTH,
    );

    csg = csg.header(0).parent().expect("Failed to encode header");

    csg.message_type(SbeMessageType::DataBar);

    // Convert string symbol id into fixed sized char [u8; 20]
    let mut byte_array = [0u8; 20];
    byte_array[..bar.symbol_id().len()].copy_from_slice(bar.symbol_id().as_bytes());
    csg.symbol_id(&byte_array);

    csg.date_time(bar.date_time().timestamp_micros());

    // Encode and convert open price to SBE decimal
    let mut open_price_encoder = csg.open_price_encoder();
    open_price_encoder.num(
        bar.open()
            .mantissa()
            .to_i64()
            .expect("Failed to convert price decimal to i64"),
    );
    open_price_encoder.scale(bar.open().scale() as u8);
    csg = open_price_encoder
        .parent()
        .expect("Failed to encode trade price");

    // Encode and convert high price to SBE decimal
    let mut high_price_encoder = csg.high_price_encoder();
    high_price_encoder.num(
        bar.high()
            .mantissa()
            .to_i64()
            .expect("Failed to convert price decimal to i64"),
    );
    high_price_encoder.scale(bar.high().scale() as u8);
    csg = high_price_encoder
        .parent()
        .expect("Failed to encode trade price");

    // Encode and convert low price to SBE decimal
    let mut low_price_encoder = csg.low_price_encoder();
    low_price_encoder.num(
        bar.low()
            .mantissa()
            .to_i64()
            .expect("Failed to convert price decimal to i64"),
    );
    low_price_encoder.scale(bar.low().scale() as u8);
    csg = low_price_encoder
        .parent()
        .expect("Failed to encode trade price");

    // Encode and convert close price to SBE decimal
    let mut close_price_encoder = csg.close_price_encoder();
    close_price_encoder.num(
        bar.close()
            .mantissa()
            .to_i64()
            .expect("Failed to convert price decimal to i64"),
    );
    close_price_encoder.scale(bar.close().scale() as u8);
    csg = close_price_encoder
        .parent()
        .expect("Failed to encode trade price");

    // Encode and convert volume to SBE decimal
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
