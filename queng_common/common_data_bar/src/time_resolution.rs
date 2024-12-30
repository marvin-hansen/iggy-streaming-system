use std::fmt;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum TimeResolution {
    #[default]
    NoValue = 0x0_u8,
    OneMin = 0x1_u8,
    FiveMin = 0x2_u8,
    FifteenMin = 0x3_u8,
    ThirtyMin = 0x4_u8,
    OneHour = 0x5_u8,
    OneDay = 0x6_u8,
    OneWeek = 0x7_u8,
    OneMonth = 0x8_u8,
}

impl From<TimeResolution> for u8 {
    #[inline]
    fn from(value: TimeResolution) -> Self {
        value as u8
    }
}

impl From<TimeResolution> for u16 {
    #[inline]
    fn from(value: TimeResolution) -> Self {
        value as u16
    }
}

impl From<TimeResolution> for u32 {
    #[inline]
    fn from(value: TimeResolution) -> Self {
        value as u32
    }
}

impl From<u8> for TimeResolution {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::NoValue,
            0x1_u8 => Self::OneMin,
            0x2_u8 => Self::FiveMin,
            0x3_u8 => Self::FifteenMin,
            0x4_u8 => Self::ThirtyMin,
            0x5_u8 => Self::OneHour,
            0x6_u8 => Self::OneDay,
            0x7_u8 => Self::OneWeek,
            0x8_u8 => Self::OneMonth,
            _ => Self::NoValue,
        }
    }
}

impl From<u16> for TimeResolution {
    #[inline]
    fn from(v: u16) -> Self {
        match v {
            0x0_u16 => Self::NoValue,
            0x1_u16 => Self::OneMin,
            0x2_u16 => Self::FiveMin,
            0x3_u16 => Self::FifteenMin,
            0x4_u16 => Self::ThirtyMin,
            0x5_u16 => Self::OneHour,
            0x6_u16 => Self::OneDay,
            0x7_u16 => Self::OneWeek,
            0x8_u16 => Self::OneMonth,
            _ => Self::NoValue,
        }
    }
}

impl fmt::Display for TimeResolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoValue => write!(f, "NoValue"),
            Self::OneMin => write!(f, "1m"),
            Self::FiveMin => write!(f, "5m"),
            Self::FifteenMin => write!(f, "15m"),
            Self::ThirtyMin => write!(f, "30m"),
            Self::OneHour => write!(f, "1h"),
            Self::OneDay => write!(f, "1d"),
            Self::OneWeek => write!(f, "1w"),
            Self::OneMonth => write!(f, "1M"),
        }
    }
}
