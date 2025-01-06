use std::fmt::{Display, Formatter};

/// An u8 encoded Enum that represents the unique ID of a service.
///
/// # Variants
///
/// * `NullVal`: Null value in case deserialization fails due to an unknown value.
/// * `Default`: Default value.
/// * `SMDB`: The `SMDb` service.
/// * `CMDB`: The CMDB service.
/// * `DBGW`: The DBGW service.
/// * `QDGW`: The QDGW service.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum ServiceID {
    #[default]
    Default = 0x0_u8,
    SMDB = 0x1_u8,
    CMDB = 0x2_u8,
    DBGW = 0x3_u8,
    QDGW = 0x4_u8,
    MDDB = 0x5_u8,
    VEX = 0x6_u8,
    ImsDataBinance = 0x7_u8,
    KaikoProxy = 0x8_u8,
    IMDB = 0x9_u8,
}

impl ServiceID {
    #[must_use]
    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }

    #[must_use]
    pub const fn as_u16(&self) -> u16 {
        *self as u16
    }

    #[must_use]
    pub const fn as_i32(&self) -> i32 {
        *self as i32
    }

    #[must_use]
    pub const fn as_u32(&self) -> u32 {
        *self as u32
    }

    #[must_use]
    pub fn name(&self) -> String {
        self.to_string()
    }
}

impl From<i16> for ServiceID {
    fn from(value: i16) -> Self {
        match value {
            0x0_i16 => Self::Default,
            0x1_i16 => Self::SMDB,
            0x2_i16 => Self::CMDB,
            0x3_i16 => Self::DBGW,
            0x4_i16 => Self::QDGW,
            0x5_i16 => Self::MDDB,
            0x6_i16 => Self::VEX,
            0x7_i16 => Self::ImsDataBinance,
            0x8_i16 => Self::KaikoProxy,
            0x9_i16 => Self::IMDB,
            _ => Self::Default,
        }
    }
}

impl From<i32> for ServiceID {
    #[inline]
    fn from(v: i32) -> Self {
        match v {
            0x0_i32 => Self::Default,
            0x1_i32 => Self::SMDB,
            0x2_i32 => Self::CMDB,
            0x3_i32 => Self::DBGW,
            0x4_i32 => Self::QDGW,
            0x5_i32 => Self::MDDB,
            0x6_i32 => Self::VEX,
            0x7_i32 => Self::ImsDataBinance,
            0x8_i32 => Self::KaikoProxy,
            0x9_i32 => Self::IMDB,
            _ => Self::Default,
        }
    }
}

impl From<u8> for ServiceID {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::SMDB,
            2 => Self::CMDB,
            3 => Self::DBGW,
            4 => Self::QDGW,
            5 => Self::MDDB,
            6 => Self::VEX,
            7 => Self::ImsDataBinance,
            8 => Self::KaikoProxy,
            9 => Self::IMDB,
            _ => Self::Default,
        }
    }
}

impl ServiceID {
    #[must_use]
    pub fn from_string(n: &str) -> Option<Self> {
        match n {
            "Default" => Some(Self::Default),
            "SMDB" => Some(Self::SMDB),
            "CMDB" => Some(Self::CMDB),
            "DBGW" => Some(Self::DBGW),
            "QDGW" => Some(Self::QDGW),
            "MDDB" => Some(Self::MDDB),
            "VEX" => Some(Self::VEX),
            "ImsDataBinance" => Some(Self::ImsDataBinance),
            "KaikoProxy" => Some(Self::KaikoProxy),
            "KAIKO_PROXY" => Some(Self::KaikoProxy),
            "IMDB" => Some(Self::IMDB),
            _ => None,
        }
    }
}

impl Display for ServiceID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
