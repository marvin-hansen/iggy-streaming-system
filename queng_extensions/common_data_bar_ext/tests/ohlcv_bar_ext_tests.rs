use common_data_bar::OHLCVBar;
use common_data_bar_ext::SbeOHLCVBarExtension;

#[test]
fn test_decode() {
    let bar = OHLCVBar::default();

    let encoded = bar.clone().encode_to_sbe().unwrap().1;
    let decoded = OHLCVBar::decode_from_sbe(&encoded).unwrap();
    assert_eq!(bar, decoded);
}
