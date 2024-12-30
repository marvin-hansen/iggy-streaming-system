use chrono::Utc;
use common_data_bar::trade::Trade;
use rust_decimal::Decimal;

#[test]
fn test_trade_creation() {
    let trade = Trade::new(
        "AAPL".to_string(),
        "12345".to_string(),
        Utc::now(),
        Utc::now(),
        Decimal::new(100, 2),
        Decimal::new(10, 0),
    );

    assert_eq!(trade.symbol_id(), "AAPL");
    assert_eq!(trade.trade_id(), "12345");
    assert_eq!(trade.price(), Decimal::new(100, 2));
    assert_eq!(trade.quantity(), Decimal::new(10, 0));
}

#[test]
fn test_trade_defaults() {
    let trade = Trade::default();

    assert_eq!(trade.symbol_id(), "");
    assert_eq!(trade.trade_id(), "");
    assert_eq!(trade.price(), Decimal::new(0, 0));
    assert_eq!(trade.quantity(), Decimal::new(0, 0));
}

#[test]
fn test_trade_eq() {
    // Timestamp must be equal for both trades to test equality
    let time = Utc::now();

    let trade1 = Trade::new(
        "AAPL".to_string(),
        "12345".to_string(),
        time,
        time,
        Decimal::new(100, 2),
        Decimal::new(10, 0),
    );

    let trade2 = Trade::new(
        "AAPL".to_string(),
        "12345".to_string(),
        time,
        time,
        Decimal::new(100, 2),
        Decimal::new(10, 0),
    );

    assert_eq!(trade1, trade2);
}

#[test]
fn test_trade_neq() {
    let trade1 = Trade::new(
        "AAPL".to_string(),
        "12345".to_string(),
        Utc::now(),
        Utc::now(),
        Decimal::new(100, 2),
        Decimal::new(10, 0),
    );

    let trade2 = Trade::new(
        "GOOG".to_string(),
        "67890".to_string(),
        Utc::now(),
        Utc::now(),
        Decimal::new(200, 2),
        Decimal::new(20, 0),
    );

    assert_ne!(trade1, trade2);
}

#[test]
fn test_trade_clone() {
    let trade = Trade::new(
        "AAPL".to_string(),
        "12345".to_string(),
        Utc::now(),
        Utc::now(),
        Decimal::new(100, 2),
        Decimal::new(10, 0),
    );

    let cloned_trade = trade.clone();

    assert_eq!(trade, cloned_trade);
}

#[test]
fn test_trade_debug() {
    let trade = Trade::new(
        "AAPL".to_string(),
        "12345".to_string(),
        Utc::now(),
        Utc::now(),
        Decimal::new(100, 2),
        Decimal::new(10, 0),
    );

    let debug_str = format!("{:?}", trade);
    assert!(debug_str.contains("Trade"));
    assert!(debug_str.contains("symbol"));
    assert!(debug_str.contains("trade_id"));
    assert!(debug_str.contains("price"));
    assert!(debug_str.contains("quantity"));
}

#[test]
fn test_trade_display() {
    let trade = Trade::new(
        "AAPL".to_string(),
        "12345".to_string(),
        Utc::now(),
        Utc::now(),
        Decimal::new(100, 2),
        Decimal::new(10, 0),
    );

    let display_str = format!("{}", trade);
    assert!(display_str.contains("Trade"));
    assert!(display_str.contains("symbol"));
    assert!(display_str.contains("trade_id"));
    assert!(display_str.contains("price"));
    assert!(display_str.contains("quantity"));
}
