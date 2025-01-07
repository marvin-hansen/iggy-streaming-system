use binance_usd_futures_data_integration::ImsBinanceUsdFuturesDataIntegration;
use mimalloc::MiMalloc;
use std::fmt::Error;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
const SVC_ID: &str = "BinanceUSDFutures";
const DBG: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    ims_data_bin::start(DBG, SVC_ID, ImsBinanceUsdFuturesDataIntegration::new())
        .await
        .expect("Failed to start Binance IMS USD Futures Data service");

    Ok(())
}
