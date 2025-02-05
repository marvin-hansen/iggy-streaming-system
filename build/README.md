# BUILD files

This folder contains a set of bazel specific configurations used across the repository.

## Platforms

Defines binary targets for cross-compilation. Currently only defines Linux and Mac for x86_64 and aarch64.

## Status

Various genrules to determine the current build status of the repository. These genrules are used
to derive tags for container images that require build timestamp and commit information.

## Tools

Contains a small script to export the current HEAD commit as short form hash, which is
required by the status genrules and used by the container images. This script is called from
within the .bazelrc during each Bazel invocation to ensure the build status gets exported correctly.

## binary

A custom bazel build rule to build Rust binary with various compiler optimizations applied.

Note, thus rule required the declaration of the Bazel opt flag in the root level BUILD file.

Ensure the following entry is added to the root level BUILD file:

config_setting(
name = "release",
values = {
"compilation_mode": "opt",
},
)

Then, calling Bazel with the opt flag will result in optimized binaries.

command bazel build -c opt //... --action_env=ENV=LOCAL

## Container

Custom bazel rules to build multi-platform (multi-arch) docker container without docker and custom
rules for generating custom container tags i.e. commit-timestamp. Requires the status genrules and tools.

## Transition

Custom bazel rule transitioning an oci container image to multiple platforms. 


