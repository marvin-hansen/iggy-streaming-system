use common_exchange::{MasterSymbol, MasterSymbolRow};

#[test]
fn test_master_symbol_properties() {
    // Create a MasterSymbol instance
    let master_symbol = MasterSymbol {
        master_symbol: "BTCUSD".to_string(),
        asset_class: "crypto".to_string(),
        base_asset: "BTC".to_string(),
        quote_asset: "USD".to_string(),
    };

    // Test MasterSymbol properties
    assert_eq!(master_symbol.master_symbol, "BTCUSD");
    assert_eq!(master_symbol.asset_class, "crypto");
    assert_eq!(master_symbol.base_asset, "BTC");
    assert_eq!(master_symbol.quote_asset, "USD");
}

#[test]
fn test_master_symbol_row_properties() {
    // Create a MasterSymbolRow instance
    let master_symbol_row = MasterSymbolRow {
        data: vec![
            "BTCUSD".to_string(),
            "crypto".to_string(),
            "BTC".to_string(),
            "USD".to_string(),
        ],
    };

    // Test MasterSymbolRow properties
    assert_eq!(master_symbol_row.data.len(), 4);
    assert_eq!(master_symbol_row.data[0], "BTCUSD");
    assert_eq!(master_symbol_row.data[1], "crypto");
    assert_eq!(master_symbol_row.data[2], "BTC");
    assert_eq!(master_symbol_row.data[3], "USD");
}
