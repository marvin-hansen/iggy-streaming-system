use crate::service::Service;
use common_errors::MessageProcessingError;
use common_exchange::ExchangeID;
use sbe_types::DataErrorType;
use trait_data_integration::ImsDataIntegration;

impl<Integration: ImsDataIntegration + 'static> Service<Integration> {
    pub async fn stop_all_data(
        &self,
        client_id: u16,
        exchange_id: &ExchangeID,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        self.dbg_print("stop_all_data");

        // Verify request i.e. client logged in, exchange id, and all symbols are valid
        self.dbg_print("Verify stop_all_data request ");
        let symbols = vec![];
        match self
            .verify_data_request(client_id, exchange_id, &symbols)
            .await
        {
            Ok(_) => {}
            Err((error_type, err)) => return Err((error_type, err)),
        };

        // Stop data for all symbols
        let integration = self.ims_integration().read().await;

        self.dbg_print("stop_all_trade_data");
        match integration.stop_all_trade_data().await {
            Ok(_) => {}
            Err(e) => {
                return Err((
                    DataErrorType::DataStopAllError,
                    MessageProcessingError(e.to_string()),
                ));
            }
        }

        self.dbg_print("stop_all_ohlcv_data");
        match integration.stop_all_ohlcv_data().await {
            Ok(_) => {}
            Err(e) => {
                return Err((
                    DataErrorType::DataStopAllError,
                    MessageProcessingError(e.to_string()),
                ));
            }
        }

        drop(integration);

        Ok(())
    }
}
