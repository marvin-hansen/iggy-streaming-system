use std::fmt::{Display, Formatter};

use crate::{AccountType, Instrument};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PortfolioConfig {
    portfolio_id: u32,
    portfolio_description: String,
    portfolio_account_type: AccountType,
    portfolio_account_id: String,
    portfolio_currency: String,
    portfolio_cash: f64,
    portfolio_margin: f64,
    portfolio_max_drawdown: f64,
    portfolio_instruments: Vec<Instrument>,
    instrument_max_allocation: f64,
    instrument_max_drawdown: f64,
    portfolio_free_margin: f64,
    portfolio_free_cash: f64,
    portfolio_free_margin_percent: f64,
    portfolio_free_cash_percent: f64,
}

impl PortfolioConfig {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        portfolio_id: u32,
        portfolio_description: String,
        portfolio_account_type: AccountType,
        portfolio_account_id: String,
        portfolio_currency: String,
        portfolio_cash: f64,
        portfolio_margin: f64,
        portfolio_max_drawdown: f64,
        portfolio_instruments: Vec<Instrument>,
        instrument_max_allocation: f64,
        instrument_max_drawdown: f64,
        portfolio_free_margin: f64,
        portfolio_free_cash: f64,
        portfolio_free_margin_percent: f64,
        portfolio_free_cash_percent: f64,
    ) -> Self {
        Self {
            portfolio_id,
            portfolio_description,
            portfolio_account_type,
            portfolio_account_id,
            portfolio_currency,
            portfolio_cash,
            portfolio_margin,
            portfolio_max_drawdown,
            portfolio_instruments,
            instrument_max_allocation,
            instrument_max_drawdown,
            portfolio_free_margin,
            portfolio_free_cash,
            portfolio_free_margin_percent,
            portfolio_free_cash_percent,
        }
    }

    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new_cash_portfolio(
        portfolio_id: u32,
        portfolio_description: String,
        portfolio_account_type: AccountType,
        portfolio_account_id: String,
        portfolio_currency: String,
        portfolio_cash: f64,
        portfolio_max_drawdown: f64,
        portfolio_instruments: Vec<Instrument>,
        instrument_max_allocation: f64,
        instrument_max_drawdown: f64,
    ) -> Self {
        Self {
            portfolio_id,
            portfolio_description,
            portfolio_account_type,
            portfolio_account_id,
            portfolio_currency,
            portfolio_cash,
            portfolio_margin: 0f64,
            portfolio_max_drawdown,
            portfolio_instruments,
            instrument_max_allocation,
            instrument_max_drawdown,
            portfolio_free_margin: 0f64,
            portfolio_free_cash: portfolio_cash,
            portfolio_free_margin_percent: 0f64,
            portfolio_free_cash_percent: 100.0,
        }
    }
}

impl PortfolioConfig {
    #[must_use]
    pub const fn portfolio_id(&self) -> u32 {
        self.portfolio_id
    }
    #[must_use]
    pub fn portfolio_description(&self) -> &str {
        &self.portfolio_description
    }
    #[must_use]
    pub const fn portfolio_account_type(&self) -> AccountType {
        self.portfolio_account_type
    }
    #[must_use]
    pub fn portfolio_account_id(&self) -> &str {
        &self.portfolio_account_id
    }
    #[must_use]
    pub fn portfolio_currency(&self) -> &str {
        &self.portfolio_currency
    }
    #[must_use]
    pub const fn portfolio_cash(&self) -> f64 {
        self.portfolio_cash
    }
    #[must_use]
    pub const fn portfolio_margin(&self) -> f64 {
        self.portfolio_margin
    }
    #[must_use]
    pub const fn portfolio_max_drawdown(&self) -> f64 {
        self.portfolio_max_drawdown
    }
    #[must_use]
    pub const fn portfolio_instruments(&self) -> &Vec<Instrument> {
        &self.portfolio_instruments
    }
    #[must_use]
    pub const fn instrument_max_allocation(&self) -> f64 {
        self.instrument_max_allocation
    }
    #[must_use]
    pub const fn instrument_max_drawdown(&self) -> f64 {
        self.instrument_max_drawdown
    }
    #[must_use]
    pub const fn portfolio_free_margin(&self) -> f64 {
        self.portfolio_free_margin
    }
    #[must_use]
    pub const fn portfolio_free_cash(&self) -> f64 {
        self.portfolio_free_cash
    }
    #[must_use]
    pub const fn portfolio_free_margin_percent(&self) -> f64 {
        self.portfolio_free_margin_percent
    }
    #[must_use]
    pub const fn portfolio_free_cash_percent(&self) -> f64 {
        self.portfolio_free_cash_percent
    }
}

impl Display for PortfolioConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "portfolio_id: {}, portfolio_description: {}, portfolio_account_type: {}, \
               portfolio_account_id: {}, portfolio_currency: {}, \
               portfolio_cash: {}, portfolio_margin: {:?}, portfolio_max_drawdown: {}, \
               portfolio_instruments: {:?}, instrument_max_allocation: {:?}, \
               instrument_max_drawdown: {:?}, portfolio_free_margin: {:?}, portfolio_free_cash: {:?}, \
               portfolio_free_margin_percent: {:?}, portfolio_free_cash_percent: {:?}",
            self.portfolio_id,
            self.portfolio_description,
            self.portfolio_account_type,
            self.portfolio_account_id,
            self.portfolio_currency,
            self.portfolio_cash,
            self.portfolio_margin,
            self.portfolio_max_drawdown,
            self.portfolio_instruments,
            self.instrument_max_allocation,
            self.instrument_max_drawdown,
            self.portfolio_free_margin,
            self.portfolio_free_cash,
            self.portfolio_free_margin_percent,
            self.portfolio_free_cash_percent
        )
    }
}
