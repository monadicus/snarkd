#![allow(clippy::module_inception)]
// #![cfg_attr(nightly, feature(doc_cfg, external_doc))]
// #![cfg_attr(nightly, warn(missing_docs))]
#![cfg_attr(test, allow(clippy::assertions_on_result_states))]
#![doc = include_str!("../documentation/the_aleo_curves/00_overview.md")]
#![cfg_attr(nightly, doc = include_str!("../documentation/the_aleo_curves/02_bls12-377.md"))]

pub mod edwards_bls12;

pub mod errors;
pub use errors::*;

pub mod templates;

pub mod field;
pub use field::*;

pub mod fr;
#[doc(inline)]
pub use fr::*;

pub mod fq;
#[doc(inline)]
pub use fq::*;

pub mod fq2;
#[doc(inline)]
pub use fq2::*;

pub mod fq6;
#[doc(inline)]
pub use fq6::*;

pub mod fq12;
#[doc(inline)]
pub use fq12::*;

pub mod g1;
#[doc(inline)]
pub use g1::*;

pub mod g2;
#[doc(inline)]
pub use g2::*;

pub mod parameters;
#[doc(inline)]
pub use parameters::*;

#[cfg(test)]
mod tests;
