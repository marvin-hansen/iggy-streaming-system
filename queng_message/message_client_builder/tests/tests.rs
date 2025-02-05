use config_manager::ConfigManager;
use iggy_test_utils::{iggy_start_config_builder, IGGY_DARWIN_AARCH64, IGGY_LINUX_X86_64};
use service_utils::ServiceUtil;

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
}