# Alias

Contains target aliased for Bazel.

Bazel build targets are path depended and therefore each relocation of a bazel target
would require an update of all other targets using it. This is a problem when a project
grows large very fast the structure may evolve over time.

The alias target allows to define aliases for targets that are used in the codebase and therefore,
whenever a target relocates, only the alias needs updating to reflect the new location.

Also, unlike Cargo, Bazel alias namespaces can be arbitrarily nested by adding folders.
These aliases are then used as regular dependencies across all Bazel targets i.e.:

```starlark
  rust_test_suite(
    name = "tests",
    srcs = glob(["*_tests.rs",]),
    visibility = ["//visibility:public"],
    deps = [
        # Crate to test
        "//alias/client:ims_data_client",
        # Internal dependencies
        "//alias:common_data_bar",
        "//alias:common_exchange",
        "//alias:common_ims",
        "//alias:trait_event_consumer",
        # External dependencies
        "//thirdparty/crates:tokio",
    ],
)
```