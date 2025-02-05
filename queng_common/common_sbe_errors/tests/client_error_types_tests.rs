use common_sbe_errors::ClientErrorType;

#[test]
fn test_client_error_type_default() {
    let error_type = ClientErrorType::default();
    assert_eq!(error_type, ClientErrorType::UnknownClientError);
}

#[test]
fn test_client_error_type_from_u8() {
    assert_eq!(ClientErrorType::from(0), ClientErrorType::UnknownClientError);
    assert_eq!(ClientErrorType::from(1), ClientErrorType::ClientAlreadyLoggedIn);
    assert_eq!(ClientErrorType::from(2), ClientErrorType::ClientLogInError);
    assert_eq!(ClientErrorType::from(3), ClientErrorType::ClientNotLoggedIn);
    assert_eq!(ClientErrorType::from(4), ClientErrorType::ClientLogOutError);
    assert_eq!(ClientErrorType::from(5), ClientErrorType::ClientNotAuthorized);
    assert_eq!(ClientErrorType::from(6), ClientErrorType::ClientShutdownError);
    assert_eq!(ClientErrorType::from(7), ClientErrorType::UnknownClientError);
}

#[test]
fn test_client_error_type_to_u8() {
    assert_eq!(u8::from(ClientErrorType::UnknownClientError), 0);
    assert_eq!(u8::from(ClientErrorType::ClientAlreadyLoggedIn), 1);
    assert_eq!(u8::from(ClientErrorType::ClientLogInError), 2);
    assert_eq!(u8::from(ClientErrorType::ClientNotLoggedIn), 3);
    assert_eq!(u8::from(ClientErrorType::ClientLogOutError), 4);
    assert_eq!(u8::from(ClientErrorType::ClientNotAuthorized), 5);
    assert_eq!(u8::from(ClientErrorType::ClientShutdownError), 6);
}

#[test]
fn test_client_error_type_display() {
    assert_eq!(format!("{}", ClientErrorType::UnknownClientError), "UnknownClientError");
    assert_eq!(format!("{}", ClientErrorType::ClientAlreadyLoggedIn), "ClientAlreadyLoggedIn");
    assert_eq!(format!("{}", ClientErrorType::ClientLogInError), "ClientLogInError");
    assert_eq!(format!("{}", ClientErrorType::ClientNotLoggedIn), "ClientNotLoggedIn");
    assert_eq!(format!("{}", ClientErrorType::ClientLogOutError), "ClientLogOutError");
    assert_eq!(format!("{}", ClientErrorType::ClientNotAuthorized), "ClientNotAuthorized");
    assert_eq!(format!("{}", ClientErrorType::ClientShutdownError), "ClientShutdownError");
}

#[test]
fn test_client_error_type_ordering() {
    assert!(ClientErrorType::UnknownClientError < ClientErrorType::ClientAlreadyLoggedIn);
    assert!(ClientErrorType::ClientAlreadyLoggedIn < ClientErrorType::ClientLogInError);
    assert!(ClientErrorType::ClientLogInError < ClientErrorType::ClientNotLoggedIn);
    assert!(ClientErrorType::ClientNotLoggedIn < ClientErrorType::ClientLogOutError);
    assert!(ClientErrorType::ClientLogOutError < ClientErrorType::ClientNotAuthorized);
    assert!(ClientErrorType::ClientNotAuthorized < ClientErrorType::ClientShutdownError);
}
