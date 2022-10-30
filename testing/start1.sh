#!/bin/bash
set -e
here=$(realpath $(dirname "$0"))
cd "$here/.."

cargo build --bin snarkd

SNARKD_CONFIG="$here/node1.yaml" ./target/debug/snarkd