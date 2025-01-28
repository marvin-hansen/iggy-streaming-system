use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct EventProcessorError(pub String);

impl EventProcessorError {
    #[inline]
    pub const fn new(field0: String) -> Self {
        Self(field0)
    }
}

impl Error for EventProcessorError {}

impl fmt::Display for EventProcessorError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EventProcessorError: {}", self.0)
    }
}
