use common_sbe_errors::DataError;
use sbe_bindings::data_error_codec::SBE_TEMPLATE_ID;
use sbe_bindings::{
    ReadBuf, data_error_codec::DataErrorDecoder, message_header_codec::MessageHeaderDecoder,
};
use sbe_types::{MessageType, SbeDecodeError};

/// Decodes a `DataErrorMessage` from a byte buffer.
///
/// # Arguments
///
/// * `buffer` - Byte buffer containing encoded `DataErrorMessage`
///
/// # Returns
///
/// Decoded `DataErrorMessage` on success
///
/// # Errors
///
/// Returns Err if decoding fails
///
/// # Process
///
/// - Create default `DataErrorDecoder`
/// - Wrap buffer in `ReadBuf`
/// - Decode header and validate template ID
/// - Decode and validate `message_type`
/// - Decode `client_id`
/// - Decode and validate `data_error_type`
/// - Create and return `DataErrorMessage`
///
pub(crate) fn decode_data_error_message(buffer: &[u8]) -> Result<DataError, SbeDecodeError> {
    let mut csg = DataErrorDecoder::default();
    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());

    csg = csg.header(header, 0);

    let sbe_message_type = csg.message_type();
    let message_type = MessageType::from(sbe_message_type as u16);
    assert_eq!(message_type, MessageType::DataError);

    let client_id = csg.client_id();
    let data_error_type_raw = csg.data_error_type();

    let message = DataError::new(client_id, data_error_type_raw);

    Ok(message)
}
