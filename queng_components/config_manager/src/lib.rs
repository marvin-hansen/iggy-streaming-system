mod env;
mod error;
mod fields;
mod svc;

use common_config::ServiceID;
use common_env::EnvironmentType;
use common_platform::PlatformType;
use environment_manager::EnvironmentManager;

#[derive(Debug)]
pub struct ConfigManager {
    dbg: bool,
    /// Type of the environment (e.g., development, testing, production).
    env_type: EnvironmentType,
    /// Type of the platform (e.g., Linux, Windows, macOS).
    platform_type: PlatformType,
}

impl ConfigManager {
    pub fn new(svc: ServiceID) -> Self {
        Self::build(false, svc)
    }

    pub fn with_debug(svc: ServiceID) -> Self {
        Self::build(true, svc)
    }

    pub fn default_with_debug() -> Self {
        Self::build(true, ServiceID::Default)
    }

    fn build(dbg: bool, _svc: ServiceID) -> Self {
        let config_manager = if dbg {
            println!("[CfgManager]: Debug mode enabled");
            EnvironmentManager::new()
        } else {
            EnvironmentManager::with_debug()
        };

        let env_type = config_manager.env_type();
        let platform_type = config_manager.platform_type();
        Self {
            dbg,
            env_type,
            platform_type,
        }
    }
}

impl ConfigManager {
    pub const fn env_type(&self) -> EnvironmentType {
        self.env_type
    }

    pub const fn platform_type(&self) -> PlatformType {
        self.platform_type
    }
}

impl ConfigManager {
    pub(crate) fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[ConfigManager]: {msg}");
        }
    }
}
