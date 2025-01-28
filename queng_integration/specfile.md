# Specfile

| Title          	| Specs for Context Generic Programming based pluggable data integration 	         |
|----------------	|----------------------------------------------------------------------------------|
| Author         	| Marvin Hansen <marvin.hansen@gmail.com>                                	         |
| Date created   	| Sunday, 2024-12-29                                                             	 |
| Date last edit 	| 	                                                                                |

## Context

Current folder: queng_integration
Programming language: Rust
Build tool: Bazel

### Internal dependencies:
common_data_bar = {workspace = true}
common_data_bar_ext = {workspace = true}
common_errors = {workspace = true}

### External dependencies:
chrono = { workspace = true }
futures-util = { workspace = true }
reqwest = { workspace = true }
rust_decimal = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
tokio = {workspace = true}
tokio-tungstenite = { workspace = true}

### Related specfiles
* queng_integration/data/binance_core_data_integration/specfile.md

## Overview

Until very recently, pluggable data integration frameworks were based on the enum_dispatch crate
and a mixture of proc_macros and trait tricks to accomplish the following goals:

1) Separate the data integration from its actual usage i.e. as CLI, binary, or service
2) Provide a unified interface for all data integrations via a common type
3) Maintain static dispatch to ensure excellent runtime performance
4) Minimize maintenance by having a single data integration free of any glue code
5) Minimize integration effort by having a single template service in which each data ingration gets plugged in, hence the name pluggable data integration.

## Problem 

The shortcomings of the initial implementation were many:

1) Disjoint traits that basically had the same signatures thus were redundant

Somehow, enum_dispatch and the async macro do not get along well and cannot be used together on the same trait,
thus resulting int two different traits. One that defines the async interface for all data integrations and one that defines the enum dispatch trait required for static dispatch. The implementation of the latter relies in a pass-through of methids from the async trait, which results in a lot of additional boilerplate code. The boilerplate code was then shifted into a proc macro, but this is rather a fix of a broken design.

2) Too many proc macros for unnecessary dispatch of default implementations.

As mentioned in 1, somehow Rust just can't pass through default implementations without
dispatch. This results in a lot of additional boilerplate code covered by too many macros that
shouldn't exists in the first place.

3) Relatively hard to evolve base traits.

For practicality and domain separation reasons, the data integration trait is already split into three traits
a data integration must implement. However, in case of Binance, a single core integration is actually relayed to all six
integrations (i.e. spot, M-futures, C-futures, and testsnets for each) again, using proc macros to generate dispatch boilerplate. The problem arises if a new method is added to any of the three traits, because then not only the actual integration needs to be updated, but all proc macros generating the dispatch boilerplate also need to be updated and excessive maintenance is just the hallmark of bad design.


## Requirements

The key requirements of the new implementation with CGP is clear cut simpler maintenance without boilerplate and unnecessary custom
proc crates to generate dispatch of default implementations.

### Definitions

**data_integration**:

A data integration refers to a type that implements the data integration traits to provide a unified interface 
to start and stop real-time data from a cryptocurrency exchange. In addition, it provides a unified interface
to query commonly used information i.e. available trading pairs, supported timeframes, etc.

**template_service**:

A template service is a library crate that implements a fully functional microservice as a lib to be used in a binary
by just passing in a configuration file and a data integration.

### Key traits

**ImsDataIntegration**

```rust
#[trait_variant::make(ImsDataIntegration: Send)]
pub trait LocalImsDataIntegration {
    async fn get_exchange_symbols(&self) -> Result<HashSet<String>, MessageProcessingError>;

    async fn validate_symbols(&self, symbols: &[String]) -> Result<bool, MessageProcessingError>;
}
```  

**ImsOhlcvDataIntegration**

```rust
#[trait_variant::make(ImsOhlcvDataIntegration: Send)]
pub trait LocalImsOhlcvDataIntegration{
    async fn start_ohlcv_data<P>(
        &self,
        symbols: &[String],
        time_resolution: TimeResolution,
        processor: Arc<P>,
    ) -> Result<(), MessageProcessingError>
    where
        P: crate::EventProcessor + Send + Sync + 'static;

    async fn stop_ohlcv_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError>;

    async fn stop_all_ohlcv_data(&self) -> Result<(), MessageProcessingError>;
}
```  

**ImsTradeDataIntegration**

```rust 
#[trait_variant::make(ImsTradeDataIntegration: Send)]
pub trait LocalImsTradeDataIntegration {
    async fn start_trade_data<P>(
        &self,
        symbols: &[String],
        processor: Arc<P>,
    ) -> Result<(), MessageProcessingError>
    where
        P: crate::EventProcessor + Send + Sync + 'static;

    async fn stop_trade_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError>;

    async fn stop_all_trade_data(&self) -> Result<(), MessageProcessingError>;
}
``` 


### Functional Requirements

Given that CGP works fundamentally different from the previous implementation, the functional requirements are 
currently stated on a fairly high level describing only outcomes to acknowledge the learning ahead and to leave room to
hammer out the details in the next stage. 

1) Decoupling of data integration provider and the actual data integration consumer

This is perhaps the easiest requirement as existing data integration provider can easily be refactored to 
match the CGP module style.

2) General integration component for the template service

The template service ideally should only recieve an context generic data integration component in its signature
so that the actual binary plugs in the specific data integration provider for that binary.
Taking Binance as an example, there would be three data integration providers:
- BinanceSpotDataIntegrationProvider
- BinanceMfuturesDataIntegrationProvider
- BinanceCfuturesDataIntegrationProvider

Accordingly, there would be three binaries, with each one plugging in a different data integration provider into
an integration component used within the template service.

3) Modular DataIntegrationProvider

As stated earlier, the global data integration traits may evolve over time and as a result, it
is important to keep the data integration provider modular and extensible. Taking the aforementioned Binance example, 
the actual BinanceSpotDataIntegrationProvider would compose of three other providers from the core integration and only
expose the default implementation of the core integration.

### Non-Functional Requirements

**Performance**
Optimize the hotpath code for best performance
Only use safe Rust APIs and features for performance optimization.
Minimize or avoid runtime memory allocations by either pre-allocating correctly or setting sizes at compile time.
Use sensible parallelism when possible.

**Reliability**
Error handling and recovery
Proper resource cleanup
Prevention of memory leaks

**Security**
Limit scope if internal methods
Minimize usage of external dependencies
Apply security best practices to prevent security vulnerabilities.  

### Tasks

**Build:**

**Test:**

**Example:**

**Document:**

**Finalize:**
