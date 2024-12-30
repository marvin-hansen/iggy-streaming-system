use std::error::Error;
use std::fmt;

/// `LookupError` custom error type.
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
pub struct LookupError(pub String);

impl LookupError {
    #[must_use]
    pub const fn new(field0: String) -> Self {
        Self(field0)
    }
}

impl Error for LookupError {}

impl fmt::Display for LookupError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LookupError: {}", self.0)
    }
}
