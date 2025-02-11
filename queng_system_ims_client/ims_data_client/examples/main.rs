use common_exchange::ExchangeID;
use iggy::models::messages::PolledMessage;
use ims_data_client::{ImsDataClient, ImsDataClientTrait};
use sbe_messages_client::{ClientLoginMessage, ClientLogoutMessage};
use sbe_types::MessageType;
use sdk::builder::{EventConsumer, EventConsumerError};
use std::fmt::Error;
//
// Ensure iggy is running before running this example
// i.e. run cargo r --bin iggy-server

const IGGY_URL: &str = "iggy://iggy:iggy@localhost:8090";

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    println!("Create ImsDataClient client");
    let ims_client = ImsDataClient::with_debug(
        120,
        ExchangeID::BinanceSpot,
        IGGY_URL,
        &PrintEventConsumer {},
        &PrintEventConsumer {},
    )
    .await
    .expect("Failed to create ImsDataClient");

    println!("✅ ImsDataClient started");

    println!("Login ImsDataClient ");
    let res = ims_client.login().await;
    // dbg!(&res);
    assert!(res.is_ok());
    println!("✅ Login ImsDataClient completed");

    println!("Logout ImsDataClient ");
    let res = ims_client.logout().await;
    // dbg!(&res);
    assert!(res.is_ok());
    println!("✅ Logout ImsDataClient completed");

    // wait 1 second for all messages to arrive
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    println!("Shutdown ImsDataClient ");
    let res = ims_client.shutdown().await;
    // dbg!(&res);
    assert!(res.is_ok());
    println!("✅ Shutdown ImsDataClient completed");

    Ok(())
}

#[derive(Debug)]
struct PrintEventConsumer {}

impl EventConsumer for PrintEventConsumer {
    async fn consume(&self, message: PolledMessage) -> Result<(), EventConsumerError> {
        // Extract message payload as raw bytes
        let raw_message = message.payload.as_ref();

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
