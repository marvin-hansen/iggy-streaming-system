use binance_spot_data_integration::ImsBinanceSpotDataIntegration;
use std::fmt::Error;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
const SVC_ID: &str = "BinanceSpotTestnet";
const DBG: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    ims_data_bin::start(DBG, SVC_ID, ImsBinanceSpotDataIntegration::testnet())
        .await
        .expect("Failed to start Binance IMS SPOT TESTNET Data service");
    Ok(())
}
