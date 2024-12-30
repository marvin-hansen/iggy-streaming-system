use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use crate::TradeBar;
/// Getter methods for TradeBar
impl TradeBar {
    /// Returns the timestamp of TradeBar
    #[must_use]
    pub const fn date_time(&self) -> DateTime<Utc> {
        self.trade_time
    }
    /// Returns the price of TradeBar
    #[must_use]
    pub const fn price(&self) -> Decimal {
        self.price
    }
    /// Returns the volume of TradeBar
    #[must_use]
    pub const fn volume(&self) -> Decimal {
        self.volume
    }
    /// Returns the symbol_id of TradeBar
    #[must_use]
    pub fn symbol_id(&self) -> &str {
        &self.symbol_id
    }
}
