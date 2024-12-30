use crate::OHLCVBar;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

impl OHLCVBar {
    #[must_use]
    pub fn range_change(&self) -> Decimal {
        self.close - self.open
    }

    #[must_use]
    pub fn range_percent(&self) -> Decimal {
        let one_hundred = Decimal::new(100, 0);
        (((self.close - self.open) / self.open) * one_hundred).round_dp(4)
    }
}

impl OHLCVBar {
    /// Returns the `date_time` field of the `OHLCVBar`.
    #[must_use]
    pub const fn date_time(&self) -> DateTime<Utc> {
        self.date_time
    }

    /// Returns the `open` field of the `OHLCVBar`.
    #[must_use]
    pub const fn open(&self) -> Decimal {
        self.open
    }

    /// Returns the `high` field of the `OHLCVBar`.
    #[must_use]
    pub const fn high(&self) -> Decimal {
        self.high
    }

    /// Returns the `low` field of the `OHLCVBar`.
    #[must_use]
    pub const fn low(&self) -> Decimal {
        self.low
    }

    /// Returns the `close` field of the `OHLCVBar`.
    #[must_use]
    pub const fn close(&self) -> Decimal {
        self.close
    }

    /// Returns the `volume` field of the `OHLCVBar`.
    #[must_use]
    pub const fn volume(&self) -> Decimal {
        self.volume
    }

    /// Returns the `symbol_id` field of the `OHLCVBar`.
    #[must_use]
    pub fn symbol_id(&self) -> &str {
        &self.symbol_id
    }
}
