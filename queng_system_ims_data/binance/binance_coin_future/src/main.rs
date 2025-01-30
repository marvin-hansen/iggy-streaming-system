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
    ims_data_service::start(
        DBG,
        EXCHANGE_ID,
        ImsBinanceCoinFuturesDataIntegration::new(),
    )
    .await
    .expect("Failed to start Binance IMS Data service");

    Ok(())
}
