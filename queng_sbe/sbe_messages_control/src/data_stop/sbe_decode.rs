use common_exchange::ExchangeID;
use sbe_bindings::{
    message_header_codec::MessageHeaderDecoder, stop_data_msg_codec::StopDataMsgDecoder, ReadBuf,
    SbeResult,
};

use crate::StopDataMessage;
use sbe_bindings::stop_data_msg_codec::SBE_TEMPLATE_ID;
use sbe_types::{DataType, MessageType};

/// Decodes a `StopDataMessage` from a byte buffer.
///
/// # Arguments
///
/// * `buffer` - Byte buffer to decode
///
/// # Returns
///
/// Decoded `StopDataMessage`
///
/// # Errors
///
/// Returns Err if decode fails
///
/// # Process
///
/// - Create default `StopDataMsgDecoder`
/// - Wrap buffer in `ReadBuf`
/// - Decode header and validate template ID
/// - Decode and validate `message_type`
/// - Decode `client_id`
/// - Decode and create `exchange_id`
/// - Decode `symbol_id`
/// - Decode and create `data_type_id`
/// - Create and return `StopDataMessage`
///
pub fn decode_stop_data_message(buffer: &[u8]) -> SbeResult<StopDataMessage> {
    let mut csg = StopDataMsgDecoder::default();
    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header, 0);

    let sbe_message_type = csg.message_type();
    let message_type = MessageType::from(sbe_message_type as u16);
    assert_eq!(message_type, MessageType::StopData);

    let client_id = csg.client_id();
    let exchange_id = ExchangeID::from(csg.exchange_id());

    let raw_string = String::from_utf8(csg.symbol_id().to_ascii_uppercase())
        .expect("Failed to decode symbol_id");
    // Remove trailing null characters
    // https://stackoverflow.com/questions/49406517/how-to-remove-trailing-null-characters-from-string
    let symbol_id = raw_string.trim().trim_matches(char::from(0)).to_string();

    let data_type_id = DataType::from(csg.data_type_id());

    let message = StopDataMessage {
        message_type,
        client_id,
        exchange_id,
        symbol_id,
        data_type_id,
    };

    Ok(message)
}
