use crate::service::ImsDataService;
use common_errors::MessageProcessingError;
use std::collections::HashSet;
use trait_data_integration::ImsDataIntegration;

impl<Integration> ImsDataService<Integration>
where
    Integration: ImsDataIntegration,
{
    pub async fn get_exchange_symbols(&self) -> Result<HashSet<String>, MessageProcessingError> {
        self.intersection.get_exchange_symbols().await
    }

    pub async fn validate_symbols(
        &self,
        symbols: &[String],
    ) -> Result<bool, MessageProcessingError> {
        self.intersection.validate_symbols(symbols).await
    }
}
