# Iggy Streaming System

A sample streaming project that showcases end to end streaming with the [Iggy.rs messaging system](https://iggy.rs/)
that is capable of processing millions of messages per second on a single machine.

This project integrates all three Binance services: spot, coin future and usd future and relays market data SBE encoded
via iggy.

## This project features:

* Multiple streaming microservices in [the ims data folder](queng_system_ims_data).
* Pluggable data source integration in the [integration/data folder](queng_integration/data).
* Efficient SBE message encoding in the [SBE folder](queng_sbe).
* Type extension based serialization in the [extensions folder](queng_extensions).
* Template based microservices in [the template folder](queng_template).
* Docker-free container builds via [Bazel's rules_oci](https://github.com/bazel-contrib/rules_oci) with secure base
  images via [rules_apko](https://github.com/chainguard-dev/rules_apko).
* Fast multi-arch container builds via [custom build rules](build).
* Parallel docker-free integration tests via Bazel and
  the [bazel build_utils](https://github.com/marvin-hansen/buildutils) in
  the [tests folder](queng_system_ims_data/binance_tests).
* Fast cross compilation to linux ARM64 via Bazel, [rules_rust](https://github.com/bazelbuild/rules_rust),
  and [musl-toolchain](https://github.com/bazel-contrib/musl-toolchain).

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

## Licence
This project is licensed under the [Apache License, Version 2.0](LICENSE).

## Author
* [Marvin Hansen](https://github.com/marvin-hansen)
* Contact: https://deepcausality.com/contact/
