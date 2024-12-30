use common_exchange::Instrument;

#[test]
fn test_new() {
    let instrument = Instrument::new(
        "ens-krw".to_string(),
        "spot".to_string(),
        "cbse".to_string(),
        "KRW-ENS".to_string(),
        "ens".to_string(),
        "krw".to_string(),
        None,
    );

    let code = "ens-krw".to_string();
    let class = "spot".to_string();
    let exchange_code = "cbse".to_string();
    let exchange_pair_code = "KRW-ENS".to_string();
    let base_asset = "ens".to_string();
    let quote_asset = "krw".to_string();

    assert_eq!(code, instrument.code());
    assert_eq!(class, instrument.class());
    assert_eq!(exchange_code, instrument.exchange_code());
    assert_eq!(exchange_pair_code, instrument.exchange_pair_code());
    assert_eq!(base_asset, instrument.base_asset());
    assert_eq!(quote_asset, instrument.quote_asset());
    assert!(instrument.instrument_figi().is_none());
}
