use std::fmt::Error;
use binance_usd_futures_data_integration::ImsBinanceUsdFuturesDataIntegration;

const SVC_ID: &str = "BinanceUSDFuturesTestnet";
const DBG: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {

    ims_data_bin::start(DBG, SVC_ID, ImsBinanceUsdFuturesDataIntegration::testnet())
        .await
        .expect("Failed to start Binance IMS USD Futures TESTNET Data service");

    Ok(())
}
