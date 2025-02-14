use chrono::{DateTime, TimeZone, Utc};
use common_data_bar::OHLCVBar;
use rust_decimal::Decimal;
use sbe_bindings::data_bar_codec::SBE_TEMPLATE_ID;
use sbe_bindings::{
    ReadBuf, data_bar_codec::DataBarDecoder, message_header_codec::MessageHeaderDecoder,
};
use sbe_types::{MessageType, SbeDecodeError};

/// Decodes an `OHLCVBar` message from a byte buffer.
///
/// # Arguments
///
/// * `buffer` - Byte buffer to decode
///
/// # Returns
///
/// Decoded `OHLCVBar`
///
/// # Errors
///
/// Returns Err if decode fails
///
/// # Process
///
/// - Create default `DataBarDecoder`
/// - Wrap buffer in `ReadBuf`
/// - Decode header and validate template ID
/// - Decode and validate `message_type`
/// - Decode `symbol_id`
/// - Decode and parse `date_time`
/// - Decode and parse `open_price`
/// - Decode and parse `high_price`
/// - Decode and parse `low_price`
/// - Decode and parse `close_price`
/// - Decode and parse volume
/// - Create and return `OHLCVBar`
///
#[inline]
pub(crate) fn decode_data_bar_message(buffer: &[u8]) -> Result<OHLCVBar, SbeDecodeError> {
    let mut csg = DataBarDecoder::default();
    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header, 0);

    let sbe_message_type = csg.message_type();
    let message_type = MessageType::from(sbe_message_type as u16);
    assert_eq!(message_type, MessageType::OHLCVBar);

    // Decode symbol_id
    let raw_string = String::from_utf8(csg.symbol_id().to_ascii_uppercase())
        .expect("Failed to decode symbol_id");
    // Remove trailing null characters https://stackoverflow.com/questions/49406517/how-to-remove-trailing-null-characters-from-string
    let symbol_id = raw_string.trim().trim_matches(char::from(0)).to_string();

    // Decode date_time
    let sbe_date_time = csg.date_time();
    let date_time: DateTime<Utc> = Utc.timestamp_micros(sbe_date_time).unwrap();

    // Decode open price
    let decoder = csg.open_price_decoder();
    let open = Decimal::new(decoder.num(), decoder.scale() as u32);

    // Decode high price
    let decoder = csg.high_price_decoder();
    let high = Decimal::new(decoder.num(), decoder.scale() as u32);

    // Decode low price
    let decoder = csg.low_price_decoder();
    let low = Decimal::new(decoder.num(), decoder.scale() as u32);

    // Decode close price
    let decoder = csg.close_price_decoder();
    let close = Decimal::new(decoder.num(), decoder.scale() as u32);

    // Decode volume
    let decoder = csg.volume_decoder();
    let volume = Decimal::new(decoder.num(), decoder.scale() as u32);

    Ok(OHLCVBar::new(
        symbol_id, date_time, open, high, low, close, volume,
    ))
}
