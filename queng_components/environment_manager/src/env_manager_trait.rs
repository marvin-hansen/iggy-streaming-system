use common_env::EnvironmentType;
use common_platform::PlatformType;

pub trait EnvironmentManagerTrait {
    /// Returns the type of environment.
    ///
    /// This method returns the `EnvironmentType` of the current `EnvironmentManager` instance.
    ///
    /// # Returns
    ///
    /// The `EnvironmentType` associated with this manager.
    ///
    fn env_type(&self) -> EnvironmentType;
    /// Returns the platform type.
    ///
    /// This method returns the `PlatformType` of the current `EnvironmentManager` instance.
    ///
    /// # Returns
    ///
    /// The `PlatformType` associated with this manager.
    ///
    fn platform_type(&self) -> PlatformType;
}
