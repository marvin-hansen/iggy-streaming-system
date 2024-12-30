use std::fmt;

use rust_decimal::Decimal;

use crate::ExchangeID;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Symbol {
    symbol_id_global: String,
    symbol_id_exchange: String,
    exchange_id: ExchangeID,
    asset_base_exchange: String,
    asset_quote_exchange: String,
    price_precision: Decimal,
    size_precision: Decimal,
}

impl Symbol {
    #[must_use]
    pub const fn new(
        symbol_id_global: String,
        symbol_id_exchange: String,
        exchange_id: ExchangeID,
        asset_base_exchange: String,
        asset_quote_exchange: String,
        price_precision: Decimal,
        size_precision: Decimal,
    ) -> Self {
        Self {
            symbol_id_global,
            symbol_id_exchange,
            exchange_id,
            asset_base_exchange,
            asset_quote_exchange,
            price_precision,
            size_precision,
        }
    }
}

impl Symbol {
    #[must_use]
    pub fn symbol_id_global(&self) -> &str {
        &self.symbol_id_global
    }
    #[must_use]
    pub fn symbol_id_exchange(&self) -> &str {
        &self.symbol_id_exchange
    }
    #[must_use]
    pub const fn exchange_id(&self) -> &ExchangeID {
        &self.exchange_id
    }
    #[must_use]
    pub fn asset_base_exchange(&self) -> &str {
        &self.asset_base_exchange
    }
    #[must_use]
    pub fn asset_quote_exchange(&self) -> &str {
        &self.asset_quote_exchange
    }
    #[must_use]
    pub const fn price_precision(&self) -> Decimal {
        self.price_precision
    }
    #[must_use]
    pub const fn size_precision(&self) -> Decimal {
        self.size_precision
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Symbol[global_id: {}, exchange_id: {}, base: {}, quote: {}, price_precision: {}, size_precision: {}]",
            self.symbol_id_global,
            self.symbol_id_exchange,
            self.asset_base_exchange,
            self.asset_quote_exchange,
            self.price_precision,
            self.size_precision
        )
    }
}
