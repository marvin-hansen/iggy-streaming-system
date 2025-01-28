use binance_usd_futures_data_integration::ImsBinanceUsdFuturesDataIntegration;
use common_exchange::ExchangeID;
use mimalloc::MiMalloc;
use std::fmt::Error;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const DBG: bool = true;
const EXCHANGE_ID: ExchangeID = ExchangeID::BinanceUsdMarginFuture;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // https://github.com/snapview/tokio-tungstenite/issues/353
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install default rustls crypto provider");

    ims_data_service::start(DBG, EXCHANGE_ID, ImsBinanceUsdFuturesDataIntegration::new())
        .await
        .expect("Failed to start Binance IMS Data service");

    Ok(())
}
