use crate::error::ConfigError;
use crate::fields::{DEFAULT_HOST, DEFAULT_PORT};
use crate::ConfigManager;
use common_env::EnvironmentType;
use common_exchange::ExchangeID;

impl ConfigManager {
    pub fn get_data_svc_socket_health_uri(
        &self,
        exchange_id: ExchangeID,
    ) -> Result<String, ConfigError> {
        let uri = format!(
            "http://{}/health",
            self.get_data_svc_socket_addr(exchange_id)?
        );
        Ok(uri)
    }

    pub fn get_data_svc_port(&self, exchange_id: ExchangeID) -> Result<u16, ConfigError> {
        self.get_port(&exchange_id)
    }

    pub fn get_data_svc_socket_addr(&self, exchange_id: ExchangeID) -> Result<String, ConfigError> {
        // Set host to default (0.0.0.0) to listen on all interfaces
        let host = DEFAULT_HOST;

        // Adjust the port relative to the environment.
        let port = self
            .get_port(&exchange_id)
            .expect("Failed to get port from config");

        // Merge the host and port into a socket address i.e. 0.0.0.0:7070
        let socket_addr = format!("{host}:{port}");

        Ok(socket_addr)
    }

    // Adjust the port relative to the environment.
    // For local and CI environments, the port is shifted by the exchange id
    // to prevent ports from clashing.
    fn get_port(&self, exchange_id: &ExchangeID) -> Result<u16, ConfigError> {
        match self.env_type {
            EnvironmentType::LOCAL => Ok(DEFAULT_PORT + exchange_id.as_u16()),
            EnvironmentType::CLUSTER => Ok(DEFAULT_PORT),
            EnvironmentType::CI => Ok(DEFAULT_PORT + exchange_id.as_u16()),
            EnvironmentType::UNKNOWN => Err(ConfigError::new("Unknown Environment".to_string())),
        }
    }
}
