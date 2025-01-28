mod env;
mod error;
mod fields;
mod svc;

use common_config::ServiceID;
use common_env::EnvironmentType;
use environment_manager::EnvironmentManager;

#[derive(Debug)]
pub struct ConfigManager {
    dbg: bool,
    /// Type of the environment (e.g., development, testing, production).
    env_type: EnvironmentType,
    /// ID of the service.
    svc: ServiceID,
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

    fn build(dbg: bool, svc: ServiceID) -> Self {
        let env_type = Self::detect_env_type(dbg);
        Self { dbg, env_type, svc }
    }

    fn detect_env_type(dbg: bool) -> EnvironmentType {
        let config_manager = if dbg {
            println!("[CfgManager]: Debug mode enabled");
            EnvironmentManager::new()
        } else {
            EnvironmentManager::with_debug()
        };

        config_manager.env_type()
    }
}

impl ConfigManager {
    pub(crate) fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[ConfigManager]: {msg}");
        }
    }
}
