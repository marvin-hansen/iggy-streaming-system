use crate::integration_message_config::IntegrationMessageConfig;
use crate::ImsIntegrationType;
use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IntegrationConfig {
    integration_id: String,
    integration_version: u16,
    ims_integration_type: ImsIntegrationType,
    online: bool,
    integration_message_config: IntegrationMessageConfig,
}

impl IntegrationConfig {
    /// Creates a new `IntegrationConfig` with the given parameters.
    ///
    /// # Parameters
    ///
    /// * `integration_id`: The unique identifier for this integration.
    /// * `integration_version`: The version of this integration.
    /// * `ims_integration_type`: The type of integration this is.
    /// * `online`: Whether this integration is currently online.
    /// * `integration_message_config`: The configuration for the messages sent by this integration.
    ///
    /// # Returns
    ///
    /// A new `IntegrationConfig` with the given parameters.
    #[must_use]
    pub const fn new(
        integration_id: String,
        integration_version: u16,
        ims_integration_type: ImsIntegrationType,
        integration_message_config: IntegrationMessageConfig,
    ) -> Self {
        Self {
            integration_id,
            integration_version,
            ims_integration_type,
            online: false,
            integration_message_config,
        }
    }

    /// Creates a new `IntegrationConfig` with the given parameters.
    ///
    /// # Parameters
    ///
    /// * `integration_id`: The unique identifier for this integration.
    /// * `integration_version`: The version of this integration.
    /// * `ims_integration_type`: The type of integration this is.
    /// * `online`: Whether this integration is currently online.
    /// * `exchange_id`: The exchange with which this integration is associated.
    /// * `integration_message_config`: The configuration for the messages sent by this integration.
    ///
    /// # Returns
    ///
    /// A new `IntegrationConfig` with the given parameters.
    #[must_use]
    pub const fn from(
        integration_id: String,
        integration_version: u16,
        ims_integration_type: ImsIntegrationType,
        online: bool,
        integration_message_config: IntegrationMessageConfig,
    ) -> Self {
        Self {
            integration_id,
            integration_version,
            ims_integration_type,
            online,
            integration_message_config,
        }
    }
}

impl IntegrationConfig {
    pub fn set_online(&mut self) {
        self.online = true;
    }

    pub fn set_offline(&mut self) {
        self.online = false;
    }
}

impl IntegrationConfig {
    /// Returns the unique identifier for this integration.
    ///
    /// # Returns
    ///
    /// The unique identifier associated with this configuration.
    #[must_use]
    pub fn integration_id(&self) -> &str {
        &self.integration_id
    }

    /// Returns the version of this integration.
    ///
    /// # Returns
    ///
    /// The version associated with this configuration.
    #[must_use]
    pub const fn integration_version(&self) -> u16 {
        self.integration_version
    }

    /// Returns the type of integration represented by this configuration.
    ///
    /// # Returns
    ///
    /// The `ImsIntegrationType` associated with this configuration.
    #[must_use]
    pub const fn ims_integration_type(&self) -> ImsIntegrationType {
        self.ims_integration_type
    }

    /// Returns whether this integration is currently online.
    ///
    /// # Returns
    ///
    /// `true` if the integration is online, `false` otherwise.
    #[must_use]
    pub const fn online(&self) -> bool {
        self.online
    }

    /// Returns a reference to the `IntegrationMessageConfig` associated with this configuration.
    #[must_use]
    pub const fn integration_message_config(&self) -> &IntegrationMessageConfig {
        &self.integration_message_config
    }

    /// Generates a channel name for the control channel based on the integration_id.
    ///
    /// # Returns
    ///
    /// A String in the format: "{integration_id}-control".
    ///
    pub fn control_channel(&self) -> String {
        format!("{}-{}", self.integration_id, "control")
    }

    /// Generates a channel name for the data channel based on the integration_id.
    ///
    /// # Returns
    ///
    /// A String in the format: "{integration_id}-data".
    ///
    pub fn data_channel(&self) -> String {
        format!("{}-{}", self.integration_id, "data")
    }

    /// Generates a channel name for the error channel based on the integration_id.
    ///
    /// # Returns
    ///
    /// A String in the format: "{integration_id}-error".
    pub fn error_channel(&self) -> String {
        format!("{}-{}", self.integration_id, "error")
    }

    /// Generates a channel name for the execution channel based on the integration_id.
    ///
    /// # Returns
    ///
    /// A String in the format: "{integration_id}-execution".
    pub fn execution_channel(&self) -> String {
        format!("{}-{}", self.integration_id, "execution")
    }
}

impl Display for IntegrationConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.integration_id)
    }
}
