# Prompt

Please design and implement 3 macros in the binance_data_integration_macro
crate:

1. BinanceImsDataIntegration
2. BinanceImsTradeDataIntegration
3. BinanceImsOhlcvDataIntegration

Please
1. Design all 3 macros based on the requirements below:
2. Implement all 3 macros in a separate source file and re-export each macro via the lib.rs file
3. Build the binance_data_integration_macro crate using cargo build -p binance_data_integration_macro to ensure it compiles.

Requirements:

1) BinanceImsDataIntegration

For a given struct, the macro generates the entire ImsDataIntegration trait.
Notice, the struct always has the following field:

integration: ImsBinanceDataIntegration,


For example, the ImsBinanceSpotDataIntegration structure looks as follows

pub struct ImsBinanceSpotDataIntegration {
integration: ImsBinanceDataIntegration,
}

Adding the BinanceImsDataIntegration macro like so:

#[derive(BinanceImsDataIntegration)]
pub struct ImsBinanceSpotDataIntegration {
integration: ImsBinanceDataIntegration,
}

Then generates the following trait implementation:

impl ImsDataIntegration for ImsBinanceSpotDataIntegration {
async fn get_exchange_symbols(&self) -> Result<HashSet<String>, MessageProcessingError> {
self.integration.get_exchange_symbols().await
}

    async fn validate_symbols(&self, symbols: &[String]) -> Result<bool, MessageProcessingError> {
        self.integration.validate_symbols(symbols).await
    }
}

2) BinanceImsTradeDataIntegration

For a given struct, the macro generates the entire ImsTradeDataIntegration trait.
Notice, the struct always has the following field:

integration: ImsBinanceDataIntegration,

For example, the ImsBinanceSpotDataIntegration structure looks as follows

pub struct ImsBinanceSpotDataIntegration {
integration: ImsBinanceDataIntegration,
}

Adding the BinanceImsTradeDataIntegration macro like so

#[derive(BinanceImsTradeDataIntegration)]
pub struct ImsBinanceSpotDataIntegration {
integration: ImsBinanceDataIntegration,
}

Then generates the following trait implementation:

impl ImsTradeDataIntegration for ImsBinanceSpotDataIntegration {
async fn start_trade_data<P>(
&self,
symbols: &[String],
processor: Arc<P>,
) -> Result<(), MessageProcessingError>
where
P: EventProcessor + Send + Sync + 'static,
{
self.integration.start_trade_data(symbols, processor).await
}

    async fn stop_trade_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError> {
        self.integration.stop_trade_data(symbols).await
    }

    async fn stop_all_trade_data(&self) -> Result<(), MessageProcessingError> {
        self.integration.stop_all_trade_data().await
    }
}

3) BinanceImsOhlcvDataIntegration

For a given struct, the macro generates the entire ImsOhlcvDataIntegration  trait.
Notice, the struct always has the following field:

integration: ImsBinanceDataIntegration,

For example, the ImsBinanceSpotDataIntegration structure looks as follows

pub struct ImsBinanceSpotDataIntegration {
integration: ImsBinanceDataIntegration,
}

Adding the BinanceImsOhlcvDataIntegration macro like so

#[derive(BinanceImsOhlcvDataIntegration)]
pub struct ImsBinanceSpotDataIntegration {
integration: ImsBinanceDataIntegration,
}

Then generates the following trait implementation:

impl ImsOhlcvDataIntegration for ImsBinanceSpotDataIntegration {
async fn start_ohlcv_data<P>(
&self,
symbols: &[String],
time_resolution: TimeResolution,
processor: Arc<P>,
) -> Result<(), MessageProcessingError>
where
P: EventProcessor + Send + Sync + 'static,
{
self.integration
.start_ohlcv_data(symbols, time_resolution, processor)
.await
}

    async fn stop_ohlcv_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError> {
        self.integration.stop_ohlcv_data(symbols).await
    }

    async fn stop_all_ohlcv_data(&self) -> Result<(), MessageProcessingError> {
        self.integration.stop_all_ohlcv_data().await
    }
}


Please
1. Design all 3 macros
2. Implement all 3 macros in a separate source file and re-export each macro via the lib.rs file
3. Build the binance_data_integration_macro crate using cargo build -p binance_data_integration_macro to ensure it compiles. 
