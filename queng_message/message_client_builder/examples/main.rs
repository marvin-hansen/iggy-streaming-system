use iggy::client::Client;
use iggy::locking::IggySharedMutFn;
use iggy::messages::send_messages::Message;
use message_client_builder::MessageClientBuilder;
use message_shared::{IggyConfig, IggyUser};
use std::fmt::Error;
use std::str::FromStr;
use tokio_util::sync::CancellationToken;
use trait_event_consumer::{EventConsumer, EventConsumerError};

//
// Ensure iggy is running before running this code example
// i.e. run cargo r --bin iggy-server
//
// Run this code example via:
// cargo run --example basic_message_client_builder
//

// Whether to delete topics and streams after the example run.
// If false, you have to delete these manually.
// If true, the MessageClientBuilder will re-create them during every run.
const CLEANUP: bool = false;

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    println!("📝 Build iggy client");
    let iggy_config = iggy_config();
    let (iggy_client, iggy_client_builder) = MessageClientBuilder::new(&iggy_config)
        .await
        .expect("Failed to build control stream");

    let message_producer = iggy_client_builder.iggy_producer().to_owned();
    let message_consumer = iggy_client_builder.iggy_consumer();
    println!("✅ iggy client build");

    println!("📝 Start iggy consumer");
    let token = CancellationToken::new();
    let token_consumer = token.clone();
    tokio::spawn(async move {
        match message_consumer
            .consume_messages(&PrintEventConsumer {}, token)
            .await
        {
            Ok(_) => {}
            Err(err) => {
                eprintln!("[ImsDataClient]: Failed to consume control messages: {err}");
            }
        }
    });
    println!("✅ iggy consumer started");

    println!("📝 Send a test message via producer");
    let payload = "Hello Iggy";
    let message = Message::from_str(&payload).expect("Failed to create test message");

    let res = message_producer.producer().send_one(message).await;
    //dbg!(&res);
    assert!(res.is_ok());
    println!("✅ test message send");

    // wait 5 seconds
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // you may want to check your iggy server to ensure the message arrived:
    // i.e. run iggy -u iggy -p iggy stream list

    println!("📝 Stop iggy consumer");
    token_consumer.cancel();
    println!("✅ iggy consumer stopped");

    if CLEANUP {

        // There is a known issue on the iggy server where the stream and topic deletion
        // during shutdown trigger some errors shown in the server log. However,
        // topics and streams get deleted and shutdown is successful so these errors
        // do not affect the correctness of the application.
        // When you set CLEANUP to true, you will see the errors during every shutdown call.

        println!("📝 Delete iggy topic");
        let data_stream_id = message_producer.stream_id();
        let data_topic_id = message_producer.topic_id();
        let res = iggy_client
            .client()
            .read()
            .await
            .delete_topic(data_stream_id, data_topic_id)
            .await;
        //dbg!(&res);
        assert!(res.is_ok());
        println!("✅ iggy topic deleted");

        println!("📝 Delete iggy stream");
        let res = iggy_client
            .client()
            .read()
            .await
            .delete_stream(data_stream_id)
            .await;
        //dbg!(&res);
        assert!(res.is_ok());
        println!("✅ iggy stream deleted");
    }

    println!("📝 Stop iggy client");
    let res = iggy_client.shutdown().await;
    assert!(res.is_ok());
    println!("✅ iggy client stopped");

    Ok(())
}

pub(crate) fn iggy_config() -> IggyConfig {
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
        println!("[PrintEventConsumer]");
        println!("====================");
        println!("Message received: {}", message);
        println!("====================");

        Ok(())
    }
}
