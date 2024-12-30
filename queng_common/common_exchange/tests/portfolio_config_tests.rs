use common_exchange::{AccountType, Instrument, PortfolioConfig};

fn get_test_instrument() -> Instrument {
    Instrument::new(
        "ens-krw".to_string(),
        "spot".to_string(),
        "cbse".to_string(),
        "KRW-ENS".to_string(),
        "ens".to_string(),
        "krw".to_string(),
        None,
    )
}

fn get_portfolio_config() -> PortfolioConfig {
    let portfolio_id = 1;
    let portfolio_description = "cash portfolio".to_string();
    let portfolio_account_type = AccountType::Spot;
    let portfolio_account_id = "cash_account".to_string();
    let portfolio_currency = "USD".to_string();
    let portfolio_cash_balance = 1000.0;
    let portfolio_max_drawdown = 20.0;
    let portfolio_instruments = vec![get_test_instrument()];
    let instrument_max_allocation = 0.0;
    let instrument_max_drawdown = 10.0;

    PortfolioConfig::new_cash_portfolio(
        portfolio_id,
        portfolio_description,
        portfolio_account_type,
        portfolio_account_id,
        portfolio_currency,
        portfolio_cash_balance,
        portfolio_max_drawdown,
        portfolio_instruments,
        instrument_max_allocation,
        instrument_max_drawdown,
    )
}

#[test]
fn portfolio_config_portfolio_id_returns_expected_value() {
    let portfolio_id = 1;
    let portfolio_config = get_portfolio_config();
    assert_eq!(portfolio_config.portfolio_id(), portfolio_id);
}

#[test]
fn test_portfolio_id() {
    let config = get_portfolio_config();
    assert_eq!(config.portfolio_id(), 1);
}

#[test]
fn test_portfolio_description() {
    let config = get_portfolio_config();
    assert_eq!(config.portfolio_description(), "cash portfolio");
}

#[test]
fn test_portfolio_account_type() {
    let account_type = AccountType::Spot;
    let config = get_portfolio_config();
    assert_eq!(config.portfolio_account_type(), account_type);
}

#[test]
fn test_portfolio_account_id() {
    let config = get_portfolio_config();
    assert_eq!(config.portfolio_account_id(), "cash_account");
}

#[test]
fn test_portfolio_currency() {
    let config = get_portfolio_config();
    assert_eq!(config.portfolio_currency(), "USD");
}

#[test]
fn test_portfolio_cash() {
    let config = get_portfolio_config();
    assert_eq!(config.portfolio_cash(), 1000.0);
}

#[test]
fn test_portfolio_margin() {
    let config = get_portfolio_config();
    assert_eq!(config.portfolio_margin(), 0.0f64);
}

#[test]
fn test_portfolio_max_drawdown() {
    let config = get_portfolio_config();
    assert_eq!(config.portfolio_max_drawdown(), 20.0);
}

#[test]
fn test_portfolio_instruments() {
    let instruments = vec![get_test_instrument()];
    let config = get_portfolio_config();
    assert_eq!(config.portfolio_instruments(), &instruments);
}

#[test]
fn test_instrument_max_allocation() {
    let instrument_max_allocation = 0.0f64;
    let config = get_portfolio_config();
    assert_eq!(
        config.instrument_max_allocation(),
        instrument_max_allocation
    );
}

#[test]
fn test_instrument_max_drawdown() {
    let max_drawdown = 10.0;
    let config = get_portfolio_config();
    assert_eq!(config.instrument_max_drawdown(), max_drawdown);
}

#[test]
fn test_portfolio_free_margin() {
    let config = get_portfolio_config();
    assert_eq!(config.portfolio_free_margin(), 0.0f64);
}

#[test]
fn test_portfolio_free_cash() {
    let config = get_portfolio_config();
    assert_eq!(config.portfolio_free_cash(), (1000.0));
}

#[test]
fn test_portfolio_free_margin_percent() {
    let config = get_portfolio_config();
    assert_eq!(config.portfolio_free_margin_percent(), 0f64);
}

#[test]
fn test_portfolio_free_cash_percent() {
    let config = get_portfolio_config();
    assert_eq!(config.portfolio_free_cash_percent(), 100.0f64);
}

#[test]
fn test_display() {
    let actual = get_portfolio_config().to_string();
    let expected = "portfolio_id: 1, portfolio_description: cash portfolio, portfolio_account_type: Spot, portfolio_account_id: cash_account, portfolio_currency: USD, portfolio_cash: 1000, portfolio_margin: 0.0, portfolio_max_drawdown: 20, portfolio_instruments: [Instrument { code: \"ens-krw\", class: \"spot\", exchange_code: \"cbse\", exchange_pair_code: \"KRW-ENS\", base_asset: \"ens\", quote_asset: \"krw\", instrument_figi: None }], instrument_max_allocation: 0.0, instrument_max_drawdown: 10.0, portfolio_free_margin: 0.0, portfolio_free_cash: 1000.0, portfolio_free_margin_percent: 0.0, portfolio_free_cash_percent: 100.0";
    assert_eq!(expected, actual);
}
