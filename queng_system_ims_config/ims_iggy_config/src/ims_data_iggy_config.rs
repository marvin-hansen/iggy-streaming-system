use common_ims::ExchangeID;
use iggy::messages::poll_messages::PollingStrategy;
use iggy::utils::duration::IggyDuration;
use sdk::builder::IggyStreamConfig;
use std::str::FromStr;

pub fn ims_data_iggy_config(client_id: u16, exchange_id: ExchangeID) -> IggyStreamConfig {
    IggyStreamConfig::new(
        format!("{}-data-client-{}", exchange_id, client_id).as_str(),
        format!("{}-data-topic", exchange_id).as_str(),
        100,
        IggyDuration::from_str("1ms").unwrap(),
        IggyDuration::from_str("1ms").unwrap(),
        PollingStrategy::last(),
    )
}
