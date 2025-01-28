use crate::service::Service;
use common_errors::MessageProcessingError;
use common_exchange::ExchangeID;
use sbe_types::DataErrorType;
use trait_data_integration::ImsDataIntegration;

impl<Integration: ImsDataIntegration> Service<Integration> {
    pub async fn verify_data_request(
        &self,
        client_id: u16,
        exchange_id: &ExchangeID,
        symbols: &[String],
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        self.dbg_print(&format!(
            "Checking if client with id {} is logged in",
            client_id
        ));
        let exists = match self.check_client_login(client_id).await {
            Ok(exists) => exists,
            Err(e) => return Err((DataErrorType::UnknownDataError, e)),
        };

        if !exists {
            return Err((
                DataErrorType::DataClientNotLoggedInError,
                MessageProcessingError(format!("Client with id {} is not logged in", client_id)),
            ));
        }

        self.dbg_print(&format!(
            "Checking if Exchange with id {} is correct",
            exchange_id
        ));

        match self.verify_exchange_id(exchange_id) {
            true => {}
            false => {
                return Err(
                    (
                        DataErrorType::DataWrongExchangeError,
                        MessageProcessingError(format!(
                            "Client with id {} has requested to stop data from the wrong exchange. \
                    The client requested exchange id: {}; however this service connects to correct exchange: {}",
                            client_id,
                            exchange_id,
                            self.exchange_id()
                        ))
                    ));
            }
        };

        match self.verify_symbols(symbols).await {
            Ok(_) => {}
            Err(e) => return Err((DataErrorType::UnknownDataError, e)),
        }

        Ok(())
    }

    pub(crate) fn verify_exchange_id(&self, exchange_id: &ExchangeID) -> bool {
        self.exchange_id() == *exchange_id
    }

    pub(crate) async fn verify_symbols(
        &self,
        symbols: &[String],
    ) -> Result<(), MessageProcessingError> {
        if symbols.is_empty() {
            return Ok(());
        }

        let integration = self.ims_integration().read().await;

        match integration.validate_symbols(symbols).await {
            Ok(_) => Ok(()),
            Err(e) => Err(MessageProcessingError::new(e.to_string())),
        }
    }
}
