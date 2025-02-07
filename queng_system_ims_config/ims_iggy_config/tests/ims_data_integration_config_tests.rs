use common_ims::{ExchangeID, ImsIntegrationType};
use ims_iggy_config::ims_data_integration_config;

#[test]
fn test_ims_data_integration_config() {
    let exchange_id = ExchangeID::BinanceSpot;
    let config = ims_data_integration_config(exchange_id);

    assert_eq!(config.integration_id(), "binancespot-data");
    assert_eq!(config.ims_integration_type(), ImsIntegrationType::Data);
    assert_eq!(config.exchange_id(), exchange_id);
    assert_eq!(config.integration_version(), 1);
}
