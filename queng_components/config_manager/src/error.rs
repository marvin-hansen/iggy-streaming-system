use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ConfigError(pub String);

impl ConfigError {
    pub const fn new(field0: String) -> Self {
        Self(field0)
    }
}

impl Error for ConfigError {}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ConfigError: {}", self.0)
    }
}
