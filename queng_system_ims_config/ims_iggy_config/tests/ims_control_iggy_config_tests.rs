use common_ims::ExchangeID;
use iggy::messages::poll_messages::PollingStrategy;
use ims_iggy_config::ims_control_iggy_config;

#[test]
fn test_ims_control_iggy_config() {
    let exchange_id = ExchangeID::BinanceSpot;
    let config = ims_control_iggy_config(exchange_id);

    assert_eq!(config.stream_name(), "binancespot-control-stream");
    assert_eq!(config.topic_name(), "binancespot-control-topic");
    assert_eq!(config.batch_size(), 100);
    assert_eq!(config.polling_strategy(), PollingStrategy::last());
}
