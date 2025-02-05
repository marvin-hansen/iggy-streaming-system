use common_sbe_errors::DataErrorType;

#[test]
fn test_data_error_type_default() {
    let error_type = DataErrorType::default();
    assert_eq!(error_type, DataErrorType::UnknownDataError);
}

#[test]
fn test_data_error_type_from_u8() {
    assert_eq!(DataErrorType::from(0), DataErrorType::UnknownDataError);
    assert_eq!(DataErrorType::from(1), DataErrorType::DataTypeNotKnownError);
    assert_eq!(DataErrorType::from(2), DataErrorType::DataUnavailableError);
    assert_eq!(DataErrorType::from(3), DataErrorType::DataEncodingError);
    assert_eq!(DataErrorType::from(4), DataErrorType::DataTableNotFound);
    assert_eq!(DataErrorType::from(5), DataErrorType::DataSendError);
    assert_eq!(DataErrorType::from(6), DataErrorType::DataChannelError);
    assert_eq!(
        DataErrorType::from(7),
        DataErrorType::DataWrongExchangeError
    );
    assert_eq!(
        DataErrorType::from(8),
        DataErrorType::DataClientNotLoggedInError
    );
    assert_eq!(DataErrorType::from(9), DataErrorType::DataStartError);
    assert_eq!(DataErrorType::from(10), DataErrorType::DataStopError);
    assert_eq!(DataErrorType::from(11), DataErrorType::DataStopAllError);
    assert_eq!(DataErrorType::from(12), DataErrorType::UnknownDataError);
}

#[test]
fn test_data_error_type_to_u8() {
    assert_eq!(u8::from(DataErrorType::UnknownDataError), 0);
    assert_eq!(u8::from(DataErrorType::DataTypeNotKnownError), 1);
    assert_eq!(u8::from(DataErrorType::DataUnavailableError), 2);
    assert_eq!(u8::from(DataErrorType::DataEncodingError), 3);
    assert_eq!(u8::from(DataErrorType::DataTableNotFound), 4);
    assert_eq!(u8::from(DataErrorType::DataSendError), 5);
    assert_eq!(u8::from(DataErrorType::DataChannelError), 6);
    assert_eq!(u8::from(DataErrorType::DataWrongExchangeError), 7);
    assert_eq!(u8::from(DataErrorType::DataClientNotLoggedInError), 8);
    assert_eq!(u8::from(DataErrorType::DataStartError), 9);
    assert_eq!(u8::from(DataErrorType::DataStopError), 10);
    assert_eq!(u8::from(DataErrorType::DataStopAllError), 11);
}

#[test]
fn test_data_error_type_display() {
    assert_eq!(
        format!("{}", DataErrorType::UnknownDataError),
        "UnknownDataError"
    );
    assert_eq!(
        format!("{}", DataErrorType::DataTypeNotKnownError),
        "DataTypeNotKnownError"
    );
    assert_eq!(
        format!("{}", DataErrorType::DataUnavailableError),
        "DataUnavailableError"
    );
    assert_eq!(
        format!("{}", DataErrorType::DataEncodingError),
        "DataEncodingError"
    );
    assert_eq!(
        format!("{}", DataErrorType::DataTableNotFound),
        "DataTableNotFound"
    );
    assert_eq!(format!("{}", DataErrorType::DataSendError), "DataSendError");
    assert_eq!(
        format!("{}", DataErrorType::DataChannelError),
        "DataChannelError"
    );
    assert_eq!(
        format!("{}", DataErrorType::DataWrongExchangeError),
        "DataWrongExchangeError"
    );
    assert_eq!(
        format!("{}", DataErrorType::DataClientNotLoggedInError),
        "DataClientNotLoggedInError"
    );
    assert_eq!(
        format!("{}", DataErrorType::DataStartError),
        "DataStartError"
    );
    assert_eq!(format!("{}", DataErrorType::DataStopError), "DataStopError");
    assert_eq!(
        format!("{}", DataErrorType::DataStopAllError),
        "DataStopAllError"
    );
}

#[test]
fn test_data_error_type_ordering() {
    assert!(DataErrorType::UnknownDataError < DataErrorType::DataTypeNotKnownError);
    assert!(DataErrorType::DataTypeNotKnownError < DataErrorType::DataUnavailableError);
    assert!(DataErrorType::DataUnavailableError < DataErrorType::DataEncodingError);
    assert!(DataErrorType::DataEncodingError < DataErrorType::DataTableNotFound);
    assert!(DataErrorType::DataTableNotFound < DataErrorType::DataSendError);
    assert!(DataErrorType::DataSendError < DataErrorType::DataChannelError);
    assert!(DataErrorType::DataChannelError < DataErrorType::DataWrongExchangeError);
    assert!(DataErrorType::DataWrongExchangeError < DataErrorType::DataClientNotLoggedInError);
    assert!(DataErrorType::DataClientNotLoggedInError < DataErrorType::DataStartError);
    assert!(DataErrorType::DataStartError < DataErrorType::DataStopError);
    assert!(DataErrorType::DataStopError < DataErrorType::DataStopAllError);
}
