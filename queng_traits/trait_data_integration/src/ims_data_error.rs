use std::error::Error;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ImsDataIntegrationError {
    FailedToConnectToWebSocket(String),
    FailedToFetchSymbols(String),
    FailedToDeserializeJsonSymbols(String),
    FailedToExtractSymbolsFromResponse(String),
    FailedToValidateSymbols(String),
    FailedToStartDataStream(String),
    FailedToStopDataStream(String),
    FailedToStopAllDataStream(String),
    FailedToProcessDataMessage(String),
    FailedToSendDataMessage(String),
    FailedToSendBatchDataMessage(String),
    SymbolNotFoundError(String),
    ShutdownError(String),
    NotSupportedError(String),
    UnknownError(String),
}

impl Error for ImsDataIntegrationError {}

impl std::fmt::Display for ImsDataIntegrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImsDataIntegrationError::FailedToConnectToWebSocket(msg) => write!(
                f,
                "[ImsDataIntegrationError]: Failed to connect to web socket: {}",
                msg
            ),
            ImsDataIntegrationError::FailedToFetchSymbols(msg) => write!(
                f,
                "[ImsDataIntegrationError]: Failed to fetch symbols: {}",
                msg
            ),
            ImsDataIntegrationError::FailedToDeserializeJsonSymbols(msg) => write!(
                f,
                "[ImsDataIntegrationError]: Failed to deserialize json symbols: {}",
                msg
            ),
            ImsDataIntegrationError::FailedToExtractSymbolsFromResponse(msg) => write!(
                f,
                "[ImsDataIntegrationError]: Failed to extract symbols from response: {}",
                msg
            ),
            ImsDataIntegrationError::FailedToValidateSymbols(msg) => write!(
                f,
                "[ImsDataIntegrationError]: Failed to validate symbols: {}",
                msg
            ),
            ImsDataIntegrationError::FailedToStartDataStream(msg) => {
                write!(
                    f,
                    "[ImsDataIntegrationError]: Failed to start data stream: {}",
                    msg
                )
            }
            ImsDataIntegrationError::FailedToStopDataStream(msg) => {
                write!(
                    f,
                    "[ImsDataIntegrationError]: Failed to stop data stream: {}",
                    msg
                )
            }
            ImsDataIntegrationError::FailedToStopAllDataStream(msg) => {
                write!(
                    f,
                    "[ImsDataIntegrationError]: Failed to stop all data stream: {}",
                    msg
                )
            }
            ImsDataIntegrationError::FailedToProcessDataMessage(msg) => {
                write!(
                    f,
                    "[ImsDataIntegrationError]: Failed to process data message: {}",
                    msg
                )
            }
            ImsDataIntegrationError::FailedToSendDataMessage(msg) => {
                write!(
                    f,
                    "[ImsDataIntegrationError]: Failed to send data message: {}",
                    msg
                )
            }
            ImsDataIntegrationError::FailedToSendBatchDataMessage(msg) => {
                write!(
                    f,
                    "[ImsDataIntegrationError]: Failed to send batch data message: {}",
                    msg
                )
            }
            ImsDataIntegrationError::SymbolNotFoundError(msg) => {
                write!(f, "[ImsDataIntegrationError]: Symbol not found: {}", msg)
            }
            ImsDataIntegrationError::ShutdownError(msg) => {
                write!(f, "[ImsDataIntegrationError]: Shutdown error: {}", msg)
            }
            ImsDataIntegrationError::NotSupportedError(msg) => {
                write!(f, "[ImsDataIntegrationError]: Not supported error: {}", msg)
            }
            ImsDataIntegrationError::UnknownError(msg) => {
                write!(f, "[ImsDataIntegrationError]: Unknown error: {}", msg)
            }
        }
    }
}
