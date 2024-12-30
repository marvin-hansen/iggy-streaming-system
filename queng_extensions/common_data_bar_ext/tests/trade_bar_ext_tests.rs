use common_data_bar::TradeBar;
use common_data_bar_ext::SbeTradeBarExtension;

#[test]
fn test_decode() {
    let bar = TradeBar::default();

    let encoded = bar.clone().encode_to_sbe().unwrap().1;
    let decoded = TradeBar::decode_from_sbe(&encoded).unwrap();
    assert_eq!(bar, decoded);
}
