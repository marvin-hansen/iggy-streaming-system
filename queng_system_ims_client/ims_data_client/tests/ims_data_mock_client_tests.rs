use common_data_bar::TimeResolution;
use common_exchange::ExchangeID;
use common_ims::{ImsIntegrationType, IntegrationConfig, IntegrationMessageConfig};
use ims_data_client::*;
use trait_event_consumer::{EventConsumer, EventConsumerError};

#[tokio::test]
async fn test_mock_ims_data_client() {
    let (_, shutdown_rx) = tokio::sync::oneshot::channel();

    let client: ImsDataClientSelector = ImsDataMockClient::new(
        0,
        ims_data_integration_config(ExchangeID::NullVal),
        &PrintEventConsumer {},
        shutdown_rx,
    )
        .await
        .expect("Failed to create mock client")
        .into();

    let res = client.login().await;
    assert!(res.is_ok());

    let res = client.start_trade_data("BTC-USDT".into()).await;
    assert!(res.is_ok());

    let res = client.stop_trade_data("BTC-USDT".into()).await;
    assert!(res.is_ok());

    let res = client
        .start_ohlcv_data("BTC-USDT".into(), TimeResolution::OneMin)
        .await;
    assert!(res.is_ok());

    let res = client.stop_ohlcv_data("BTC-USDT".into()).await;
    assert!(res.is_ok());

    let res = client.logout().await;
    assert!(res.is_ok());

    client.shutdown().await.expect("Failed to shutdown client");
}

pub fn ims_data_integration_config(exchange_id: ExchangeID) -> IntegrationConfig {
    IntegrationConfig::new(
        format!("{}-data", exchange_id),
        1,
        ImsIntegrationType::Data,
        exchange_id,
        IntegrationMessageConfig::new(1, 1, exchange_id),
    )
}

#[derive(Debug)]
struct PrintEventConsumer {}

impl EventConsumer for PrintEventConsumer {
    async fn consume(&self, data: Vec<u8>) -> Result<(), EventConsumerError> {
        let raw_message = data.as_slice();

        // convert into raw string and print
        let message = String::from_utf8_lossy(raw_message);
        println!("{}", message);

        Ok(())
    }
}
