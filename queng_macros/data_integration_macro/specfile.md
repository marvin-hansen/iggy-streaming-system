# Prompt

Please design and implement one macros in the data_integration_macro crate:

ImsDataIntegration

Please
1. Design the macro based on the requirements below:
2. Implement the macro 
3. Build the data_integration_macro crate using cargo build -p data_integration_macro to ensure it compiles.

Requirements:

For a given struct, the macro generates the entire ImsDataIntegration trait.
Notice, the struct always has the following field:

integration: ImsBinanceDataIntegration,

For example, the ImsBinanceSpotDataIntegration structure looks as follows

```rust
pub struct ImsBinanceSpotDataIntegration {
    integration: ImsBinanceDataIntegration,
}
``` 

Adding the ImsDataIntegration macro like so:

```rust
#[derive(ImsDataIntegration)] 
pub struct ImsBinanceSpotDataIntegration {
    integration: ImsBinanceDataIntegration,
}
``` 

Then generates the following trait implementation:

```rust
impl ImsDataIntegration for ImsBinanceSpotDataIntegration {}

impl ImsSymbolIntegration for ImsBinanceSpotDataIntegration {
    fn get_exchange_symbols(
        &self,
    ) -> impl Future<Output = Result<HashSet<String>, MessageProcessingError>> + Send {
        self.integration.get_exchange_symbols()
    }

    fn validate_symbols(
        &self,
        symbols: &[String],
    ) -> impl Future<Output = Result<bool, MessageProcessingError>> + Send {
        self.integration.validate_symbols(symbols)
    }
}

impl ImsTradeDataIntegration for ImsBinanceSpotDataIntegration {
    fn start_trade_data<P>(
        &self,
        symbols: &[String],
        processor: Arc<P>,
    ) -> impl Future<Output = Result<(), MessageProcessingError>> + Send
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        self.integration.start_trade_data(symbols, processor)
    }

    fn stop_trade_data(
        &self,
        symbols: &[String],
    ) -> impl Future<Output = Result<(), MessageProcessingError>> + Send {
        self.integration.stop_trade_data(symbols)
    }

    fn stop_all_trade_data(
        &self,
    ) -> impl Future<Output = Result<(), MessageProcessingError>> + Send {
        self.integration.stop_all_trade_data()
    }
}

impl ImsOhlcvDataIntegration for ImsBinanceSpotDataIntegration {
    fn start_ohlcv_data<P>(
        &self,
        symbols: &[String],
        time_resolution: TimeResolution,
        processor: Arc<P>,
    ) -> impl Future<Output = Result<(), MessageProcessingError>> + Send
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        self.integration
            .start_ohlcv_data(symbols, time_resolution, processor)
    }

    fn stop_ohlcv_data(
        &self,
        symbols: &[String],
    ) -> impl Future<Output = Result<(), MessageProcessingError>> + Send {
        self.integration.stop_ohlcv_data(symbols)
    }

    fn stop_all_ohlcv_data(
        &self,
    ) -> impl Future<Output = Result<(), MessageProcessingError>> + Send {
        self.integration.stop_all_ohlcv_data()
    }
}
```  

Please
1. Design the macro based on the requirements below:
2. Implement the macro
3. Build the data_integration_macro crate using cargo build -p data_integration_macro to ensure it compiles.
