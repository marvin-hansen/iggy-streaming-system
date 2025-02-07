use common_ims::ExchangeID;
use sdk::builder::{IggyConfig, IggyUser};

pub fn ims_control_iggy_config(exchange_id: ExchangeID) -> IggyConfig {
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
        "consumer_control".to_string(),
        1,
        true,
    )
}