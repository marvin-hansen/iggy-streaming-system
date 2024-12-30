use std::fmt::{Display, Formatter};

/// A struct to represent an instrument traded on a specific exchange.
///
/// The information contained by the struct is as follows:
/// - `code`: the instrument's code
/// - `class`: the instrument's class
/// - `exchange_code`: the instrument's exchange code
/// - `exchange_pair_code`: the instrument's exchange pair code
/// - `base_asset`: the instrument's base asset
/// - `quote_asset`: the instrument's quote asset
/// - `instrument_figi`: the instrument's Figi (Financial Instrument Global Identifier)
///
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Instrument {
    code: String,
    class: String,
    exchange_code: String,
    exchange_pair_code: String,
    base_asset: String,
    quote_asset: String,
    instrument_figi: Option<String>,
}

/// Construct a new `Instrument` from its components.
///
/// `code`: the instrument's code
/// `class`: the instrument's class
/// `exchange_code`: the instrument's exchange code
/// `exchange_pair_code`: the instrument's exchange pair code
/// `base_asset`: the instrument's base asset
/// `quote_asset`: the instrument's quote asset
/// `instrument_figi`: the instrument's Figi (Financial Instrument Global Identifier)
impl Instrument {
    #[must_use]
    pub const fn new(
        code: String,
        class: String,
        exchange_code: String,
        exchange_pair_code: String,
        base_asset: String,
        quote_asset: String,
        instrument_figi: Option<String>,
    ) -> Self {
        Self {
            code,
            class,
            exchange_code,
            exchange_pair_code,
            base_asset,
            quote_asset,
            instrument_figi,
        }
    }
}

/// Getters for the instrument's properties.
impl Instrument {
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    #[must_use]
    pub fn class(&self) -> &str {
        &self.class
    }

    #[must_use]
    pub fn exchange_code(&self) -> &str {
        &self.exchange_code
    }

    #[must_use]
    pub fn exchange_pair_code(&self) -> &str {
        &self.exchange_pair_code
    }

    #[must_use]
    pub fn base_asset(&self) -> &str {
        &self.base_asset
    }

    #[must_use]
    pub fn quote_asset(&self) -> &str {
        &self.quote_asset
    }

    #[must_use]
    pub const fn instrument_figi(&self) -> &Option<String> {
        &self.instrument_figi
    }
}

impl Display for Instrument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Instrument: {self:?}")
    }
}
