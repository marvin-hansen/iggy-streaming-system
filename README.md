# Iggy Streaming System

A sample streaming project written in Rust that showcases end to end streaming with
the [Iggy.rs messaging system](https://iggy.rs/) that is capable of processing millions of messages per second on a
single machine.

This project integrates all three Binance services: spot, coin future and usd future and relays market data SBE encoded
via iggy.

## Current status

Active and early development so things may break or change any given day.

## This project features:

* Multiple streaming microservices in [the ims data folder](queng_system_ims_data).
* Pluggable data source integration in the [integration/data folder](queng_integration/data).
* Efficient SBE message encoding in the [SBE folder](queng_sbe).
* Type extension based serialization in the [extensions folder](queng_extensions).
* Template based microservices in [the template folder](queng_template).

## Quick start

### **1) Clone the iggy repo and start the iggy-server:**

```text
    git clone https://github.com/marvin-hansen/iggy.git
    cd iggy
    # Build the release binaries with Cargo 
    RUSTFLAGS='-C target-cpu=native' cargo build --release
    # Start the iggy server
    ./target/release/iggy-server 
     
  ___                             ____                                      
 |_ _|   __ _    __ _   _   _    / ___|    ___   _ __  __   __   ___   _ __ 
  | |   / _` |  / _` | | | | |   \___ \   / _ \ | '__| \ \ / /  / _ \ | '__|
  | |  | (_| | | (_| | | |_| |    ___) | |  __/ | |     \ V /  |  __/ | |   
 |___|  \__, |  \__, |  \__, |   |____/   \___| |_|      \_/    \___| |_|   
        |___/   |___/   |___/                                               

Loading config from path: 'configs/server.toml'...
Found configuration file at path: 'configs/server.toml'.
Config loaded successfully.       
``` 

### **2) Clone this repo and build it:**

Note, if you want to build with Bazel, ensure you
have [Bazelisk installed](https://github.com/bazelbuild/bazelisk?tab=readme-ov-file#installation). Bazelisk then
downloads Bazel automatically.

```text
    git clone https://github.com/marvin-hansen/iggy-streaming-system.git
    cd iggy-streaming-system
    # Build release binaries with Bazel 
    make release
    # OR build release binaries with Cargo
    RUSTFLAGS='-C target-cpu=native' cargo build --release      
``` 

### **3) Run the Iggy Streaming System**

@TODO

## Build

This project is built using [Bazel](https://bazel.build/) and [Cargo](https://doc.rust-lang.org/cargo/).
For details about Bazel, see the official documentation [https://bazel.build](https://bazel.build)
and the project specific notes in the [Bazel](BAZEL.md) document.

## Repo structure

The main folders of this project:

* [queng_common](queng_common): common libraries shared across the project
* [queng_components](queng_components): common components shared across the project
* [queng_extensions](queng_extensions): type extensions shared across the project
* [queng_macros](queng_macros): macros mostly used in [queng_integration](queng_integration)
* [queng_message](queng_message): iggy messaging utils
* [queng_sbe](queng_sbe): SBE message encoding
* [queng_system_ims_client](queng_system_ims_client): client for the IMS data system
* [queng_system_ims_data](queng_system_ims_data): microservices of the IMS data system
* [queng_integration](queng_integration): integration code for the IMS data system
* [queng_template](queng_template): microservice template used in [queng_system_ims_data](queng_system_ims_data)
* [queng_traits](queng_traits): traits used across all parts of the IMS data system.
* [queng_utils_test](queng_utils_test): Utils for integration tests mostly used
  in [queng_system_ims_data](queng_system_ims_data)

## Pluggable data source integration

The data source integration is implemented in the [queng_integration](queng_integration) folder.
Each integration implements the [ImsDataIntegration](queng_traits/trait_data_integration/src/lib.rs) trait. Notice, the
Binance integration is s special case because of its separation into spot, coin future and usd future markets for which
all API's are almost identical. Therefore, the core integration for all of Binance is implemented in
the [BinanceIntegration](queng_integration/data/binance_core_data_integration) crate. All specialized implementations
i.e. Spot, Coin Future and Usd Future are derived from the core implementation via
the [data_integration_macro](queng_macros/data_integration_macro/src/lib.rs).

Each data source integration is implemented as a standalone library crate with no other dependency other than the
ImsDataIntegration trait and supporting macros.

The standalone integration is then plugged into a [microservice template](queng_template/ims_data_service)via a simple
main function to serve as a data microservice for the IMS data
system. [The Binance Spot integration](queng_system_ims_data/binance/binance_spot/src/main.rs), is shown below as an
example:

```rust
const DBG: bool = true;
const EXCHANGE_ID: ExchangeID = ExchangeID::BinanceSpot;
  
#[tokio::main]
async fn main() -> Result<(), Error> {
    ims_data_service::start(DBG, EXCHANGE_ID, ImsBinanceSpotDataIntegration::new())
        .await
        .expect("Failed to start Binance IMS Data service");

    Ok(())
}
```

The service is then started via cargo:

```bash
  RUSTFLAGS='-C target-cpu=native' ENV=LOCAL cargo run --bin binance_spot
```

Or via Bazel:

```bash
  ENV=LOCAL bazel run  -c opt //alias/service:binance_spot
```

## Licence
This project is licensed under the [Apache License, Version 2.0](LICENSE).

## Author
* [Marvin Hansen](https://github.com/marvin-hansen)
* Contact: https://deepcausality.com/contact/
