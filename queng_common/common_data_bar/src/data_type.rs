use std::fmt::{Display, Formatter};

/// The `DataType` enum represents the different data types that can be transmitted.
///
/// The variants represent the following data types:
///
/// - `UnknownDataType` - Default unknown data type
/// - `TradeData` - Trade/tick data
/// - `OHLCVData` - Open-high-low-close-volume bar data
///
/// The enum is represented as a `u8` under the hood.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum DataType {
    #[default]
    UnknownDataType = 0_u8,
    TradeData = 1_u8,
    OHLCVData = 2_u8,
    // OrderBookData = 3_u8,
    // QuoteData = 4_u8,
}

impl From<u8> for DataType {
    /// Converts a `u8` value to a `DataType` enum variant.
    ///
    /// # Parameters
    ///
    /// * `value` - The `u8` value to convert.
    ///
    /// # Returns
    ///
    /// The corresponding `DataType` variant:
    ///
    /// - `0_u8` maps to `DataType::UnknownDataType`
    /// - `1_u8` maps to `DataType::TradeData`
    /// - `2_u8` maps to `DataType::OHLCVData`
    ///
    /// Any other value maps to `DataType::UnknownDataType`.
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            1_u8 => Self::TradeData,
            2_u8 => Self::OHLCVData,
            _ => Self::UnknownDataType,
        }
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
