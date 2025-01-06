use std::fmt::{Display, Formatter};

use crate::Endpoint;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MetricConfig {
    uri: String,
    host: String,
    port: u32,
}

impl MetricConfig {
    #[must_use]
    pub const fn new(uri: String, host: String, port: u32) -> Self {
        Self { uri, host, port }
    }

    #[must_use]
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
    pub fn uri(&self) -> &str {
        &self.uri
    }
    #[must_use]
    pub fn host(&self) -> &str {
        &self.host
    }
    #[must_use]
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
