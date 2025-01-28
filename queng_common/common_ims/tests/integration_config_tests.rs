use common_exchange::ExchangeID;
use common_ims::{ImsIntegrationType, IntegrationConfig, IntegrationMessageConfig};

#[test]
fn test_new_integration_config_online_default() {
    let msg_config = IntegrationMessageConfig::new(1, 1, ExchangeID::BinanceSpot);
    let config = IntegrationConfig::new(
        "test-integration".to_string(),
        1,
        ImsIntegrationType::Data,
        ExchangeID::BinanceSpot,
        msg_config,
    );

    assert!(!config.online());
}

#[test]
fn test_integration_config_display() {
    let msg_config = IntegrationMessageConfig::new(1, 1, ExchangeID::BinanceSpot);
    let config = IntegrationConfig::new(
        "test-integration".to_string(),
        1,
        ImsIntegrationType::Data,
        ExchangeID::BinanceSpot,
        msg_config,
    );

    assert_eq!(config.to_string(), "test-integration");
}

#[test]
fn test_integration_config_getters() {
    let integration_id = "test-integration".to_string();
    let version = 2;
    let integration_type = ImsIntegrationType::Execution;
    let exchange = ExchangeID::Kraken;
    let msg_config = IntegrationMessageConfig::new(1, 1, exchange);

    let config = IntegrationConfig::new(
        integration_id.clone(),
        version,
        integration_type,
        exchange,
        msg_config.clone(),
    );

    assert_eq!(config.integration_id(), integration_id);
    assert_eq!(config.integration_version(), version);
    assert_eq!(config.ims_integration_type(), integration_type);
    assert_eq!(config.exchange_id(), exchange);
    assert_eq!(config.integration_message_config(), &msg_config);
}
