# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# https://www.kevinsimper.dk/posts/how-to-bazel-pass-environment-variables
command bazel build  -c opt  //... --action_env=ENV=LOCAL

command bazel build -c opt //:push --action_env=ENV=LOCAL
