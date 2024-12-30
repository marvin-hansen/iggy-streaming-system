use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct DownloadError(pub String);

impl From<&str> for DownloadError {
    fn from(field0: &str) -> Self {
        Self(field0.to_string())
    }
}

impl From<String> for DownloadError {
    fn from(field0: String) -> Self {
        Self(field0)
    }
}

impl Display for DownloadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "DownloadError: {}", self.0)
    }
}

impl Error for DownloadError {}
