use std::error::Error;
use std::fmt;

/// Database gateway custom error type.
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
pub struct DBGatewayError(pub String);

impl Error for DBGatewayError {}

impl fmt::Display for DBGatewayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DBGatewayError: {}", self.0)
    }
}
