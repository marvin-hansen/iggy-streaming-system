use common_exchange::ExchangeID;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IntegrationMessageConfig {
    id: u16,
    name: String,
    version: u16,
    exchange_id: ExchangeID,
}

const NAME: &str = "integration";

impl IntegrationMessageConfig {
    #[must_use]
    pub fn new(id: u16, version: u16, exchange_id: ExchangeID) -> Self {
        let name = format!("{exchange_id}-{NAME}-{id}");

        Self {
            id,
            name,
            version,
            exchange_id,
        }
    }
}
impl IntegrationMessageConfig {
    /// Returns the client id.
    ///
    /// # Returns
    ///
    /// A u16 representing the client id.
    #[must_use]
    pub const fn id(&self) -> u16 {
        self.id
    }

    /// Returns the name of the client.
    ///
    /// # Returns
    ///
    /// A string slice containing the name of the client.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the client version.
    ///
    /// # Returns
    ///
    /// A string slice containing the version of the client.
    #[must_use]
    pub const fn version(&self) -> &u16 {
        &self.version
    }

    /// Returns the `ExchangeID` of the client.
    ///
    /// # Returns
    ///
    /// An `ExchangeID` representing the id of the exchange the client is connected to.
    #[must_use]
    pub const fn exchange_id(&self) -> ExchangeID {
        self.exchange_id
    }

    /// Generates a channel name for the control channel based on the client name.
    ///
    /// # Returns
    ///
    /// A String in the format: "{client_name}-control".
    ///
    #[must_use]
    pub fn control_channel(&self) -> String {
        format!("{}-{}", self.name, "control")
    }

    /// Generates a channel name for the data channel based on the client name.
    ///
    /// # Returns
    ///
    /// A String in the format: "{client_name}-data".
    ///
    #[must_use]
    pub fn data_channel(&self) -> String {
        format!("{}-{}", self.name, "data")
    }

    /// Generates a channel name for the error channel based on the client name.
    ///
    /// # Returns
    ///
    /// A String in the format: "{client_name}-error".
    #[must_use]
    pub fn error_channel(&self) -> String {
        format!("{}-{}", self.name, "error")
    }

    /// Generates a channel name for the execution channel based on the client name.
    ///
    /// # Returns
    ///
    /// A String in the format: "{client_name}-execution".
    #[must_use]
    pub fn execution_channel(&self) -> String {
        format!("{}-{}", self.name, "execution")
    }
}

impl Default for IntegrationMessageConfig {
    fn default() -> Self {
        Self::new(0, 0, ExchangeID::default())
    }
}
