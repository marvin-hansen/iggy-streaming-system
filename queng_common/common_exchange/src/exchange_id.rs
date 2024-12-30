use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ExchangeID {
    #[default]
    NullVal = 0_u8,
    Kraken = 1_u8,
    COINBASE = 2_u8,
    VEX = 3_u8,
    Binance = 4_u8,
}

// Convert an ExchangeID to an i32
impl From<ExchangeID> for i32 {
    #[inline]
    fn from(val: ExchangeID) -> Self {
        val as Self
    }
}

// Convert an ExchangeID to an u32
impl From<ExchangeID> for u32 {
    #[inline]
    fn from(val: ExchangeID) -> Self {
        val as Self
    }
}

// Convert an ExchangeID to an u8
impl From<ExchangeID> for u8 {
    #[inline]
    fn from(val: ExchangeID) -> Self {
        val as Self
    }
}

// Create an ExchangeID from an u8
impl From<u8> for ExchangeID {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0 => Self::NullVal,
            1 => Self::Kraken,
            2 => Self::COINBASE,
            3 => Self::VEX,
            4 => Self::Binance,
            _ => Self::NullVal,
        }
    }
}

// Create an ExchangeID from an u16
impl From<u16> for ExchangeID {
    #[inline]
    fn from(v: u16) -> Self {
        match v {
            0xff_u16 => Self::NullVal,
            0x1_u16 => Self::Kraken,
            0x2_u16 => Self::COINBASE,
            0x3_u16 => Self::VEX,
            0x4_u16 => Self::Binance,
            _ => Self::NullVal,
        }
    }
}

// Create an ExchangeID from an u32
impl From<u32> for ExchangeID {
    #[inline]
    fn from(v: u32) -> Self {
        match v {
            0xff_u32 => Self::NullVal,
            0x1_u32 => Self::Kraken,
            0x2_u32 => Self::COINBASE,
            0x3_u32 => Self::VEX,
            0x4_u32 => Self::Binance,
            _ => Self::NullVal,
        }
    }
}

// Create an ExchangeID from an i32
impl From<i32> for ExchangeID {
    #[inline]
    fn from(v: i32) -> Self {
        match v {
            0xff_i32 => Self::NullVal,
            0x1_i32 => Self::Kraken,
            0x2_i32 => Self::COINBASE,
            0x3_i32 => Self::VEX,
            0x4_i32 => Self::Binance,
            _ => Self::NullVal,
        }
    }
}

impl Display for ExchangeID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
