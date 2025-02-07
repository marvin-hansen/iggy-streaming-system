use crate::config_manager_trait::ConfigManagerTrait;
use crate::error::ConfigError;
use crate::ConfigManager;
use common_env::EnvironmentType;
use common_exchange::ExchangeID;
use common_platform::PlatformType;

impl ConfigManagerTrait for ConfigManager {
    fn env_type(&self) -> EnvironmentType {
        self.get_env_type()
    }

    fn platform_type(&self) -> PlatformType {
        self.get_platform_type()
    }

    fn env_var(&self) -> (String, String) {
        self.get_env_var()
    }

    fn data_svc_socket_health_uri(
        &self,
        exchange_id: ExchangeID,
    ) -> Result<String, ConfigError> {
        self.get_data_svc_socket_health_uri(exchange_id)
    }

    fn data_svc_port(&self, exchange_id: ExchangeID) -> Result<u16, ConfigError> {
        self.get_data_svc_port(exchange_id)
    }

    fn data_svc_socket_addr(&self, exchange_id: ExchangeID) -> Result<String, ConfigError> {
        self.get_data_svc_socket_addr(exchange_id)
    }
}