use std::fmt::{Display, Formatter};

/// Struct that represents a host endpoint.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct HostEndpoint<'l> {
    /// Host URI.
    host_uri: &'l str,
    /// Port number.
    port: u16,
}

impl<'l> HostEndpoint<'l> {
    /// Creates a new `HostEndpoint` instance.
    ///
    /// # Arguments
    ///
    /// * `host_uri` - Host URI.
    /// * `port` - Port number.
    #[must_use]
    pub const fn new(host_uri: &'l str, port: u16) -> Self {
        Self { host_uri, port }
    }

    /// Returns the host URI.
    #[must_use]
    pub const fn host_uri(&self) -> &str {
        self.host_uri
    }

    /// Returns the port number.
    #[must_use]
    pub const fn port(&self) -> u16 {
        self.port
    }
}

impl Display for HostEndpoint<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "host_uri: {},  port: {}", self.host_uri, self.port)
    }
}
