use common_ims::ExchangeID;
use iggy::messages::poll_messages::PollingStrategy;
use iggy::utils::duration::IggyDuration;
use sdk::builder::IggyStreamConfig;
use std::str::FromStr;

pub fn ims_control_iggy_config(exchange_id: ExchangeID) -> IggyStreamConfig {
    IggyStreamConfig::new(
        format!("{}-control-stream", exchange_id).as_str(),
        format!("{}-control-topic", exchange_id).as_str(),
        100,
        IggyDuration::from_str("1ms").unwrap(),
        IggyDuration::from_str("1ms").unwrap(),
        PollingStrategy::last(),
    )
}
