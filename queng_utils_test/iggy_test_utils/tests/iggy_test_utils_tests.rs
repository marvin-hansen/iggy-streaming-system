use common_env::EnvironmentType;
use iggy_test_utils::*;
use service_utils::WaitStrategy;

#[test]
fn test_iggy_start_config_builder_local() {
    let env = EnvironmentType::LOCAL;
    let config = iggy_start_config_builder(env);

    assert_eq!(config.program(), IGGY_DARWIN_AARCH64);
    assert!(config.env_vars().is_some());
    assert_eq!(
        config.env_vars().as_deref(),
        Some(&vec![("IGGY_SYSTEM_CACHE_ENABLED".into(), "false".into())]).map(|v| &**v)
    );

    if let WaitStrategy::WaitForHttpHealthCheck(uri, attempts) = config.wait_strategy() {
        assert_eq!(uri, IGGY_HEALTH_URI);
        assert_eq!(*attempts, 5);
    } else {
        panic!("Expected WaitForHttpHealthCheck strategy");
    }
}

#[test]
fn test_iggy_start_config_builder_ci() {
    let env = EnvironmentType::CI;
    let config = iggy_start_config_builder(env);

    assert_eq!(config.program(), IGGY_LINUX_X86_64);
    assert_eq!(
        config.env_vars().as_deref(),
        Some(&vec![("IGGY_SYSTEM_CACHE_ENABLED".into(), "false".into())]).map(|v| &**v)
    );

    if let WaitStrategy::WaitForHttpHealthCheck(uri, attempts) = config.wait_strategy() {
        assert_eq!(uri, IGGY_HEALTH_URI);
        assert_eq!(*attempts, 5);
    } else {
        panic!("Expected WaitForHttpHealthCheck strategy");
    }
}

#[test]
fn test_iggy_start_config_builder_cluster() {
    let env = EnvironmentType::CLUSTER;
    let config = iggy_start_config_builder(env);

    assert_eq!(config.program(), IGGY_LINUX_X86_64);
    assert_eq!(
        config.env_vars().as_deref(),
        Some(&vec![("IGGY_SYSTEM_CACHE_ENABLED".into(), "false".into())]).map(|v| &**v)
    );

    if let WaitStrategy::WaitForHttpHealthCheck(uri, attempts) = config.wait_strategy() {
        assert_eq!(uri, IGGY_HEALTH_URI);
        assert_eq!(*attempts, 5);
    } else {
        panic!("Expected WaitForHttpHealthCheck strategy");
    }
}
