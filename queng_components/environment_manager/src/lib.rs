mod util;

use common_env::EnvironmentType;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct EnvironmentManager {
    dbg: bool,
    env_type: EnvironmentType,
}

impl EnvironmentManager {
    /// Creates a new instance of `EnvironmentManager`.
    ///
    /// The `new` method creates an instance of `EnvironmentManager` with debug mode disabled.
    ///
    /// # Returns
    ///
    /// The constructed instance of `EnvironmentManager`.
    ///
    #[must_use]
    pub fn new() -> Self {
        let env_type = util::detect_env_type(false);
        Self::build(false, env_type)
    }

    /// Creates a new instance of `EnvironmentManager` with debug mode enabled.
    ///
    /// # Returns
    ///
    /// The constructed instance of `EnvironmentManager`.
    ///
    #[must_use]
    pub fn with_debug() -> Self {
        let env_type = util::detect_env_type(true);
        Self::build(true, env_type)
    }

    const fn build(dbg: bool, env_type: EnvironmentType) -> Self {
        Self { dbg, env_type }
    }
}

impl EnvironmentManager {
    /// Returns the type of environment.
    ///
    /// This method returns the `EnvironmentType` of the current `EnvironmentManager` instance.
    ///
    /// # Returns
    ///
    /// The `EnvironmentType` associated with this manager.
    ///
    #[must_use]
    pub const fn env_type(&self) -> EnvironmentType {
        self.env_type
    }
}
