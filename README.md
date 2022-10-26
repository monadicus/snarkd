# snarkd

## Dependencies

- [Rust](https://www.rust-lang.org/tools/install)
- [protoc](https://github.com/protocolbuffers/protobuf/releases) must be in your `PATH`

## Building

`cargo build`

## Running

Be sure to [configure](#configuration) your node before running.

`cargo run`

## Configuration

### ENV

snarkd uses the following ENV:

| Name | Description |
|---|---|
|`RUST_LOG`|Controlled via [env_logger](https://docs.rs/env_logger/latest/env_logger/#enabling-logging) crate.|
|`SNARKD_CONFIG`|Path to YAML config file|

### YAML File

Config is written in [YAML](https://yaml.org/) and read by default from `./snarkd.yaml` if present, or `/etc/snarkd.yaml`.

The [example config](snarkd.yaml.default), below, can be copied via `cp snarkd.yaml.default snarkd.yaml`:

```yml
# one of none, error, warn, info, debug, trace
# overridden by RUST_LOG present in ENV
verbosity: info
# path to database, when not present
database: ./snarkd.db
```
