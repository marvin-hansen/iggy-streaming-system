use common_errors::DBGatewayError;

#[test]
fn test_new_dbgw_error() {
    let error = DBGatewayError("test error".to_string());
    assert_eq!(error.to_string(), "DBGatewayError: test error");
}

#[test]
fn test_debug() {
    let error = DBGatewayError("test error".to_string());
    assert_eq!(format!("{error:?}"), "DBGatewayError(\"test error\")");
}

#[test]
fn test_display() {
    let error = DBGatewayError("test error".to_string());
    assert_eq!(format!("{error}"), "DBGatewayError: test error");
}
