use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ExchangeID {
    #[default]
    NullVal = 0_u8,
    Kraken = 1_u8,
    COINBASE = 2_u8,
    VEX = 3_u8,
    BinanceSpot = 4_u8,
    BinanceSpotTestnet = 5_u8,
    BinanceCoinMarginFuture = 6_u8,
    BinanceCoinMarginFutureTestnet = 7_u8,
    BinanceUsdMarginFuture = 8_u8,
    BinanceUsdMarginFutureTestnet = 9_u8,
}

impl ExchangeID {
    pub fn as_u16(&self) -> u16 {
        *self as u16
    }
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
            4 => Self::BinanceSpot,
            5 => Self::BinanceSpotTestnet,
            6 => Self::BinanceCoinMarginFuture,
            7 => Self::BinanceCoinMarginFutureTestnet,
            8 => Self::BinanceUsdMarginFuture,
            9 => Self::BinanceUsdMarginFutureTestnet,
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
            4_u16 => Self::BinanceSpot,
            5_u16 => Self::BinanceSpotTestnet,
            6_u16 => Self::BinanceCoinMarginFuture,
            7_u16 => Self::BinanceCoinMarginFutureTestnet,
            8_u16 => Self::BinanceUsdMarginFuture,
            9_u16 => Self::BinanceUsdMarginFutureTestnet,
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
            0x4_u32 => Self::BinanceSpot,
            0x5_u32 => Self::BinanceSpotTestnet,
            0x6_u32 => Self::BinanceCoinMarginFuture,
            0x7_u32 => Self::BinanceCoinMarginFutureTestnet,
            0x8_u32 => Self::BinanceUsdMarginFuture,
            0x9_u32 => Self::BinanceUsdMarginFutureTestnet,
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
            0x4_i32 => Self::BinanceSpot,
            0x5_i32 => Self::BinanceSpotTestnet,
            0x6_i32 => Self::BinanceCoinMarginFuture,
            0x7_i32 => Self::BinanceCoinMarginFutureTestnet,
            0x8_i32 => Self::BinanceUsdMarginFuture,
            0x9_i32 => Self::BinanceUsdMarginFutureTestnet,
            _ => Self::NullVal,
        }
    }
}

impl Display for ExchangeID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExchangeID::NullVal => write!(f, "{}", "NullVal".to_ascii_lowercase()),
            ExchangeID::Kraken => write!(f, "{}", "Kraken".to_ascii_lowercase()),
            ExchangeID::COINBASE => write!(f, "{}", "Coinbase".to_ascii_lowercase()),
            ExchangeID::VEX => write!(f, "{}", "VEX".to_ascii_lowercase()),
            ExchangeID::BinanceSpot => write!(f, "{}", "BinanceSpot".to_ascii_lowercase()),
            ExchangeID::BinanceSpotTestnet => {
                write!(f, "{}", "BinanceSpotTestnet".to_ascii_lowercase())
            }
            ExchangeID::BinanceCoinMarginFuture => {
                write!(f, "{}", "BinanceCoinMarginFuture".to_ascii_lowercase())
            }
            ExchangeID::BinanceCoinMarginFutureTestnet => write!(
                f,
                "{}",
                "BinanceCoinMarginFutureTestnet".to_ascii_lowercase()
            ),
            ExchangeID::BinanceUsdMarginFuture => {
                write!(f, "{}", "BinanceUsdMarginFuture".to_ascii_lowercase())
            }
            ExchangeID::BinanceUsdMarginFutureTestnet => write!(
                f,
                "{}",
                "BinanceUsdMarginFutureTestnet".to_ascii_lowercase()
            ),
        }
    }
}
