use common_data_bar::DataType;

#[test]
fn test_from_u8_conversion() {
    assert_eq!(DataType::from(0), DataType::UnknownDataType);
    assert_eq!(DataType::from(1), DataType::TradeData);
    assert_eq!(DataType::from(2), DataType::OHLCVData);
}

#[test]
fn test_from_u8_unknown_value() {
    assert_eq!(DataType::from(3), DataType::UnknownDataType);
    assert_eq!(DataType::from(255), DataType::UnknownDataType);
}

#[test]
fn test_display_formatting() {
    assert_eq!(format!("{}", DataType::UnknownDataType), "UnknownDataType");
    assert_eq!(format!("{}", DataType::TradeData), "TradeData");
    assert_eq!(format!("{}", DataType::OHLCVData), "OHLCVData");
}
