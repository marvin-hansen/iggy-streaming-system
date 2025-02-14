use common_data_bar::TimeResolution;
use common_exchange::ExchangeID;
use common_ims::{ImsIntegrationType, IntegrationConfig, IntegrationMessageConfig};
use iggy::clients::consumer::ReceivedMessage;
use iggy::consumer_ext::MessageConsumer;
use iggy::error::IggyError;
use ims_data_client::ImsDataClientTrait;
use ims_data_client::*;

const IGGY_URL: &str = "iggy://iggy:iggy@localhost:8090";

#[tokio::test]
async fn test_mock_ims_data_client() {
    let client: ImsDataClientSelector = ImsDataMockClient::new(
        0,
        ims_data_integration_config(ExchangeID::NullVal),
        &PrintEventConsumer {},
        &PrintEventConsumer {},
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
        IGGY_URL.to_string(),
        exchange_id,
        IntegrationMessageConfig::new(1, 1, exchange_id),
    )
}

#[derive(Debug)]
struct PrintEventConsumer {}

impl MessageConsumer for PrintEventConsumer {
    async fn consume(&self, message: ReceivedMessage) -> Result<(), IggyError> {
        // convert message into raw bytes
        let raw_message = message.message.payload.as_ref();

        // convert into raw string
        let message = String::from_utf8_lossy(raw_message);

        // Print message to stdout
        println!("[PrintEventConsumer]: {}", message);

        Ok(())
    }
}
