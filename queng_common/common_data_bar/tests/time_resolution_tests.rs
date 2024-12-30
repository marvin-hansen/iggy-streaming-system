use common_data_bar::TimeResolution;

#[test]
fn test_from_u8() {
    assert_eq!(TimeResolution::from(0x0_u8), TimeResolution::NoValue);
    assert_eq!(TimeResolution::from(0x1_u8), TimeResolution::OneMin);
    assert_eq!(TimeResolution::from(0x2_u8), TimeResolution::FiveMin);
    assert_eq!(TimeResolution::from(0x3_u8), TimeResolution::FifteenMin);
    assert_eq!(TimeResolution::from(0x4_u8), TimeResolution::ThirtyMin);
    assert_eq!(TimeResolution::from(0x5_u8), TimeResolution::OneHour);
    assert_eq!(TimeResolution::from(0x6_u8), TimeResolution::OneDay);
    assert_eq!(TimeResolution::from(0x7_u8), TimeResolution::OneWeek);
    assert_eq!(TimeResolution::from(0x8_u8), TimeResolution::OneMonth);
}

#[test]
fn test_display() {
    assert_eq!(format!("{}", TimeResolution::NoValue), "NoValue");
    assert_eq!(format!("{}", TimeResolution::OneMin), "1m");
    assert_eq!(format!("{}", TimeResolution::FiveMin), "5m");
    assert_eq!(format!("{}", TimeResolution::FifteenMin), "15m");
    assert_eq!(format!("{}", TimeResolution::ThirtyMin), "30m");
    assert_eq!(format!("{}", TimeResolution::OneHour), "1h");
    assert_eq!(format!("{}", TimeResolution::OneDay), "1d");
    assert_eq!(format!("{}", TimeResolution::OneWeek), "1w");
    assert_eq!(format!("{}", TimeResolution::OneMonth), "1M");
}
