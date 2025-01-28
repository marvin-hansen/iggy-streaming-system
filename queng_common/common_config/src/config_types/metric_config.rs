use std::fmt::{Display, Formatter};

use crate::Endpoint;

/// A structure for describing a metrics endpoint.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MetricConfig {
    /// The URI of the metrics endpoint.
    uri: String,
    /// The hostname of the metrics endpoint.
    host: String,
    /// The port of the metrics endpoint.
    port: u32,
}

/// Methods for `MetricConfig` that are used to get the `uri`, `host` and `port`
/// fields of the struct.
impl MetricConfig {
    #[must_use]
    /// Creates a new `MetricConfig` with `uri`, `host` and `port` set to the
    /// corresponding arguments.
    pub const fn new(uri: String, host: String, port: u32) -> Self {
        Self { uri, host, port }
    }

    #[must_use]
    /// Creates a new `MetricConfig` with `uri` set to the `Endpoint`'s uri and
    /// `host` and `port` set to the `Endpoint`'s host and port.
    pub fn from_endpoint(endpoint: &Endpoint) -> Self {
        let uri = endpoint.uri().to_string();
        let host = String::from("0.0.0.0");
        let port = endpoint.port();
        Self { uri, host, port }
    }
}

impl Default for MetricConfig {
    fn default() -> Self {
        Self {
            uri: String::from("metrics"),
            host: String::from("0.0.0.0"),
            port: 8080,
        }
    }
}

impl MetricConfig {
    #[must_use]
    /// Returns the `uri` field of the `MetricConfig`.
    pub fn uri(&self) -> &str {
        &self.uri
    }
    #[must_use]
    /// Returns the `host` field of the `MetricConfig`.
    pub fn host(&self) -> &str {
        &self.host
    }
    #[must_use]
    /// Returns the `port` field of the `MetricConfig`.
    pub const fn port(&self) -> u32 {
        self.port
    }
}

impl Display for MetricConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "metric_uri: {},  metric_host: {},  metric_port: {}",
            self.uri, self.host, self.port
        )
    }
}
