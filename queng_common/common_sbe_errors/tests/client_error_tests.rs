use common_sbe_errors::{ClientError, ClientErrorType};

#[test]
fn test_client_error_new() {
    let error = ClientError::new(123, 0);
    assert_eq!(error.client_id(), 123);
    assert_eq!(
        error.client_error_type(),
        ClientErrorType::UnknownClientError
    );
}

#[test]
fn test_client_error_default() {
    let error = ClientError::default();
    assert_eq!(error.client_id(), 0);
    assert_eq!(
        error.client_error_type(),
        ClientErrorType::UnknownClientError
    );
}

#[test]
fn test_client_error_display() {
    let error = ClientError::new(456, 0);
    let display_output = format!("{}", error);
    assert_eq!(
        display_output,
        "ClientError { client_id: 456, client_error_type: UnknownClientError }"
    );
}
