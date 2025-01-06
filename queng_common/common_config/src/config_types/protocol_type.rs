use std::fmt::{Display, Formatter};

/// A `ProtocolType` represents the protocol type used for communication.
///
/// # Variants
///
/// * `GRPC`: The gRPC protocol.
/// * `HTTP`: The HTTP protocol.
/// * `UDP`: The UDP protocol.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum ProtocolType {
    #[default]
    UnknownProtocol = 0,
    /// The gRPC protocol.
    GRPC = 1,
    /// The HTTP protocol.
    HTTP = 2,
    /// The UDP protocol.
    UDP = 3,
}

impl ProtocolType {
    #[must_use]
    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }
}

impl From<i16> for ProtocolType {
    fn from(value: i16) -> Self {
        match value {
            0x1_i16 => Self::GRPC,
            0x2_i16 => Self::HTTP,
            0x3_i16 => Self::UDP,
            _ => Self::UnknownProtocol,
        }
    }
}

impl From<i8> for ProtocolType {
    fn from(value: i8) -> Self {
        match value {
            0x1_i8 => Self::GRPC,
            0x2_i8 => Self::HTTP,
            0x3_i8 => Self::UDP,
            _ => Self::UnknownProtocol,
        }
    }
}

impl From<u8> for ProtocolType {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0x1_u8 => Self::GRPC,
            0x2_u8 => Self::HTTP,
            0x3_u8 => Self::UDP,
            _ => Self::UnknownProtocol,
        }
    }
}

impl From<i32> for ProtocolType {
    /// All .proto enumeration types convert to the Rust i32 type.
    /// This functions converts a raw i32 byte value back into a `ServiceType`.
    /// Unknown message type results in `NullVal`
    #[inline]
    fn from(value: i32) -> Self {
        match value {
            0x1_i32 => Self::GRPC,
            0x2_i32 => Self::HTTP,
            0x3_i32 => Self::UDP,
            _ => Self::UnknownProtocol,
        }
    }
}

impl ProtocolType {
    #[must_use]
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "GRPC" => Some(Self::GRPC),
            "HTTP" => Some(Self::HTTP),
            "UDP" => Some(Self::UDP),
            _ => None,
        }
    }
}

impl Display for ProtocolType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GRPC => write!(f, "GRPC"),
            Self::HTTP => write!(f, "HTTP"),
            Self::UDP => write!(f, "UDP"),
            Self::UnknownProtocol => write!(f, "UnknownProtocol"),
        }
    }
}
