use binance_spot_data_integration::ImsBinanceSpotDataIntegration;
use std::fmt::Error;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
const SVC_ID: &str = "BinanceSpot";
const DBG: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    ims_data_bin::start(DBG, SVC_ID, ImsBinanceSpotDataIntegration::new())
        .await
        .expect("Failed to start Binance IMS SPOT Data service");

    Ok(())
}
