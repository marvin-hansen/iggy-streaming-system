use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

mod display;
mod getters;

#[derive(Debug, Default, Eq, Clone, PartialEq)]
pub struct TradeBar {
    symbol_id: String,
    trade_time: DateTime<Utc>,
    price: Decimal,
    volume: Decimal,
}

impl TradeBar {
    /// Creates a new `TradeBar` with the provided values.
    ///
    /// # Parameters
    ///
    /// * `date_time` - The `DateTime` of the trade bar
    /// * `price` - The price for the trade bar as a Decimal
    /// * `volume` - The volume for the trade bar as a Decimal
    ///
    /// # Returns
    ///
    /// A new `TradeBar` instance with the given `date_time`, price and volume.
    #[must_use]
    pub const fn new(
        symbol_id: String,
        trade_time: DateTime<Utc>,
        price: Decimal,
        volume: Decimal,
    ) -> Self {
        Self {
            symbol_id,
            trade_time,
            price,
            volume,
        }
    }
}
