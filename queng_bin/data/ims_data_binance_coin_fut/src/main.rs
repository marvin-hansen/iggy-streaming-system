use binance_coin_futures_data_integration::ImsBinanceCoinFuturesDataIntegration;
use mimalloc::MiMalloc;
use std::fmt::Error;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const SVC_ID: &str = "BinanceCoinFutures";
const DBG: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    // Initialize rustls crypto provider https://github.com/snapview/tokio-tungstenite/issues/353
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install default rustls crypto provider");

    ims_data_bin::start(DBG, SVC_ID, ImsBinanceCoinFuturesDataIntegration::new())
        .await
        .expect("Failed to start Binance IMS Coin Futures Data service");

    Ok(())
}
