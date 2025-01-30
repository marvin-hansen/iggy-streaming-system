use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ImsClientError {
    Error(String) = 1,
    FailedToCreateIggyClient(String) = 2,
    FailedToConnectToIggyServer(String) = 3,
    FailedToLoginIggyUser(String) = 4,
    FailedToCreateIggyProducer(String) = 5,
    FailedToCreateIggyConsumer(String) = 6,
    FailedToStartIggyConsumer(String) = 7,
}

impl Error for ImsClientError {}

impl Display for ImsClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImsClientError::Error(msg) => write!(f, "Error: {}", msg),
            ImsClientError::FailedToCreateIggyClient(msg) => {
                write!(f, "FailedToCreateIggyClient: {}", msg)
            }
            ImsClientError::FailedToConnectToIggyServer(msg) => {
                write!(f, "FailedToConnectToIggyServer: {}", msg)
            }
            ImsClientError::FailedToLoginIggyUser(msg) => {
                write!(f, "FailedToLoginIggyUser: {}", msg)
            }
            ImsClientError::FailedToCreateIggyProducer(msg) => {
                write!(f, "FailedToCreateIggyProducer: {}", msg)
            }
            ImsClientError::FailedToCreateIggyConsumer(msg) => {
                write!(f, "FailedToCreateIggyConsumer: {}", msg)
            }
            ImsClientError::FailedToStartIggyConsumer(msg) => {
                write!(f, "FailedToStartIggyConsumer: {}", msg)
            }
        }
    }
}
