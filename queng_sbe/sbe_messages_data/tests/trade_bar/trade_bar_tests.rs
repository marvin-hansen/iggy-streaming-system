use chrono::{DateTime, Utc};
use common_data_bar::TradeBar;
use rust_decimal::Decimal;
use std::str::FromStr;

// Default uses utc::now() for date_time, which is not deterministic,
// and that would cause the encode test to fail. Therefore we use a fixed date.
fn get_trade_bar() -> TradeBar {
    let symbol_id = "APPL".to_string();
    let date_str = "2022-04-12T22:10:57+02:00";
    // convert the string into DateTime<FixedOffset>
    let datetime_fixed = DateTime::parse_from_rfc3339(date_str).unwrap();
    // convert the string into DateTime<Utc> or other timezone
    let date_time = datetime_fixed.with_timezone(&Utc);
    let price = Decimal::from_str("250.50").unwrap();
    let volume = Decimal::from(100);
    //
    TradeBar::new(symbol_id, date_time, price, volume)
}

#[test]
fn test_encode_data_bar_message() {
    let bar = get_trade_bar();

    let result = sbe_messages_data::encode_trade_bar_message(bar);

    assert!(result.is_ok()); // Assert encode passes

    let (size, encoded) = result.unwrap();
    assert_eq!(size, 56); // Assert encoded message size matches expected
    assert!(!encoded.is_empty()); // Assert non-empty encoded message

    let actual = encoded;
    let expected: Vec<u8> = vec![
        48, 0, 205, 0, 1, 0, 1, 0, 205, 0, 65, 80, 80, 76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 64, 22, 164, 168, 122, 220, 5, 0, 218, 97, 0, 0, 0, 0, 0, 0, 2, 100, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    assert_eq!(expected, actual);
}

#[test]
fn test_decode_trade_bar_message() {
    let encoded: Vec<u8> = vec![
        48, 0, 205, 0, 1, 0, 1, 0, 205, 0, 65, 80, 80, 76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 64, 22, 164, 168, 122, 220, 5, 0, 218, 97, 0, 0, 0, 0, 0, 0, 2, 100, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let message = sbe_messages_data::decode_trade_bar_message(&encoded).unwrap();

    let symbol_id = "APPL".to_string();
    assert_eq!(message.symbol_id(), symbol_id);
    assert_eq!(message.price(), Decimal::from_str("250.50").unwrap());
    assert_eq!(message.volume(), Decimal::from(100));
}
