use crate::service::Service;
use common_data_bar::TimeResolution;
use common_errors::MessageProcessingError;
use common_exchange::ExchangeID;
use sbe_types::{DataErrorType, DataType};
use std::sync::Arc;
use trait_data_integration::ImsDataIntegration;

impl<Integration: ImsDataIntegration + 'static> Service<Integration> {
    pub async fn start_data(
        &self,
        client_id: u16,
        data_type: &DataType,
        exchange_id: &ExchangeID,
        symbols: &[String],
        time_resolution: TimeResolution,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        self.dbg_print("start_data");

        // Verify request i.e. client logged in, exchange id, and all symbols are valid
        self.dbg_print("Verify start_data request");
        match self
            .verify_data_request(client_id, exchange_id, symbols)
            .await
        {
            Ok(_) => {}
            Err((error_type, err)) => return Err((error_type, err)),
        };

        // Start data for symbols
        let integration = self.ims_integration().read().await;

        match data_type {
            DataType::UnknownDataType => {
                return Err((
                    DataErrorType::DataStartError,
                    MessageProcessingError("Unknown data type".to_string()),
                ));
            }
            DataType::TradeData => {
                self.dbg_print("Start trade data for symbols");
                let processor = self
                    .get_data_producer(client_id)
                    .await
                    .expect("Failed to get data producer");

                let processor = Arc::new(processor);
                match integration.start_trade_data(symbols, &processor).await {
                    Ok(_) => {}
                    Err(e) => {
                        return Err((
                            DataErrorType::DataStartError,
                            MessageProcessingError::new(e.to_string()),
                        ))
                    }
                };
            }
            DataType::OHLCVData => {
                self.dbg_print("Start ohlcv data for symbols");
                let processor = self
                    .get_data_producer(client_id)
                    .await
                    .expect("Failed to get data producer");
                let processor = Arc::new(processor);
                match integration
                    .start_ohlcv_data(symbols, time_resolution, &processor)
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        return Err((
                            DataErrorType::DataStartError,
                            MessageProcessingError::new(e.to_string()),
                        ))
                    }
                };
            }
        }

        drop(integration);

        Ok(())
    }
}
