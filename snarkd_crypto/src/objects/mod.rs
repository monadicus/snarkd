// NOTE: placeholder
type Scalar = [u8; 32];
type Group = [u8; 48];
type Field = [u8; 32];

mod address;
mod block;
use block::*;
mod block_header;
use block_header::*;
mod compute_key;
use compute_key::*;
mod graph_key;
mod metadata;
use metadata::*;
mod private_key;
mod signature;
use signature::*;
mod transaction;
use transaction::*;
mod view_key;
