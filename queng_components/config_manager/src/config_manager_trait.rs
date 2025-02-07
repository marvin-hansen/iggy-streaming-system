use crate::error::ConfigError;
use common_env::EnvironmentType;
use common_exchange::ExchangeID;
use common_platform::PlatformType;


pub trait ConfigManagerTrait {
    /// Return the environment type
    ///
    /// Return the current environment type.
    fn env_type(&self) -> EnvironmentType;

    /// Return the platform type
    ///
    /// Return the current platform type.
    fn platform_type(&self) -> PlatformType;

    /// Return environment variable
    ///
    /// Return the environment variable.
    ///
    /// # Returns
    ///
    /// A tuple of strings containing the environment variable name and value.
    fn env_var(&self) -> (String, String);

    /// Return the Data Service socket health URI
    ///
    /// Return the Data Service socket health URI
    /// for the given exchange ID.
    ///
    /// # Arguments
    ///
    /// * `exchange_id`: The exchange ID.
    ///
    /// # Returns
    ///
    /// A string containing the Data Service socket health URI.
    fn data_svc_socket_health_uri(
        &self,
        exchange_id: ExchangeID,
    ) -> Result<String, ConfigError>;

    /// Return the Data Service port
    ///
    /// Return the Data Service port for the given exchange ID.
    ///
    /// # Arguments
    ///
    /// * `exchange_id`: The exchange ID.
    ///
    /// # Returns
    ///
    /// A u16 containing the Data Service port.
    fn data_svc_port(&self, exchange_id: ExchangeID) -> Result<u16, ConfigError>;

    /// Return the Data Service socket address
    ///
    /// Return the Data Service socket address for the given exchange ID.
    ///
    /// # Arguments
    ///
    /// * `exchange_id`: The exchange ID.
    ///
    /// # Returns
    ///
    /// A string containing the Data Service socket address.
    fn data_svc_socket_addr(&self, exchange_id: ExchangeID) -> Result<String, ConfigError>;
}