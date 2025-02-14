use common_ims::{ExchangeID, IntegrationConfig};
use common_ims::{ImsIntegrationType, IntegrationMessageConfig};

const IGGY_URL: &str = "iggy://iggy:iggy@localhost:8090";

pub fn ims_data_integration_config(exchange_id: ExchangeID) -> IntegrationConfig {
    IntegrationConfig::new(
        format!("{}-data", exchange_id),
        1,
        ImsIntegrationType::Data,
        IGGY_URL.to_string(),
        exchange_id,
        IntegrationMessageConfig::new(1, 1, exchange_id),
    )
}
