use std::error::Error;
use std::fmt;

/// `InitError` custom error type.
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
pub struct InitError(pub String);

impl InitError {
    #[must_use]
    pub const fn new(field0: String) -> Self {
        Self(field0)
    }
}

impl Error for InitError {}

impl fmt::Display for InitError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InitError: {}", self.0)
    }
}
