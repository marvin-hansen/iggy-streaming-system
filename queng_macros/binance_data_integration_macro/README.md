# Binance IMS Data Integration Macros

This crate provides a set of derive macros for implementing Binance IMS (Integration Management System) data integration traits. 
These macros automatically generate trait implementations for structs that contain an `ImsBinanceDataIntegration` field.

## Available Macros

### 1. `BinanceImsDataIntegration`

Implements the `ImsDataIntegration` trait by delegating all methods to the underlying `integration` field.

```rust
#[derive(BinanceImsDataIntegration)]
pub struct ImsBinanceSpotDataIntegration {
    integration: ImsBinanceDataIntegration,
}
```

This generates:
```rust
impl ImsDataIntegration for ImsBinanceSpotDataIntegration {
    async fn get_exchange_symbols(&self) -> Result<HashSet<String>, MessageProcessingError> {
        self.integration.get_exchange_symbols().await
    }

    async fn validate_symbols(&self, symbols: &[String]) -> Result<bool, MessageProcessingError> {
        self.integration.validate_symbols(symbols).await
    }
}
```

### 2. `BinanceImsTradeDataIntegration`

Implements the `ImsTradeDataIntegration` trait by delegating all methods to the underlying `integration` field.

```rust
#[derive(BinanceImsTradeDataIntegration)]
pub struct ImsBinanceSpotDataIntegration {
    integration: ImsBinanceDataIntegration,
}
```

This generates:
```rust
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
```

### 3. `BinanceImsOhlcvDataIntegration`

Implements the `ImsOhlcvDataIntegration` trait by delegating all methods to the underlying `integration` field.

```rust
#[derive(BinanceImsOhlcvDataIntegration)]
pub struct ImsBinanceSpotDataIntegration {
    integration: ImsBinanceDataIntegration,
}
```

This generates:
```rust
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
```

## Usage

1. Add this crate as a dependency in your `Cargo.toml`:
```toml
[dependencies]
binance_data_integration_macro = { path = "../path/to/binance_data_integration_macro" }
```

2. Import and use the macros:
```rust
use binance_data_integration_macro::{
    BinanceImsDataIntegration,
    BinanceImsTradeDataIntegration,
    BinanceImsOhlcvDataIntegration,
};

#[derive(
    BinanceImsDataIntegration,
    BinanceImsTradeDataIntegration,
    BinanceImsOhlcvDataIntegration
)]
pub struct ImsBinanceSpotDataIntegration {
    integration: ImsBinanceDataIntegration,
}
```

## Requirements

- The struct must have a field named `integration` of type `ImsBinanceDataIntegration`
- Each macro generates a trait implementation that delegates all method calls to this `integration` field
- The macros handle async functions and generic type parameters correctly

## Note

These macros are designed to work specifically with the Binance IMS integration system and assume the presence of certain traits and types in the codebase. Make sure your project has all the necessary dependencies and types available.
