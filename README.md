# snarkd

## Dependencies

- [Rust](https://www.rust-lang.org/tools/install)
- [protoc](https://github.com/protocolbuffers/protobuf/releases) must be in your `PATH`

## Building

The following builds both `snarkd` and `snarkd_cli`

**Production**: `cargo build --release`  
**Development**: `cargo build`

## Running

Be sure to [configure](#configuration) your node before running.

### Snarkd

- If you have previously run `cargo build`:  
    **Production**: `target/release/snarkd`  
    **Development**: `target/debug/snarkd`  
- Otherwise, you can run with:  
    **Production**: `cargo run --release --bin snarkd`  
    **Development**: `cargo run  --bin snarkd`  

### Snarkd CLI

- If you have previously run `cargo build`:  
    **Production**: `target/release/snarkd_cli --help`  
    **Development**: `target/debug/snarkd_cli --help`  
- Otherwise, you can run with:  
    **Production**: `cargo run --release --bin snarkd_cli -- --help`  
    **Development**: `cargo run  --bin snarkd_cli -- --help`  

## Testing

### Rust

Our rust-based tests can be run via `cargo test`.

### Scripts

We have some test scripts in `testing/`.

Prerequisites:
  - Ubuntu/Debian: `sudo apt install tmux coreutils`
  - OS X: `brew install tmux coreutils`

|Filename|Description|
|-|-|
|`start1.sh`|Starts a "node1" snarkd node that connects to node2|
|`start2.sh`|Starts a "node2" snarkd node that connects to node1|
|`start.sh`|Starts "node1" and "node2" snarkd nodes in a split-window tmux session|

## Configuration

### ENV

snarkd uses the following ENV:

|Name|Description|
|-|-|
|`RUST_LOG`|Controlled via [env_logger](https://docs.rs/env_logger/latest/env_logger/#enabling-logging) crate.|
|`SNARKD_CONFIG`|Path to YAML config file|

### YAML File

Config is written in [YAML](https://yaml.org/) and read by default from `./snarkd.yaml` if present, or `/etc/snarkd.yaml`.

The [example config](snarkd.yaml.default), below, can be copied via `cp snarkd.yaml.default snarkd.yaml`:

**Note**: Comments are denoted by `##`, commented out defaults are denoted by `#`.

```yml
## Log level verbosity. One of none, error, warn, info, debug, trace
## overridden by RUST_LOG present in ENV
verbosity: info
## If not specified, an in-memory database is used
database_file: ./snarkd.db
## At least this number of connections will be maintained
minium_connection_count: 20
## No more than this number of connections will be maintained
maximum_connection_count: 50
## If true (default), then we announce our existence to the tracker
enable_tracker_announce: true
## Seconds between peer syncs. Default 1.
peer_sync_interval: 1
## Port we are actually listening to
listen_port: 5423
## Address that we are listening to. Defaults to 0.0.0.0
listen_ip: 0.0.0.0
## Port that we are receiving connections on. Generally the same as `listen_port` but a port rewrite firewall rule might change that.
inbound_port: 5423
## configuration for talking to trackers. defaults should be fine.
#tracker:
  ## Bittorrent Peer id, defaults to `-MD0001-{12 random hex chars}`
  # peer_id: '20_length_string_pls'
  ## Info Hash for finding peers
  # info_hash: '40 character hex string'

  ## List of trackers to find peers from. Leave empty to disable tracker based peer discovery
  ## HTTP trackers only at the moment
  # trackers:
  #   - 'http://tracker.opentrackr.org:1337/announce'

  ## List of initial peers to connect to (via bittorrent)
  # peers:
  #     - '192.168.1.2:5423'
```
