# Iggy Streaming System

A simple streaming project that showcases end to end streaming with the [Iggy.rs messaging system](https://iggy.rs/)
that is capable of processing millions of messages per second on a single machine.

This project integrates all three Binance services: spot, coin future and usd future and relays market data SBE encoded
via iggy.

## This project features:

* Multiple streaming microservices in [the ims data folder](queng_system_ims_data)
* Pluggable data source integration in the [integration/data folder](queng_integration/data)
* Efficient SBE message encoding in the [SBE folder](queng_sbe).
* Type extension based serialization in the [extensions folder](queng_extensions)
* Template based microservices in [the template folder](queng_template)

## Quick start

1) Clone iggy and start the iggy-server:

```text
    git clone https://github.com/marvin-hansen/iggy.git
    cd iggy
    RUSTFLAGS='-C target-cpu=native' cargo build --release
    # Start the server
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

2) Clone this repo and build it:

```text
    git clone https://github.com/marvin-hansen/iggy-streaming-system.git
    cd iggy-streaming-system
    # Build release binaries with Bazel 
    make release
    # OR build release binaries with Cargo
    RUSTFLAGS='-C target-cpu=native' cargo build --release      
``` 

3) Run the Iggy Streaming System

@TODO


## Licence
This project is licensed under the [Apache License, Version 2.0](LICENSE).

## Author
* [Marvin Hansen](https://github.com/marvin-hansen)
* Contact: https://deepcausality.com/contact/
