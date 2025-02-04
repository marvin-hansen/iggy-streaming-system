use std::error::Error;
use std::fmt::{Display, Formatter};

#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MessageClientBuilderError {
    FailedToCreateIggyClient(String),
    FailedToConnectToIggyServer(String),
    FailedToLoginIggyUser(String),
    FailedToCreateIggyStream(String),
    FailedToCreateIggyProducer(String),
    FailedToCreateIggyConsumer(String),
}

impl Error for MessageClientBuilderError {}

impl Display for MessageClientBuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageClientBuilderError::FailedToCreateIggyClient(msg) => {
                write!(
                    f,
                    "[MessageClientBuilder]: Failed to create iggy client: {}",
                    msg
                )
            }
            MessageClientBuilderError::FailedToConnectToIggyServer(msg) => {
                write!(
                    f,
                    "[MessageClientBuilder]: Failed to connect to iggy server: {}",
                    msg
                )
            }
            MessageClientBuilderError::FailedToLoginIggyUser(msg) => {
                write!(
                    f,
                    "[MessageClientBuilder]: Failed to login iggy user: {}",
                    msg
                )
            }
            MessageClientBuilderError::FailedToCreateIggyStream(msg) => {
                write!(
                    f,
                    "[MessageClientBuilder]: Failed to create iggy stream: {}",
                    msg
                )
            }
            MessageClientBuilderError::FailedToCreateIggyProducer(msg) => {
                write!(
                    f,
                    "[MessageClientBuilder]: Failed to create iggy producer: {}",
                    msg
                )
            }
            MessageClientBuilderError::FailedToCreateIggyConsumer(msg) => {
                write!(
                    f,
                    "[MessageClientBuilder]: Failed to create iggy consumer: {}",
                    msg
                )
            }
        }
    }
}
