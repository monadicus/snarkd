#!/bin/bash
set -e
here=$(realpath $(dirname "$0"))
cd "$here/.."

cargo build --bin snarkd

SNARKD_CONFIG="$here/node2.yaml" ./target/debug/snarkd