use std::fmt::{Display, Formatter};

/// An `EnvironmentType` represents the environment type of the application.
///
/// # Variants
///
/// * `UNKNOWN`: The unknown environment type.
/// * `LOCAL`: The local environment type.
/// * `CLUSTER`: The cluster environment type.
///
/// Default is set to `UNKNOWN`.
///
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum EnvironmentType {
    /// The unknown environment type.
    #[default]
    UNKNOWN,
    /// The local environment type.
    LOCAL,
    /// The cluster environment type.
    CLUSTER,
    /// Continuous Integration (CI) environment type.
    CI,
}

impl Display for EnvironmentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}
