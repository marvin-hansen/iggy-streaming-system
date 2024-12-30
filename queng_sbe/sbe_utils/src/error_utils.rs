use common_errors::MessageProcessingError;
use sbe_types::{ClientErrorMessage, ClientErrorType, DataErrorMessage, DataErrorType};

/// Encodes a `ClientErrorMessage` message for the given client id and client error type.
///
/// # Parameters
///
/// * `client_id` - The id of the client to send the message to.
/// * `client_error` - The client error type to encode.
///
/// # Returns
///
/// Returns a `Result` with the encoded message buffer if successful, otherwise returns a
/// `(DataErrorType, MessageProcessingError)` tuple containing:
///
/// - `DataErrorType::DataEncodingError` if encoding fails
/// - The underlying encoding error wrapped in `MessageProcessingError`
///
pub fn encode_client_error(
    client_id: u16,
    client_error: ClientErrorType,
) -> Result<Vec<u8>, (DataErrorType, MessageProcessingError)> {
    let message = ClientErrorMessage::new(client_id, client_error);
    let enc_result = message.encode();
    match enc_result {
        Ok((_, bytes)) => Ok(bytes),
        Err(e) => Err((
            DataErrorType::DataEncodingError,
            MessageProcessingError(e.to_string()),
        )),
    }
}

/// Encodes a `DataErrorMessage` message for the given client id and data error type.
///
/// # Parameters
///
/// * `client_id` - The id of the client to send the message to.
/// * `data_error` - The data error type to encode.
///
/// # Returns
///
/// Returns a `Result` with the encoded message buffer if successful, otherwise returns a
/// `(DataErrorType, MessageProcessingError)` tuple containing:
///
/// - `DataErrorType::DataEncodingError` if encoding fails
/// - The underlying encoding error wrapped in `MessageProcessingError`
///
pub fn encode_data_error(
    client_id: u16,
    data_error: DataErrorType,
) -> Result<Vec<u8>, (DataErrorType, MessageProcessingError)> {
    let message = DataErrorMessage::new(client_id, data_error);
    let enc_result = message.encode();
    match enc_result {
        Ok((_, bytes)) => Ok(bytes),
        Err(e) => Err((
            DataErrorType::DataEncodingError,
            MessageProcessingError(e.to_string()),
        )),
    }
}
