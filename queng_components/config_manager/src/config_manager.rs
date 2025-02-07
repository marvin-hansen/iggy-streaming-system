use common_config::ServiceID;
use common_env::EnvironmentType;
use common_platform::PlatformType;
use environment_manager::{EnvironmentManager, EnvironmentManagerTrait};

#[derive(Debug)]
pub struct ConfigManager {
    dbg: bool,
    /// Type of the environment (e.g., development, testing, production).
    pub(crate) env_type: EnvironmentType,
    /// Type of the platform (e.g., Linux, Windows, macOS).
    pub(crate) platform_type: PlatformType,
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

    pub fn default() -> Self {
        Self::build(false, ServiceID::Default)
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
    pub(crate) fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[ConfigManager]: {msg}");
        }
    }
}
