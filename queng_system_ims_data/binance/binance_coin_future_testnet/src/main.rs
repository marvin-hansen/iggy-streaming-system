use binance_coin_futures_testnet_data_integration::ImsBinanceCoinFuturesTestnetDataIntegration;
use common_exchange::ExchangeID;
use mimalloc::MiMalloc;
use std::fmt::Error;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const DBG: bool = true;
const EXCHANGE_ID: ExchangeID = ExchangeID::BinanceCoinMarginFutureTestnet;

#[tokio::main]
async fn main() -> Result<(), Error> {
    ims_data_service::start(
        DBG,
        EXCHANGE_ID,
        ImsBinanceCoinFuturesTestnetDataIntegration::new(),
    )
    .await
    .expect("Failed to start Binance IMS Data service");

    Ok(())
}
