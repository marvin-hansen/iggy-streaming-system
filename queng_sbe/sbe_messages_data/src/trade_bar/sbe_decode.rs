use chrono::{DateTime, TimeZone, Utc};
use common_data_bar::TradeBar;
use rust_decimal::Decimal;
use sbe_bindings::trade_bar_codec::SBE_TEMPLATE_ID;
use sbe_bindings::{
    ReadBuf, message_header_codec::MessageHeaderDecoder, trade_bar_codec::TradeBarDecoder,
};
use sbe_types::{MessageType, SbeDecodeError};

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
/// # Process
///
/// - Create default `TradeBarDecoder`
/// - Wrap buffer in `ReadBuf`
/// - Decode header and validate template ID
/// - Decode and validate `message_type`
/// - Decode `symbol_id`
/// - Decode `date_time` as timestamp and create `DateTime`
/// - Decode price as SBE decimal and convert to Decimal
/// - Decode volume as SBE decimal and convert to Decimal
/// - Create and return `TradeBar`
///
#[inline]
pub(crate) fn decode_trade_bar_message(buffer: &[u8]) -> Result<TradeBar, SbeDecodeError> {
    let mut csg = TradeBarDecoder::default();
    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header, 0);

    let sbe_message_type = csg.message_type();
    let message_type = MessageType::from(sbe_message_type as u16);
    assert_eq!(message_type, MessageType::TradeBar);

    let raw_string = String::from_utf8(csg.symbol_id().to_ascii_uppercase())
        .expect("Failed to decode symbol_id");
    // Remove trailing null characters https://stackoverflow.com/questions/49406517/how-to-remove-trailing-null-characters-from-string
    let symbol_id = raw_string.trim().trim_matches(char::from(0)).to_string();

    let sbe_date_time = csg.date_time();
    let date_time: DateTime<Utc> = Utc.timestamp_micros(sbe_date_time).unwrap();

    let price_decoder = csg.price_decoder();
    let price = Decimal::new(price_decoder.num(), price_decoder.scale() as u32);

    let volume_decoder = csg.volume_decoder();
    let volume = Decimal::new(volume_decoder.num(), volume_decoder.scale() as u32);

    Ok(TradeBar::new(symbol_id, date_time, price, volume))
}
