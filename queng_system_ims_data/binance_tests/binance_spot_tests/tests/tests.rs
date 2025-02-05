use common_exchange::ExchangeID;
use common_ims::{ImsIntegrationType, IntegrationConfig, IntegrationMessageConfig};
use config_manager::ConfigManager;
use iggy_test_utils::{iggy_start_config_builder, IGGY_DARWIN_AARCH64, IGGY_LINUX_X86_64};
use ims_data_client::{EventConsumer, EventConsumerError, ImsDataClient, ImsDataClientTrait};
use service_utils::{ServiceStartConfig, ServiceUtil, WaitStrategy};

const ROOT_PATH: &str = "queng_system_ims_data/binance_tests/binance_spot_tests/tests";

const PROGRAM: &str = "ims_data_service";

const BINARIES: [&str; 3] = [PROGRAM, IGGY_DARWIN_AARCH64, IGGY_LINUX_X86_64];

const EXCHANGE_ID: ExchangeID = ExchangeID::BinanceSpot;

fn get_service_start_config(health_url: String) -> ServiceStartConfig {
    ServiceStartConfig::builder()
        .program(PROGRAM)
        .wait_strategy(WaitStrategy::WaitForHttpHealthCheck(health_url, 5))
        .build()
}

//
// Bazel only test; run with
//
// bazel test //... --test_tag_filters=binance_spot_acceptance_test --test_env=ENV=LOCAL
//
// Cargo cannot execute this test, because cargo cannot copy the iggy binaries in the test environment
// set by the root path and Cargo does not stop the iggy server after the test.
// Therefore, this test is Bazel only.

#[tokio::test]
async fn test_binance_spot() {
    // ###############################################################################
    // # Start iggy messaging service
    // ###############################################################################

    dbg!("Start service util");
    let res = ServiceUtil::with_debug(ROOT_PATH, Vec::from(BINARIES)).await;
    // dbg!(&res);
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

    dbg!("Configure iggy messaging service");
    let iggy_start_config = iggy_start_config_builder(platform);

    dbg!("Start iggy messaging service");
    let result = svc_util.start_service_from_config(iggy_start_config).await;
    // dbg!(&result);
    assert!(result.is_ok());
    dbg!("✅ iggy messaging service started");

    // ###############################################################################
    // # Start BinanceSpot IMS Data integration service
    // ###############################################################################

    dbg!("Configure IMS Data service - Binance Spot");
    let uri = config_manager
        .get_data_svc_socket_addr(EXCHANGE_ID)
        .expect("Failed to get host and port for IMS Data service");

    dbg!(&format!(" IMS Data service uri: {uri}"));

    dbg!("Configure IMS Data service - Binance Spot");
    let dbgw_start_config = get_service_start_config(uri);

    dbg!("Start IMS Data service - Binance Spot");
    let result = svc_util.start_service_from_config(dbgw_start_config).await;
    // dbg!(&result);
    assert!(result.is_ok());
    dbg!("✅ IMS Data service service started");

    // ###############################################################################
    // # Start IMS Data Client and connect to IMS Data service via Iggy messaging
    // ###############################################################################

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

    // ###############################################################################
    // # Run tests
    // ###############################################################################

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

    println!("Shutdown ImsDataClient ");
    let res = client.shutdown().await;
    // dbg!(&res);
    assert!(res.is_ok());
    println!("✅ Shutdown ImsDataClient completed");
}

#[derive(Debug)]
struct PrintEventConsumer {}

impl EventConsumer for PrintEventConsumer {
    async fn consume(&self, data: Vec<u8>) -> Result<(), EventConsumerError> {
        // convert message into raw bytes
        let raw_message = data.as_slice();

        // Replace with SBE decoder
        let message = String::from_utf8_lossy(raw_message);

        // Print message to stdout
        println!("[PrintEventConsumer]: {}", message);

        Ok(())
    }
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
