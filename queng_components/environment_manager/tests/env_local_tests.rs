use common_env::EnvironmentType;
use environment_manager::EnvironmentManager;
use std::env;

#[test]
fn test_local_env_type() {
    // Environment access only happens in single-threaded code.
    unsafe { env::set_var("ENV", "LOCAL") };

    let config_manager = EnvironmentManager::with_debug();

    assert_eq!(config_manager.env_type(), EnvironmentType::LOCAL);
}
