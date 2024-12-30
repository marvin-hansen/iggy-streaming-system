use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Eq, Clone, PartialEq)]
pub struct Trade {
    symbol_id: String,
    trade_id: String,
    time_exchange: DateTime<Utc>,
    time_integration: DateTime<Utc>,
    price: Decimal,
    quantity: Decimal,
}

impl Trade {
    /// Creates a new `Trade` instance.
    ///
    /// # Arguments
    /// * `symbol` - Trading symbol (e.g., "BTCUSDT")
    /// * `time_exchange` - Timestamp of the trade in the exchange  (UTC)
    /// * `time_integration` - Timestamp of trade data arriving on the integration system  (UTC)
    /// * `trade_id` - Unique identifier for the trade
    /// * `price` - Price of the trade
    /// * `quantity` - Quantity of the trade
    ///
    /// # Returns
    /// - A new `Trade` instance
    ///
    pub fn new(
        symbol_id: String,
        trade_id: String,
        time_exchange: DateTime<Utc>,
        time_integration: DateTime<Utc>,
        price: Decimal,
        quantity: Decimal,
    ) -> Self {
        Self {
            symbol_id,
            trade_id,
            time_exchange,
            time_integration,
            price,
            quantity,
        }
    }
}

impl Trade {
    pub fn symbol_id(&self) -> &str {
        &self.symbol_id
    }

    pub fn time_exchange(&self) -> DateTime<Utc> {
        self.time_exchange
    }

    pub fn time_integration(&self) -> DateTime<Utc> {
        self.time_integration
    }

    pub fn trade_id(&self) -> &str {
        &self.trade_id
    }

    pub fn price(&self) -> Decimal {
        self.price
    }

    pub fn quantity(&self) -> Decimal {
        self.quantity
    }
}

impl Display for Trade {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
