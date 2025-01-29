use common_ims::{ExchangeID, IntegrationConfig};
use common_ims::{ImsIntegrationType, IntegrationMessageConfig};
use message_shared::{IggyConfig, IggyUser};

pub(crate) fn ims_data_integration_config(exchange_id: ExchangeID) -> IntegrationConfig {
    IntegrationConfig::new(
        format!("{}-data", exchange_id),
        1,
        ImsIntegrationType::Data,
        exchange_id,
        IntegrationMessageConfig::new(1, 1, exchange_id),
    )
}

pub(crate) fn ims_data_iggy_config(exchange_id: ExchangeID) -> IggyConfig {
    IggyConfig::new(
        IggyUser::default(),
        "127.0.0.1:8090",
        exchange_id as u32,
        1,
        1,
        1,
        true,
    )
}
