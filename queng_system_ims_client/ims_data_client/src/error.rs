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
    FailedToCreateIggyStream(String) = 5,
    FailedToCreateIggyProducer(String) = 6,
    FailedToCreateIggyConsumer(String) = 7,
    FailedToStartIggyConsumer(String) = 8,
    FailedToEncodeControlMessage(String) = 9,
    FailedToSendControlMessageToIggyServer(String) = 10,
    FailedToDeleteIggyStream(String) = 11,
    FailedToDeleteIggyTopic(String) = 12,
    FailedToShutdownIggyConsumer(String) = 13,
    FailedToShutdownIggyClient(String) = 14,
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
            ImsClientError::FailedToCreateIggyStream(msg) => {
                write!(f, "FailedToCreateIggyStream: {}", msg)
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
            ImsClientError::FailedToEncodeControlMessage(msg) => {
                write!(f, "FailedToEncodeControlMessage: {}", msg)
            }
            ImsClientError::FailedToSendControlMessageToIggyServer(msg) => {
                write!(f, "FailedToSendControlMessageToIggyServer: {}", msg)
            }
            ImsClientError::FailedToDeleteIggyStream(msg) => {
                write!(f, "FailedToDeleteIggyStream: {}", msg)
            }
            ImsClientError::FailedToDeleteIggyTopic(msg) => {
                write!(f, "FailedToDeleteIggyTopic: {}", msg)
            }
            ImsClientError::FailedToShutdownIggyConsumer(msg) => {
                write!(f, "FailedToShutdownIggyConsumer: {}", msg)
            }
            ImsClientError::FailedToShutdownIggyClient(msg) => {
                write!(f, "FailedToShutdownIggyClient: {}", msg)
            }
        }
    }
}
