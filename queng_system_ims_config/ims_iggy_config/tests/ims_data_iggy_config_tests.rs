use common_ims::ExchangeID;
use iggy::messages::poll_messages::PollingStrategy;
use ims_iggy_config::ims_data_iggy_config;

#[test]
fn test_ims_data_iggy_config() {
    let exchange_id = ExchangeID::BinanceSpot;
    let client_id = 120;
    let config = ims_data_iggy_config(client_id, exchange_id);

    assert_eq!(config.stream_name(), "binancespot-data-client-120");
    assert_eq!(config.topic_name(), "binancespot-data-topic");
    assert_eq!(config.batch_size(), 100);
    assert_eq!(config.polling_strategy(), PollingStrategy::last());
}
