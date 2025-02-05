use common_sbe_errors::ClientError;
use sbe_bindings::message_type::MessageType as SbeMessageType;
use sbe_bindings::{
    client_error_codec::ClientErrorEncoder, message_header_codec, Encoder, WriteBuf,
};
use sbe_types::SbeEncodeError;

/// Encodes a `ClientErrorMessage` to a byte buffer.
///
/// # Arguments
///
/// * `self` - `ClientErrorMessage` to encode
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
/// - Create 13 byte buffer
/// - Create default `ClientErrorEncoder`
/// - Wrap buffer in `WriteBuf`
/// - Encode header
/// - Encode `message_type`
/// - Encode `client_id`
/// - Encode `client_error_type`
/// - Return encoded size and buffer
///
pub(crate) fn encode_client_error_message(
    client_error: ClientError,
) -> Result<(usize, Vec<u8>), SbeEncodeError> {
    let mut buffer = vec![0u8; 13];

    let mut csg = ClientErrorEncoder::default();

    csg = csg.wrap(
        WriteBuf::new(buffer.as_mut_slice()),
        message_header_codec::ENCODED_LENGTH,
    );

    csg = csg.header(0).parent().expect("Failed to encode header");

    let value = SbeMessageType::ClientError;
    csg.message_type(value);

    let value = client_error.client_id();
    csg.client_id(value);

    let value = client_error.client_error_type() as u8;
    csg.client_error_type(value);

    let limit = csg.get_limit();
    Ok((limit, buffer))
}
