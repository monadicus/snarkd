#![allow(clippy::module_inception)]
// #![cfg_attr(nightly, feature(doc_cfg, external_doc))]
// #![cfg_attr(nightly, warn(missing_docs))]
#![cfg_attr(test, allow(clippy::assertions_on_result_states))]
#![doc = include_str!("../documentation/the_aleo_curves/00_overview.md")]

#[macro_use]
extern crate thiserror;

pub mod bls12_377;

pub mod edwards_bls12;

pub mod errors;
pub use errors::*;

pub mod templates;

#[cfg_attr(test, macro_use)]
pub mod traits;
pub use traits::*;
