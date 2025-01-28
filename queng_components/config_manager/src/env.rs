use crate::ConfigManager;
use common_env::EnvironmentType;

impl ConfigManager {
    pub const fn env_type(&self) -> EnvironmentType {
        self.env_type
    }

    pub fn env_var(&self) -> (String, String) {
        self.dbg_print("env_var");
        self.dbg_print("EnvironmentType");
        self.dbg_print(self.env_type.to_string().as_str());

        match self.env_type {
            EnvironmentType::UNKNOWN => ("ENV".to_string(), "UNKNOWN".to_string()),
            EnvironmentType::LOCAL => ("ENV".to_string(), "LOCAL".to_string()),
            EnvironmentType::CLUSTER => ("ENV".to_string(), "CLUSTER".to_string()),
            EnvironmentType::CI => ("ENV".to_string(), "CI".to_string()),
        }
    }
}
