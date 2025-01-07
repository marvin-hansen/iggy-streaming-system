use binance_usd_futures_testnet_data_integration::ImsBinanceUsdFuturesTestnetDataIntegration;
use mimalloc::MiMalloc;
use std::fmt::Error;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
const SVC_ID: &str = "BinanceUSDFuturesTestnet";
const DBG: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    ims_data_bin::start(
        DBG,
        SVC_ID,
        ImsBinanceUsdFuturesTestnetDataIntegration::new(),
    )
    .await
    .expect("Failed to start Binance IMS USD Futures TESTNET Data service");

    Ok(())
}
