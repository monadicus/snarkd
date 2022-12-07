#![allow(clippy::derive_partial_eq_without_eq, clippy::module_inception)]

include!(concat!(env!("OUT_DIR"), "/_includes.rs"));
pub use ir::*;
pub use opcode::*;
