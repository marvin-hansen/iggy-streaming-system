use chrono::Utc;
use common_data_bar::{OHLCVBar, SampledDataBars};
use rust_decimal::Decimal;

fn get_ohlcv_bar(date_time: chrono::DateTime<Utc>) -> OHLCVBar {
    let symbol_id = "APPL".to_string();
    let open = Decimal::new(10000, 2);
    let high = Decimal::new(11000, 2);
    let low = Decimal::new(9000, 2);
    let close = Decimal::new(10500, 2);
    let volume = Decimal::new(1000000, 2);

    OHLCVBar::new(symbol_id, date_time, open, high, low, close, volume)
}

#[test]
fn test_sampled_data_bars_new_initialization() {
    let sampled_data_bars = SampledDataBars::new();
    assert!(sampled_data_bars.day_bars().is_empty());
    assert!(sampled_data_bars.month_bars().is_empty());
    assert!(sampled_data_bars.year_bars().is_empty());
}

#[test]
fn test_set_day_bars() {
    let mut sampled_data_bars = SampledDataBars::new();
    let now = Utc::now();
    let day_bars = vec![get_ohlcv_bar(now)];
    sampled_data_bars.set_day_bars(day_bars.clone());
    assert_eq!(sampled_data_bars.day_bars(), &day_bars);
}

#[test]
fn test_get_day_bars() {
    let mut sampled_data_bars = SampledDataBars::new();
    let now = Utc::now();
    let day_bars = vec![get_ohlcv_bar(now)];
    sampled_data_bars.set_day_bars(day_bars.clone());
    let retrieved_day_bars = sampled_data_bars.day_bars();
    assert_eq!(retrieved_day_bars, &day_bars);
}
