use std::error::Error;
use std::fmt;

/// `MessageClientConfigError` custom error type.
///
/// Contains a single String field to hold the error message.
///
/// # Fields
///
/// `String` - The error message
///
/// # Implements
///
/// `Debug` - Formatted debug output
/// `Clone` - Clone support
/// `Error` - `std::error::Error` impl
/// `Display` - Formatted display output
///
#[derive(Debug, Clone)]
pub struct MessageClientConfigError(pub String);

impl Error for MessageClientConfigError {}

impl fmt::Display for MessageClientConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MessageClientConfigError: {}", self.0)
    }
}
