use common_env::EnvironmentType;
use environment_manager::EnvironmentManager;
use std::env;

#[test]
fn test_cluster_env_type() {
    // Environment access only happens in single-threaded code.
    unsafe { env::set_var("ENV", "CLUSTER") };

    let config_manager = EnvironmentManager::new();

    assert_eq!(config_manager.env_type(), EnvironmentType::CLUSTER);
}
