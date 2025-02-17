use common_ims::ExchangeID;
use iggy::stream_builder::IggyStreamConfig;
use iggy::utils::duration::IggyDuration;
use std::str::FromStr;

pub fn ims_control_iggy_config(exchange_id: ExchangeID) -> IggyStreamConfig {
    IggyStreamConfig::from_stream_topic(
        format!("{}-control-stream", exchange_id).as_str(),
        format!("{}-control-topic", exchange_id).as_str(),
        100,
        IggyDuration::from_str("1ms").unwrap(),
        IggyDuration::from_str("1ms").unwrap(),
    )
    .expect("Failed to create iggy config")
}
