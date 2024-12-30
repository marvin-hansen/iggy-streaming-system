use common_errors::LookupError;

#[test]
fn test_lookup_error_new() {
    let error_message = String::from("An error occurred");
    let error = LookupError::new(error_message.clone());
    assert_eq!(error.0, error_message);
}

#[test]
fn test_lookup_error_fmt() {
    let error_message = String::from("An error occurred");
    let error = LookupError::new(error_message);
    let formatted_error = format!("{error}");
    assert_eq!(formatted_error, "LookupError: An error occurred");
}

#[test]
fn test_lookup_error_clone() {
    let error_message = String::from("An error occurred");
    let error = LookupError::new(error_message.clone());
    let cloned_error = error;
    assert_eq!(cloned_error.0, error_message);
}
