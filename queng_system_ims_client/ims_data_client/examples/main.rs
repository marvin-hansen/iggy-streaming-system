use common_exchange::ExchangeID;
use common_ims::{ImsIntegrationType, IntegrationConfig, IntegrationMessageConfig};
use ims_data_client::{ImsDataClient, ImsDataClientTrait};
use std::fmt::Error;
use trait_event_consumer::{EventConsumer, EventConsumerError};

// Ensure iggy is running before running this example
// i.e. run cargo r --bin iggy-server


#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    println!("Create ImsDataClient client");
    let client: ImsDataClient = ImsDataClient::with_debug(
        120,
        ims_data_integration_config(ExchangeID::BinanceSpot),
        &PrintEventConsumer {},
        &PrintEventConsumer {},
    )
        .await
        .expect("Failed to create mock client")
        .into();

    println!("✅ ImsDataClient started");

    // wait 5 seconds
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    println!("Login ImsDataClient ");
    let res = client.login().await;
    assert!(res.is_ok());
    println!("✅ Login ImsDataClient completed");

    // wait 5 seconds
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    println!("Logout ImsDataClient ");
    let res = client.logout().await;
    assert!(res.is_ok());
    println!("✅ Logout ImsDataClient completed");

    // wait 5 seconds
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    println!("Shutdown ImsDataClient ");
    let res = client.shutdown().await;
    assert!(res.is_ok());
    println!("✅ Shutdown ImsDataClient completed");

    Ok(())
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
        // convert message into raw bytes
        let raw_message = data.as_slice();

        // convert into raw string
        let message = String::from_utf8_lossy(raw_message);

        // Print message to stdout
        println!("[PrintEventConsumer]: {}", message);

        Ok(())
    }
}
