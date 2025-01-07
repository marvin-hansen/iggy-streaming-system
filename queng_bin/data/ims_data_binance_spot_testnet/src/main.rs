use binance_spot_testnet_data_integration::ImsBinanceSpotTestnetDataIntegration;
use mimalloc::MiMalloc;
use std::fmt::Error;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
const SVC_ID: &str = "BinanceSpotTestnet";
const DBG: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    ims_data_bin::start(DBG, SVC_ID, ImsBinanceSpotTestnetDataIntegration::new())
        .await
        .expect("Failed to start Binance IMS SPOT TESTNET Data service");
    Ok(())
}
