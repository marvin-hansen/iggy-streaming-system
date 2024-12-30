use std::fmt::{Display, Formatter};

// Type is u16 for direct conversion from proto integer. The smallest possible integer in proto is 16B.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SymbolID {
    #[default]
    NullVal = 0xff_u16,
    BTCUSD = 0x1_u16,
    ETHUSD = 0x2_u16,
    LTCUSD = 0x3_u16,
}

impl From<u16> for SymbolID {
    #[inline]
    fn from(v: u16) -> Self {
        match v {
            0xff_u16 => Self::NullVal,
            0x1_u16 => Self::BTCUSD,
            0x2_u16 => Self::ETHUSD,
            0x3_u16 => Self::LTCUSD,
            _ => Self::NullVal,
        }
    }
}

impl Display for SymbolID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NullVal => write!(f, "NullVal"),
            Self::BTCUSD => write!(f, "BTCUSD"),
            Self::ETHUSD => write!(f, "ETHUSD"),
            Self::LTCUSD => write!(f, "LTCUSD"),
        }
    }
}
