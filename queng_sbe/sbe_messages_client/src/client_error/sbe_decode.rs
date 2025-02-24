use common_sbe_errors::ClientError;
use sbe_bindings::client_error_codec::SBE_TEMPLATE_ID;
use sbe_bindings::{
    ReadBuf, client_error_codec::ClientErrorDecoder, message_header_codec::MessageHeaderDecoder,
};
use sbe_types::{MessageType, SbeDecodeError};

/// Decodes a `ClientErrorMessage` from a byte buffer.
///
/// # Arguments
///
/// * `buffer` - Byte buffer containing encoded `ClientErrorMessage`
///
/// # Returns
///
/// Decoded `ClientErrorMessage` on success
///
/// # Errors
///
/// Returns Err if decoding fails
///
/// # Process
///
/// - Create default `ClientErrorDecoder`
/// - Wrap buffer in `ReadBuf`
/// - Decode header and validate template ID
/// - Decode and validate `message_type`
/// - Decode `client_id`
/// - Decode and validate `client_error_type`
/// - Create and return `ClientErrorMessage`
///
pub(crate) fn decode_client_error_message(buffer: &[u8]) -> Result<ClientError, SbeDecodeError> {
    let buf = ReadBuf::new(buffer);

    let mut csg = ClientErrorDecoder::default();

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header, 0);

    let sbe_message_type = csg.message_type();
    let message_type = MessageType::from(sbe_message_type as u16);
    assert_eq!(message_type, MessageType::ClientError);

    let client_id = csg.client_id();
    let client_error_type_raw = csg.client_error_type();

    let message = ClientError::new(client_id, client_error_type_raw);

    Ok(message)
}
