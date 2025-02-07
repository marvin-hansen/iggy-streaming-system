use crate::service::Service;
use bytes::Bytes;
use client_error_ext::SbeClientErrorExtension;
use common_errors::MessageProcessingError;
use common_sbe_errors::{ClientError, DataError};
use data_error_ext::SbeDataErrorExtension;
use sbe_types::{ClientErrorType, DataErrorType};
use sdk::builder::EventProducer;
use sdk::builder::Message as IggyMessage;
use trait_data_integration::ImsDataIntegration;

impl<Integration: ImsDataIntegration> Service<Integration> {
    /// Sends a `ClientError` message to the given producer.
    ///
    /// # Parameters
    ///
    /// * `producer` - The topic producer to send the message on
    /// * `client_id` - The id of the client the error is for
    /// * `client_error` - The `ClientErrorType` to send
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` if successful, otherwise returns a
    /// `MessageProcessingError` on failure to send.
    ///
    pub(crate) async fn send_client_error(
        &self,
        client_id: u16,
        client_error: ClientErrorType,
    ) -> Result<(), MessageProcessingError> {
        // Build error type
        let client_error = ClientError::new(client_id, client_error as u8);

        // Encode message as SBE binary
        let (_, message) = client_error
            .encode_to_sbe()
            .expect("Failed to encode client error message");

        // Send message
        self.send_error(message)
            .await
            .expect("Failed to send client error message");

        Ok(())
    }

    /// Sends a `DataError` message to the given producer.
    ///
    /// # Parameters
    ///
    /// * `producer` - The topic producer to send the message on
    /// * `client_id` - The id of the client the error is for
    /// * `data_error` - The `DataErrorType` to send
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` if successful, otherwise returns a
    /// `MessageProcessingError` on failure to send.
    ///
    pub(crate) async fn send_data_error(
        &self,
        client_id: u16,
        data_error: DataErrorType,
    ) -> Result<(), MessageProcessingError> {
        // Build error type
        let data_error = DataError::new(client_id, data_error as u8);

        // Encode message as SBE binary
        let (_, message) = data_error
            .encode_to_sbe()
            .expect("Failed to encode data error message");

        // Send message
        self.send_error(message)
            .await
            .expect("Failed to send error message");

        Ok(())
    }

    pub(crate) async fn send_error(&self, data: Vec<u8>) -> Result<(), MessageProcessingError> {
        let payload = Bytes::from(data);
        let message = IggyMessage::new(None, payload, None);

        // Send message
        self.producer()
            .read()
            .await
            .send_one_event(message)
            .await
            .expect("Failed to send error message");

        Ok(())
    }
}
