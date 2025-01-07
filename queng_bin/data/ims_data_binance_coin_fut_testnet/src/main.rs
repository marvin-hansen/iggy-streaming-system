use binance_coin_futures_data_integration::ImsBinanceCoinFuturesDataIntegration;
use mimalloc::MiMalloc;
use std::fmt::Error;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const SVC_ID: &str = "BinanceCoinFuturesTestnet";
const DBG: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    ims_data_bin::start(DBG, SVC_ID, ImsBinanceCoinFuturesDataIntegration::testnet())
        .await
        .expect("Failed to start Binance IMS Coin Futures TESTNET Data service");

    Ok(())
}
