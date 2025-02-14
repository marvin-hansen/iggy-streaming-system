use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Debug, Clone)]
pub enum SanitizeError {
    InvalidTableName(String),
    EmptyTableName(String),
    TableNameTooLong(String),
    TableDoesNotExist(String, String),
}

impl Error for SanitizeError {}

impl fmt::Display for SanitizeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTableName(e) => write!(
                f,
                "Invalid table name provided: Only use alphanumeric characters and underscores as table name. Error: {e}"
            ),

            Self::EmptyTableName(e) => write!(
                f,
                "Empty table name provided: Table must have a name. Error: {e}"
            ),

            Self::TableNameTooLong(e) => write!(
                f,
                "Table name exceeds maximum length: Table can only be 63 characters long. Error: {e}"
            ),

            Self::TableDoesNotExist(table_name, e) => write!(
                f,
                "Table does not exist: Table {table_name} does not exist. Error: {e}"
            ),
        }
    }
}
