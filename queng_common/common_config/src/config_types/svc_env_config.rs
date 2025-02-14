use std::fmt::{Display, Formatter};

use crate::ServiceID;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SvcEnvConfig {
    /// The service ID of the service
    service_id: ServiceID,
    /// The port on which the service is listening
    service_port: String,
    /// The hostname address of the cluster
    cluster_host: String,
    /// The hostname address in Continuous Integration (CI) for testing
    ci_host: String,
    /// The hostname address of the local machine
    local_host: String,
    /// The host address when running in Docker
    docker_host: String,
    /// The metric endpoint address
    metrics_host: String,
    /// The metric endpoint URI of the service
    metrics_uri: String,
    /// The metric endpoint port of the service
    metrics_port: u32,
}

impl SvcEnvConfig {
    /// Creates a new `SvcEnvConfig` with the given parameters
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        service_id: ServiceID,
        cluster_host: String,
        ci_host: String,
        local_host: String,
        docker_host: String,
        service_port: String,
        metrics_host: String,
        metrics_uri: String,
        metrics_port: u32,
    ) -> Self {
        Self {
            service_id,
            service_port,
            cluster_host,
            ci_host,
            local_host,
            docker_host,
            metrics_host,
            metrics_uri,
            metrics_port,
        }
    }
}

impl SvcEnvConfig {
    /// Returns the hostname address of the host in a cluster
    #[must_use]
    pub fn cluster_host(&self) -> &str {
        &self.cluster_host
    }
    /// Returns the hostname address of the host in Continuous Integration (CI)
    #[must_use]
    pub fn ci_host(&self) -> &str {
        &self.ci_host
    }
    /// Returns the hostname of the host on a local machine
    #[must_use]
    pub fn local_host(&self) -> &str {
        &self.local_host
    }
    /// Returns the hostname address of the host when running in Docker
    #[must_use]
    pub fn docker_host(&self) -> &str {
        &self.docker_host
    }
    /// Returns the port on which the service is listening
    #[must_use]
    pub fn service_port(&self) -> &str {
        &self.service_port
    }
    /// Returns the service ID
    #[must_use]
    pub const fn service_id(&self) -> ServiceID {
        self.service_id
    }
    /// Returns the metrics host of the service
    #[must_use]
    pub fn metrics_host(&self) -> &str {
        &self.metrics_host
    }
    /// Returns the metric endpoint URI of the service
    #[must_use]
    pub fn metrics_uri(&self) -> &str {
        &self.metrics_uri
    }
    /// Returns the metric endpoint port of the service
    #[must_use]
    pub const fn metrics_port(&self) -> &u32 {
        &self.metrics_port
    }
}

impl Display for SvcEnvConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SvcEnvConfig {{ service_id: {:?}, cluster_host: {:?}, ci_host: {:?}, local_host: {:?}, docker_host: {:?}, service_port: {:?}, metrics_host: {:?}, metrics_uri: {:?}, metrics_port: {:?} }}",
            self.service_id,
            self.cluster_host,
            self.ci_host,
            self.local_host,
            self.docker_host,
            self.service_port,
            self.metrics_host,
            self.metrics_uri,
            self.metrics_port
        )
    }
}
