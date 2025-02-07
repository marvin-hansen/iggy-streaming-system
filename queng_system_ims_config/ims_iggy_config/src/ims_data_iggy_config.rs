use common_ims::ExchangeID;
use sdk::builder::{IggyConfig, IggyUser};

pub fn ims_data_iggy_config(exchange_id: ExchangeID) -> IggyConfig {
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
        "consumer_data".to_string(),
        1,
        true,
    )
}

pub fn ims_client_data_iggy_config(client_id: u16, exchange_id: ExchangeID) -> IggyConfig {
    IggyConfig::new(
        IggyUser::default(),
        1,
        format!("{}-data-client-{}", exchange_id, client_id),
        1,
        1,
        "data".to_string(),
        Some("localhost:8090".to_string()),
        None,
        1,
        format!("data-client-{}", client_id),
        1,
        true,
    )
}
