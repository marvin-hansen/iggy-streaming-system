use common_exchange::ExchangeID;
use common_ims::{ImsIntegrationType, IntegrationConfig, IntegrationMessageConfig};
use ims_data_client::{ImsDataClient, ImsDataClientTrait};
use sbe_messages_client::{ClientLoginMessage, ClientLogoutMessage};
use sbe_types::MessageType;
use sdk::builder::{EventConsumer, EventConsumerError};
use std::fmt::Error;
//
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
    .expect("Failed to create ImsDataClient");

    println!("✅ ImsDataClient started");

    println!("Login ImsDataClient ");
    let res = client.login().await;
    // dbg!(&res);
    assert!(res.is_ok());
    println!("✅ Login ImsDataClient completed");

    println!("Logout ImsDataClient ");
    let res = client.logout().await;
    // dbg!(&res);
    assert!(res.is_ok());
    println!("✅ Logout ImsDataClient completed");

    // wait 1 second
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    println!("Shutdown ImsDataClient ");
    let res = client.shutdown().await;
    // dbg!(&res);
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

        // determine message type
        let message_type = MessageType::from(u16::from(raw_message[2]));

        // decode SBE message and convert to string
        let msg_string = match message_type {
            MessageType::ClientLogin => {
                let client_login_msg = ClientLoginMessage::from(raw_message);
                client_login_msg.to_string()
            }
            MessageType::ClientLogout => {
                let client_logout_msg = ClientLogoutMessage::from(raw_message);
                client_logout_msg.to_string()
            }
            _ => "Unknown message type".to_string(),
        };

        // Print message to stdout
        println!("###################");
        println!("Message received: ");
        println!("{msg_string}");
        println!("###################");

        Ok(())
    }
}
