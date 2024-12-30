use common_errors::ValidationError;
use std::error::Error;

#[test]
fn test_validation_error_new() {
    let error_message = String::from("An error occurred");
    let validation_error = ValidationError::new(error_message.clone());
    assert_eq!(validation_error.0, error_message);
}

#[test]
fn test_validation_error_fmt() {
    let error_message = String::from("An error occurred");
    let validation_error = ValidationError::new(error_message);
    let formatted_error = format!("{validation_error}");
    assert_eq!(formatted_error, "ValidationError: An error occurred");
}

#[test]
fn test_validation_error_trait_impl() {
    let error_message = String::from("An error occurred");
    let validation_error = ValidationError::new(error_message);
    let error_trait_object: &dyn Error = &validation_error;
    assert_eq!(
        error_trait_object.to_string(),
        "ValidationError: An error occurred"
    );
}
