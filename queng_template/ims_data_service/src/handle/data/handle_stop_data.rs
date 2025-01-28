use crate::service::Service;
use common_errors::MessageProcessingError;
use sbe_messages_control::StopDataMessage;
use trait_data_integration::ImsDataIntegration;

impl<Integration: ImsDataIntegration + 'static> Service<Integration> {
    /// Handles a stop data message from a client. This involves verifying the message
    /// and then calling the stop_data method of the service.
    ///
    /// # Errors
    ///
    /// If the stop data message is invalid or the service is unable to stop data for
    /// the client, a MessageProcessingError is returned.
    ///
    pub(crate) async fn handle_stop_data(
        &self,
        start_data_message: &StopDataMessage,
    ) -> Result<(), MessageProcessingError> {
        self.dbg_print("handle_start_data");
        let client_id = *start_data_message.client_id();
        let data_type = start_data_message.data_type_id();
        let exchange_id = start_data_message.exchange_id();
        let symbols = Vec::from(["BTCUSD".to_string()]);

        match self
            .stop_data(client_id, data_type, exchange_id, &symbols)
            .await
        {
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
