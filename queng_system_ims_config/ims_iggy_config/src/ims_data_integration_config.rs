use common_ims::{ExchangeID, IntegrationConfig};
use common_ims::{ImsIntegrationType, IntegrationMessageConfig};
pub fn ims_data_integration_config(exchange_id: ExchangeID) -> IntegrationConfig {
    IntegrationConfig::new(
        format!("{}-data", exchange_id),
        1,
        ImsIntegrationType::Data,
        exchange_id,
        IntegrationMessageConfig::new(1, 1, exchange_id),
    )
}

