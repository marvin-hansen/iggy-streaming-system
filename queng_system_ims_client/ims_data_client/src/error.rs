use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct ImsClientError(pub String);

impl Error for ImsClientError {}

impl Display for ImsClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImsClientError: {}", self.0)
    }
}
