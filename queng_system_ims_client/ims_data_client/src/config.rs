use common_exchange::ExchangeID;
use message_shared::{IggyConfig, IggyUser};

pub(crate) fn control_stream_config(exchange_id: ExchangeID) -> IggyConfig {
    IggyConfig::new(
        IggyUser::default(),
        1,
        format!("{}-control", exchange_id),
        1,
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
