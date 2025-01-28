use crate::service::Service;
use common_errors::MessageProcessingError;
use sbe_messages_control::StopAllDataMessage;
use trait_data_integration::ImsDataIntegration;

impl<Integration: ImsDataIntegration + 'static> Service<Integration> {
    /// Handles a stop all data message from a client. This involves verifying the message
    /// and then calling the stop_all_data method of the service.
    ///
    /// # Errors
    ///
    /// If the stop all data message is invalid or the service is unable to stop all data for
    /// the client, a MessageProcessingError is returned.
    ///
    pub(crate) async fn handle_stop_all_data(
        &self,
        start_data_message: &StopAllDataMessage,
    ) -> Result<(), MessageProcessingError> {
        self.dbg_print("handle_start_data");
        let client_id = *start_data_message.client_id();
        let exchange_id = start_data_message.exchange_id();

        match self.stop_all_data(client_id, exchange_id).await {
            Ok(_) => {}
            Err((error_type, err)) => {
                // Print error
                println!("[handle_stop_all_data]: StopAllDataError: {}", err);

                // Send error message to client
                match self.send_data_error(client_id, error_type).await {
                    Ok(_) => {}
                    Err(e) => {
                        println!("[handle_stop_all_data]: SendDataError: {}", e);
                        return Err(e);
                    }
                }
            }
        }

        Ok(())
    }
}
