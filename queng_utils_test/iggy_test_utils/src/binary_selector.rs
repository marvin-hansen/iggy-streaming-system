use common_platform::PlatformType;
use service_utils::{ServiceStartConfig, WaitStrategy};

pub const IGGY_DARWIN_AARCH64: &str = "iggy_server_darwin_aarch64";
pub const IGGY_LINUX_AARCH64: &str = "iggy_server_linux_aarch64";
pub const IGGY_LINUX_X86_64: &str = "iggy_server_linux_x86_64";

pub const IGGY_HEALTH_URI : &str = "http://127.0.0.1:3000";

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
fn select_iggy_binary(platform: PlatformType) -> &'static str {
    match platform {
        PlatformType::LinuxX86_64 => IGGY_LINUX_X86_64,
        PlatformType::LinuxAarch64 => IGGY_LINUX_AARCH64,
        PlatformType::MacOSAarch64 => IGGY_DARWIN_AARCH64,
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
pub fn iggy_start_config_builder(platform: PlatformType) -> ServiceStartConfig {
    ServiceStartConfig::builder()
        .program(select_iggy_binary(platform))
        .env_vars(vec![("IGGY_SYSTEM_CACHE_ENABLED".into(), "false".into())])
        .wait_strategy(WaitStrategy::WaitForHttpHealthCheck(
            IGGY_HEALTH_URI.to_string(),
            5,
        ))
        .build()
}
