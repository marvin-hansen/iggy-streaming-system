use binance_coin_futures_data_integration::ImsBinanceCoinFuturesDataIntegration;
use common_exchange::ExchangeID;
use mimalloc::MiMalloc;
use std::fmt::Error;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const DBG: bool = true;
const EXCHANGE_ID: ExchangeID = ExchangeID::BinanceCoinMarginFuture;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // https://github.com/snapview/tokio-tungstenite/issues/353
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install default rustls crypto provider");

    ims_data_service::start(
        DBG,
        EXCHANGE_ID,
        ImsBinanceCoinFuturesDataIntegration::new(),
    )
    .await
    .expect("Failed to start Binance IMS Data service");

    Ok(())
}
