mod util;
mod api;

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
