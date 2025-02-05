use common_sbe_errors::{DataError, DataErrorType};

#[test]
fn test_data_error_new() {
    let error = DataError::new(123, 0);
    assert_eq!(error.client_id(), 123);
    assert_eq!(error.data_error_type(), DataErrorType::UnknownDataError);
}

#[test]
fn test_data_error_default() {
    let error = DataError::default();
    assert_eq!(error.client_id(), 0);
    assert_eq!(error.data_error_type(), DataErrorType::UnknownDataError);
}

#[test]
fn test_data_error_display() {
    let error = DataError::new(456, 1);
    let display_output = format!("{}", error);
    assert_eq!(
        display_output,
        "DataError: client_id=456, data_error_type=DataTypeNotKnownError"
    );
}

#[test]
fn test_data_error_debug() {
    let error = DataError::new(789, 1);
    let debug_output = format!("{:?}", error);
    assert!(debug_output.contains("789"));
    assert!(debug_output.contains("DataTypeNotKnownError"));
}
