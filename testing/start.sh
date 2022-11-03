#!/bin/bash
set -e
here=$(realpath $(dirname "$0"))
cd "$here/.."

cargo build --bin snarkd

tmux \
    new-session "SNARKD_CONFIG='$here/node1.yaml' ./target/debug/snarkd" \; \
    split-window -h "SNARKD_CONFIG='$here/node2.yaml' ./target/debug/snarkd"
