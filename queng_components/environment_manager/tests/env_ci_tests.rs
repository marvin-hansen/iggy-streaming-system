use common_env::EnvironmentType;
use environment_manager::{EnvironmentManager, EnvironmentManagerTrait};
use std::env;

#[test]
fn test_ci_env_type() {
    // Environment access only happens in single-threaded code.
    unsafe { env::set_var("ENV", "CI") };

    let config_manager = EnvironmentManager::with_debug();

    assert_eq!(config_manager.env_type(), EnvironmentType::CI);
}
