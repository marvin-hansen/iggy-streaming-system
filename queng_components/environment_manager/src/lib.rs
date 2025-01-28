mod util;

use common_env::EnvironmentType;
use common_platform::PlatformType;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct EnvironmentManager {
    dbg: bool,
    env_type: EnvironmentType,
    platform_type: PlatformType,
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
    pub fn new() -> Self {
        Self::build(false)
    }

    /// Creates a new instance of `EnvironmentManager` with debug mode enabled.
    ///
    /// # Returns
    ///
    /// The constructed instance of `EnvironmentManager`.
    ///
    pub fn with_debug() -> Self {
        Self::build(true)
    }

    fn build(dbg: bool) -> Self {
        let env_type = util::detect_env_type(dbg);
        let platform_type = util::detect_platform_type(dbg);
        Self {
            dbg,
            env_type,
            platform_type,
        }
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
