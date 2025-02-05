use config_manager::ConfigManager;
use iggy::client::Client;
use iggy::messages::send_messages::Message;
use iggy_test_utils::{iggy_start_config_builder, IGGY_DARWIN_AARCH64, IGGY_LINUX_X86_64};
use message_client_builder::*;
use message_shared::{IggyConfig, IggyUser};
use service_utils::ServiceUtil;
use std::str::FromStr;
use tokio_util::sync::CancellationToken;

const ROOT_PATH: &str = "queng_message/message_client_builder/tests";

const BINARIES: [&str; 2] = [IGGY_DARWIN_AARCH64, IGGY_LINUX_X86_64];

//
// Bazel only; run with
//
// bazel test //... --test_tag_filters=message_client_builder_test --test_env=ENV=LOCAL
//
// Cargo cannot execute this test, because cargo cannot copy the iggy binaries in the test environment
// set by the root path and Cargo does not stop the iggy server after the test.
// Therefore, this test is Bazel only.

#[tokio::test]
async fn test_binance_spot() {
    dbg!("Start service util");
    let res = ServiceUtil::with_debug(ROOT_PATH, Vec::from(BINARIES)).await;
    if res.is_err() {
        dbg!(&res);
    }
    assert!(res.is_ok());
    let svc_util = res.unwrap();
    dbg!("✅ service util started");

    dbg!("Start config manager");
    let config_manager = ConfigManager::default_with_debug();
    dbg!("✅ config manager started");

    dbg!("Detect Environment");
    let env = config_manager.env_type();
    dbg!(&format!("✅ Detected Environment: {}", env));

    let platform = config_manager.platform_type();
    dbg!(&format!("✅ Detected platform: {}", platform));

    dbg!("Start iggy messaging service");
    let iggy_start_config = iggy_start_config_builder(platform);
    let result = svc_util.start_service_from_config(iggy_start_config).await;
    if result.is_err() {
        dbg!(&result);
    }
    assert!(result.is_ok());
    dbg!("✅ iggy messaging service started");

    dbg!("Build iggy client");
    let iggy_config = iggy_config();
    let (iggy_client, iggy_client_builder) = MessageClientBuilder::with_debug(&iggy_config)
        .await
        .expect("Failed to build control stream");

    let message_producer = iggy_client_builder.iggy_producer().to_owned();
    let message_consumer = iggy_client_builder.iggy_consumer();
    dbg!("✅ iggy client build");

    dbg!("Start iggy consumer");
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
    dbg!("✅ iggy consumer started");

    dbg!("Send a test message via producer");
    let payload = "hello world";
    let message = Message::from_str(&payload).expect("Failed to create test message");

    let res = message_producer.producer().send_one(message).await;
    assert!(res.is_ok());
    dbg!("✅ test message send");

    dbg!("Stop iggy consumer");
    token_consumer.cancel();
    dbg!("✅ iggy consumer stopped");

    dbg!("Stop iggy client");
    let res = iggy_client.shutdown().await;
    assert!(res.is_ok());
    dbg!("✅ iggy client stopped");
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
        println!("[PrintEventConsumer] Message received: {}", message);

        Ok(())
    }
}
