use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum AccountType {
    NullVal = 0xff_u8,
    #[default]
    Spot = 0x1_u8,
    Margin = 0x2_u8,
    Future = 0x3_u8,
}

impl AccountType {
    #[must_use]
    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }

    #[must_use]
    pub const fn as_i32(&self) -> i32 {
        *self as i32
    }
}

impl From<i16> for AccountType {
    #[inline]
    fn from(v: i16) -> Self {
        match v {
            0x1_i16 => Self::Spot,
            0x2_i16 => Self::Margin,
            0x3_i16 => Self::Future,
            _ => Self::NullVal,
        }
    }
}

impl From<u8> for AccountType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::Spot,
            0x2_u8 => Self::Margin,
            0x3_u8 => Self::Future,
            _ => Self::NullVal,
        }
    }
}

impl From<i32> for AccountType {
    #[inline]
    fn from(v: i32) -> Self {
        match v {
            0x1_i32 => Self::Spot,
            0x2_i32 => Self::Margin,
            0x3_i32 => Self::Future,
            _ => Self::NullVal,
        }
    }
}
impl From<String> for AccountType {
    #[inline]
    fn from(v: String) -> Self {
        match v.as_str() {
            "Spot" => Self::Spot,
            "Margin" => Self::Margin,
            "Future" => Self::Future,
            _ => Self::NullVal,
        }
    }
}

impl From<&str> for AccountType {
    #[inline]
    fn from(v: &str) -> Self {
        match v {
            "Spot" => Self::Spot,
            "Margin" => Self::Margin,
            "Future" => Self::Future,
            _ => Self::NullVal,
        }
    }
}

impl Display for AccountType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Spot => write!(f, "Spot"),
            Self::Margin => write!(f, "Margin"),
            Self::Future => write!(f, "Future"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
