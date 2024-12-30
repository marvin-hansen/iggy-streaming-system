use common_errors::SanitizeError;

#[test]
fn test_display_invalid_table_name() {
    let error = SanitizeError::InvalidTableName("invalid_table!".to_string());
    let formatted_error = format!("{error}");
    assert_eq!(
        formatted_error,
        "Invalid table name provided: Only use alphanumeric characters and underscores as table name. Error: invalid_table!"
    );
}

#[test]
fn test_display_empty_table_name() {
    let error = SanitizeError::EmptyTableName(String::new());
    let formatted_error = format!("{error}");
    assert_eq!(
        formatted_error,
        "Empty table name provided: Table must have a name. Error: "
    );
}

#[test]
fn test_display_table_does_not_exist() {
    let error = SanitizeError::TableDoesNotExist(
        "nonexistent_table".to_string(),
        "error details".to_string(),
    );
    let formatted_error = format!("{error}");
    assert_eq!(
        formatted_error,
        "Table does not exist: Table nonexistent_table does not exist. Error: error details"
    );
}
