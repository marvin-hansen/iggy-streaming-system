use iggy::client::Client;
use iggy::messages::send_messages::Message;
use sdk::builder::config::IggyConfig;
use sdk::builder::{EventConsumer, EventConsumerError, IggyBuilder, IggyUser};
use std::str::FromStr;
use tokio_util::sync::CancellationToken;

//
// Make sure iggy is running on 127.0.0.1:8090
//

#[tokio::test]
async fn test_iggy_builder() {
    println!("Build iggy client");
    let iggy_config = iggy_config();

    let (iggy_client, iggy_client_builder) = IggyBuilder::from_config(&iggy_config)
        .await
        .expect("Failed to build control IggyBuilder");
    let message_producer = iggy_client_builder.iggy_producer().to_owned();
    let message_consumer = iggy_client_builder.iggy_consumer();
    println!("✅ iggy client build");
    println!("✅ iggy consumer build");
    println!("✅ iggy producer build");

    println!("Start iggy consumer");
    let token = CancellationToken::new();
    let token_consumer = token.clone();
    tokio::spawn(async move {
        match message_consumer
            .consume_messages(&PrintEventConsumer {}, token)
            .await
        {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Failed to consume messages: {err}");
            }
        }
    });
    println!("✅ iggy consumer started");

    println!("Send a test message via producer");
    let payload = "hello world";
    let message = Message::from_str(payload).expect("Failed to create test message");

    let res = message_producer.producer().send_one(message).await;
    assert!(res.is_ok());
    println!("✅ test message send");

    println!("Stop iggy consumer");
    token_consumer.cancel();
    println!("✅ iggy consumer stopped");

    println!("Stop iggy client");
    let res = iggy_client.shutdown().await;
    assert!(res.is_ok());
    println!("✅ iggy client stopped");
}

fn iggy_config() -> IggyConfig {
    IggyConfig::new(
        IggyUser::default(),
        42,
        "stream_42".to_string(),
        1,
        23,
        "topic_23".to_string(),
        Some("localhost:8090".to_string()),
        None, // No TLS config for this test
        1,
        "consumer_data".to_string(),
        1,
        true,
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
        println!("[PrintEventConsumer] Message received: {}", message);

        Ok(())
    }
}
