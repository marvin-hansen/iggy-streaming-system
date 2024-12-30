use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use common_data_bar::OHLCVBar;

#[must_use]
pub fn get_ohlcv_bar(date_time: DateTime<Utc>) -> OHLCVBar {
    let symbol_id = "APPL".to_string();
    let open = Decimal::new(10000, 2);
    let high = Decimal::new(11000, 2);
    let low = Decimal::new(9000, 2);
    let close = Decimal::new(10500, 2);
    let volume = Decimal::new(1000000, 2);

    OHLCVBar::new(symbol_id, date_time, open, high, low, close, volume)
}

#[test]
fn test_date_time() {
    let now = Utc::now();
    let bar = get_ohlcv_bar(now);

    let expected = now;
    let actual = bar.date_time();

    assert_eq!(expected, actual);
}

#[test]
fn test_open() {
    let now = Utc::now();
    let bar = get_ohlcv_bar(now);

    let expected = Decimal::new(10000, 2);
    let actual = bar.open();

    assert_eq!(expected, actual);
}

#[test]
fn test_high() {
    let now = Utc::now();
    let bar = get_ohlcv_bar(now);

    let expected = Decimal::new(11000, 2);
    let actual = bar.high();

    assert_eq!(expected, actual);
}

#[test]
fn test_low() {
    let now = Utc::now();
    let bar = get_ohlcv_bar(now);

    let expected = Decimal::new(9000, 2);
    let actual = bar.low();

    assert_eq!(expected, actual);
}

#[test]
fn test_close() {
    let now = Utc::now();
    let bar = get_ohlcv_bar(now);

    let expected = Decimal::new(10500, 2);
    let actual = bar.close();

    assert_eq!(expected, actual);
}

#[test]
fn test_volume() {
    let now = Utc::now();
    let bar = get_ohlcv_bar(now);

    let expected = Decimal::new(1000000, 2);
    let actual = bar.volume();

    assert_eq!(expected, actual);
}

#[test]
fn test_symbol_id() {
    let now = Utc::now();
    let bar = get_ohlcv_bar(now);

    let expected = "APPL".to_string();
    let actual = bar.symbol_id();

    assert_eq!(expected, actual);
}
