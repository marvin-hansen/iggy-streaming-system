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
        1,
        format!("{}-data", exchange_id),
        exchange_id as u32,
        1,
        "data".to_string(),
        Some("localhost:8090".to_string()),
        None,
        1,
        1,
        true,
    )
}

pub(crate) fn ims_control_iggy_config(exchange_id: ExchangeID) -> IggyConfig {
    IggyConfig::new(
        IggyUser::default(),
        1,
        format!("{}-control", exchange_id),
        exchange_id as u32,
        1,
        "control".to_string(),
        Some("localhost:8090".to_string()),
        None,
        1,
        1,
        true,
    )
}