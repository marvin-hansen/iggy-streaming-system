use common_exchange::ExchangeID;

#[test]
fn test_from_valid_values() {
    assert_eq!(ExchangeID::from(0x0), ExchangeID::NullVal);
    assert_eq!(ExchangeID::from(0x1), ExchangeID::Kraken);
    assert_eq!(ExchangeID::from(0x2), ExchangeID::COINBASE);
    assert_eq!(ExchangeID::from(0x3), ExchangeID::VEX);
}

#[test]
fn test_from_invalid_values() {
    assert_eq!(ExchangeID::from(0x0), ExchangeID::NullVal);
}

#[test]
fn test_null_val() {
    let exchange_id = ExchangeID::NullVal;
    assert_eq!(format!("{exchange_id}"), "nullval");
}

#[test]
fn test_binance() {
    let exchange_id = ExchangeID::Kraken;
    assert_eq!(format!("{exchange_id}"), "kraken");
}

#[test]
fn test_vex() {
    let exchange_id = ExchangeID::VEX;
    assert_eq!(format!("{exchange_id}"), "vex");
}
