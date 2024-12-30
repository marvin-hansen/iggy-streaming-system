# cgp-exploration

Exploration of [Context Generic Programming](https://contextgeneric.dev/blog/early-preview-announcement/) in Rust. 


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
