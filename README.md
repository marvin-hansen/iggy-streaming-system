# Iggy Streaming System

Sample project that showcases end to end streaming with the [Iggy.rs messaging system](https://iggy.rs/)
that is capable of processing millions of messages per second on a single machine.


## Build commands

```text
    make build          Builds the code base incrementally (fast) for dev.
    make current        Builds the current target incrementally (fast) defined in current.txt.
    make doc            Builds documentation for the project.
    make format         Formats call code according to cargo fmt style.
    make lint           Lints and formats the code of the project.
    make fix            Fixes linting issues as reported by clippy.
    make test           Tests across all crates.
    make vendor         Vendors all Bazel managed Rust dependencies to folder thirdparty.
```


## Licence
This project is licensed under the [Apache License, Version 2.0](LICENSE).

## Author
* [Marvin Hansen](https://github.com/marvin-hansen)
* Contact: https://deepcausality.com/contact/
