use common_exchange::ExchangeID;
use config_manager::{ConfigManager, ConfigManagerTrait};
use iggy_test_utils::{IGGY_DARWIN_AARCH64, IGGY_LINUX_X86_64, iggy_start_config_builder};
use service_utils::{ServiceStartConfig, ServiceUtil, WaitStrategy};

const ROOT_PATH: &str = "queng_system_ims_data/binance_tests/binance_usd_future_tests/tests";

const PROGRAM: &str = "ims_data_service";

const BINARIES: [&str; 3] = [PROGRAM, IGGY_DARWIN_AARCH64, IGGY_LINUX_X86_64];

const EXCHANGE_ID: ExchangeID = ExchangeID::BinanceUsdMarginFuture;

fn get_service_start_config(health_url: String) -> ServiceStartConfig {
    ServiceStartConfig::builder()
        .program(PROGRAM)
        .wait_strategy(WaitStrategy::WaitForHttpHealthCheck(health_url, 5))
        .build()
}

//
// Bazel only test; run with
//
// bazel test //... --test_tag_filters=binance_usd_future_test --test_env=ENV=LOCAL
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

    dbg!("Configure iggy messaging service");
    let iggy_start_config = iggy_start_config_builder(platform);

    dbg!("Start iggy messaging service");
    let result = svc_util.start_service_from_config(iggy_start_config).await;
    if result.is_err() {
        dbg!(&result);
    }
    assert!(result.is_ok());
    dbg!("✅ iggy messaging service started");

    dbg!("Configure IMS Data service - Binance Spot");
    let uri = config_manager
        .data_svc_socket_addr(EXCHANGE_ID)
        .expect("Failed to get host and port for IMS Data service");

    dbg!(&format!(" IMS Data service uri: {uri}"));

    dbg!("Configure IMS Data service - Binance Spot");
    let dbgw_start_config = get_service_start_config(uri);

    dbg!("Start IMS Data service - Binance Spot");
    let result = svc_util.start_service_from_config(dbgw_start_config).await;
    if result.is_err() {
        dbg!(&result);
    }
    assert!(result.is_ok());
    dbg!("✅ IMS Data service service started");
}
