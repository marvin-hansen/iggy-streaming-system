use crate::fields::IGGY_HEALTH_URI;
use crate::{IGGY_DARWIN_AARCH64, IGGY_LINUX_X86_64};
use common_env::EnvironmentType;
use service_utils::{ServiceStartConfig, WaitStrategy};

/// Selects the appropriate Iggy binary based on the environment type.
///
/// # Arguments
///
/// * `env` - The environment type.
///
/// # Returns
///
/// A string slice containing the name of the Iggy binary.
///
/// # Panics
///
/// If `env` is not supported.
///
fn select_iggy_binary(env: EnvironmentType) -> &'static str {
    match env {
        EnvironmentType::LOCAL => IGGY_DARWIN_AARCH64,
        EnvironmentType::CI => IGGY_LINUX_X86_64,
        EnvironmentType::CLUSTER => IGGY_LINUX_X86_64,
        _ => panic!("Unsupported environment"),
    }
}
/// Creates a ServiceStartConfig for Iggy based on the environment type.
///
/// # Arguments
///
/// * `env` - The environment type.
///
/// # Returns
///
/// A ServiceStartConfig that can be used to start Iggy with the service_utils.
///
/// # Panics
///
/// If the environment type is not supported.
///
pub fn iggy_start_config_builder(env: EnvironmentType) -> ServiceStartConfig {
    ServiceStartConfig::builder()
        .program(select_iggy_binary(env))
        .env_vars(vec![("IGGY_SYSTEM_CACHE_ENABLED".into(), "false".into())])
        .wait_strategy(WaitStrategy::WaitForHttpHealthCheck(
            IGGY_HEALTH_URI.to_string(),
            5,
        ))
        .build()
}
