use std::fmt::{Display, Formatter};

/// A `ServiceType` represents the type of service.
///
/// # Variants
///
/// * `ENDPOINT`: An endpoint service type.
/// * `CHANNEL`: The channel service type.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum ServiceType {
    NullVal = 0x0_u8,
    /// The endpoint service type.
    #[default]
    ENDPOINT = 0x1_u8,
    /// The channel service type.
    CHANNEL = 0x2_u8,
}

impl ServiceType {
    #[must_use]
    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }
}

impl From<i16> for ServiceType {
    fn from(value: i16) -> Self {
        match value {
            0x0_i16 => Self::NullVal,
            0x1_i16 => Self::ENDPOINT,
            0x2_i16 => Self::CHANNEL,
            _ => Self::NullVal,
        }
    }
}

impl From<i32> for ServiceType {
    /// Converts a raw byte value into a `ServiceType`.
    /// Unknown message type results in `NullVal`
    #[inline]
    fn from(v: i32) -> Self {
        match v {
            0x0_i32 => Self::NullVal,
            0x1_i32 => Self::ENDPOINT,
            0x2_i32 => Self::CHANNEL,
            _ => Self::NullVal,
        }
    }
}

impl Display for ServiceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ENDPOINT => write!(f, "ENDPOINT"),
            Self::CHANNEL => write!(f, "CHANNEL"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
