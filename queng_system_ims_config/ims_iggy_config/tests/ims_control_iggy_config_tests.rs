use common_ims::ExchangeID;
use ims_iggy_config::ims_control_iggy_config;


#[test]
fn test_ims_control_iggy_config() {
    let exchange_id = ExchangeID::BinanceSpot;
    let config = ims_control_iggy_config(exchange_id);

    let expected_stream = format!("{}-control", exchange_id).to_ascii_lowercase();
    assert_eq!(config.stream_name(), expected_stream);
    assert_eq!(config.topic_name(), "control");
    assert_eq!(config.tcp_server_addr(), Some("localhost:8090".to_string()));
    assert_eq!(config.message_consumer_name(), "consumer_control");
    assert_eq!(config.partition_id(), 1);
}