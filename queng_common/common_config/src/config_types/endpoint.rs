use std::fmt::{Display, Formatter};

use crate::{HostEndpoint, ProtocolType};

/// An Endpoint represents a single endpoint of a service.
///
/// # Fields
///
/// * `name`: The name of the endpoint.
/// * `version`: The version of the endpoint.
/// * `uri`: The Uniform Resource Identifier (URI) of the endpoint.
/// * `port`: The port number of the endpoint.
/// * `protocol`: The protocol Enum type of the endpoint.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Endpoint {
    name: String,
    version: i32,
    uri: String,
    port: i32,
    protocol: ProtocolType,
}

impl Endpoint {
    #[must_use]
    pub const fn new(
        name: String,
        version: u32,
        uri: String,
        port: u32,
        protocol: ProtocolType,
    ) -> Self {
        Self {
            name,
            version: version as i32,
            uri,
            port: port as i32,
            protocol,
        }
    }
}

impl Endpoint {
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
    #[must_use]
    pub const fn version(&self) -> u32 {
        self.version as u32
    }
    #[must_use]
    pub fn uri(&self) -> &str {
        &self.uri
    }
    #[must_use]
    pub const fn port(&self) -> u32 {
        self.port as u32
    }
    #[must_use]
    pub const fn protocol(&self) -> ProtocolType {
        self.protocol
    }
}

impl Endpoint {
    #[must_use]
    pub fn host_endpoint(&self) -> HostEndpoint {
        HostEndpoint::new(self.uri(), self.port() as u16)
    }
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {},  version: {},  port: {},  uri: {},  protocol: {}",
            self.name, self.version, self.port, self.uri, self.protocol
        )
    }
}
