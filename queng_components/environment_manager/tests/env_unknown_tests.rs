use common_env::EnvironmentType;
use environment_manager::{EnvironmentManager, EnvironmentManagerTrait};
use std::env;

#[test]
fn test_unknown_env_type() {
    // Environment access only happens in single-threaded code.
    unsafe { env::set_var("ENV", "UNKNOWN") };

    let config_manager = EnvironmentManager::with_debug();

    assert_eq!(config_manager.env_type(), EnvironmentType::UNKNOWN);
}
