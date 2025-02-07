use crate::EnvironmentManager;
use common_env::EnvironmentType;
use common_platform::PlatformType;

impl EnvironmentManager {
    /// Returns the type of environment.
    ///
    /// This method returns the `EnvironmentType` of the current `EnvironmentManager` instance.
    ///
    /// # Returns
    ///
    /// The `EnvironmentType` associated with this manager.
    ///
    #[inline]
    pub const fn env_type(&self) -> EnvironmentType {
        self.env_type
    }

    /// Returns the platform type.
    ///
    /// This method returns the `PlatformType` of the current `EnvironmentManager` instance.
    ///
    /// # Returns
    ///
    /// The `PlatformType` associated with this manager.
    ///
    #[inline]
    pub const fn platform_type(&self) -> PlatformType {
        self.platform_type
    }
}
