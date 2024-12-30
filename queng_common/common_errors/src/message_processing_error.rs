use std::error::Error;
use std::fmt;

/// `MessageProcessingError` custom error type.
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
//
#[derive(Debug, Clone)]
pub struct MessageProcessingError(pub String);

impl MessageProcessingError {
    pub fn new(field0: String) -> Self {
        Self(field0)
    }
}

impl Error for MessageProcessingError {}

impl fmt::Display for MessageProcessingError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MessageProcessingError: {}", self.0)
    }
}
