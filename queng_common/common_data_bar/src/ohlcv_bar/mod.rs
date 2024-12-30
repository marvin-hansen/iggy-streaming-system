use std::fmt::Debug;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

mod display;
mod getters;

#[derive(Debug, Default, Eq, Clone, PartialEq)]
pub struct OHLCVBar {
    symbol_id: String,
    date_time: DateTime<Utc>,
    open: Decimal,
    high: Decimal,
    low: Decimal,
    close: Decimal,
    volume: Decimal,
}

impl OHLCVBar {
    /// Creates a new `OHLCVBar` instance with the provided parameters.
    ///
    /// # Parameters
    ///
    /// - `symbol_id` - The symbol ID this bar is for
    /// - `date_time` - The date/time of this bar
    /// - `open` - The opening price
    /// - `high` - The high price
    /// - `low` - The low price
    /// - `close` - The closing price
    /// - `volume` - The volume traded
    ///
    /// # Returns
    ///
    /// A new `OHLCVBar` instance with the provided fields populated.
    #[must_use]
    pub const fn new(
        symbol_id: String,
        date_time: DateTime<Utc>,
        open: Decimal,
        high: Decimal,
        low: Decimal,
        close: Decimal,
        volume: Decimal,
    ) -> Self {
        Self {
            symbol_id,
            date_time,
            open,
            high,
            low,
            close,
            volume,
        }
    }
}
