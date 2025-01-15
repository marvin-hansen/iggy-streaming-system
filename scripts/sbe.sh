# bin/sh
set -o errexit
set -o nounset
set -o pipefail


# Check if Java is working
# Java is assumed to be installed with SDKMAN
# https://www.andrewhoog.com/post/3-ways-to-install-java-on-macos-2023/#install-java-with-sdkman-1
command java --version >/dev/null 2>&1 || {
#  If not, source it.
    source "$HOME/.sdkman/bin/sdkman-init.sh"
}

#  https://github.com/real-logic/simple-binary-encoding?tab=readme-ov-file
command java --add-opens java.base/jdk.internal.misc=ALL-UNNAMED -Dsbe.xinclude.aware=true -Dsbe.generate.ir=true -Dsbe.target.language=Rust -Dsbe.target.namespace=sbe -Dsbe.output.dir=queng_sbe/ -Dsbe.errorLog=yes -jar tools/sbe/sbe-all-1.35.0.jar queng_sbe/sbe_schema/schema.xml

# Check if folder queng_sbe/sbe_bindings exists, and if so remove it
if [ -d "queng_sbe/sbe_bindings" ]; then
    rm -rf queng_sbe/sbe_bindings
fi

# Rename queng_sbe/bindings to queng_sbe/sbe_bindings
mv queng_sbe/bindings queng_sbe/sbe_bindings

# Create a BUILD.bazel in the queng_sbe/sbe_bindings folder to build with Bazel
cat > queng_sbe/sbe_bindings/BUILD.bazel <<EOF
load("@rules_rust//rust:defs.bzl", "rust_doc", "rust_doc_test", "rust_library", "rust_test")

rust_library(
    name = "sbe_bindings",
    srcs = glob([
        "src/**/*.rs",
    ]),
    crate_root = "src/lib.rs",
    tags = [
        "sbe",
        "sbe_bindings",
    ],
    visibility = ["//visibility:public"],
    deps = [],
)

rust_doc(
    name = "doc",
    crate = ":sbe_bindings",
    tags = ["doc"],
    visibility = ["//visibility:public"],
)

# Test documentation
rust_doc_test(
    name = "doc_test",
    crate = ":sbe_bindings",
    tags = ["doc-test"],
    visibility = ["//visibility:public"],
)
EOF

echo "Done: SBE Bindings generated!"
exit 0
