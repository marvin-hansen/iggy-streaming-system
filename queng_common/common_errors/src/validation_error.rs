use std::error::Error;
use std::fmt;

/// `ValidationError` custom error type.
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
pub struct ValidationError(pub String);

impl ValidationError {
    #[must_use]
    pub const fn new(field0: String) -> Self {
        Self(field0)
    }
}

impl Error for ValidationError {}

impl fmt::Display for ValidationError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ValidationError: {}", self.0)
    }
}
