use binance_coin_futures_data_integration::ImsBinanceCoinFuturesDataIntegration;
use std::fmt::Error;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const SVC_ID: &str = "BinanceCoinFutures";
const DBG: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    ims_data_bin::start(DBG, SVC_ID, ImsBinanceCoinFuturesDataIntegration::new())
        .await
        .expect("Failed to start Binance IMS Coin Futures Data service");

    Ok(())
}
