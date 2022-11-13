//! A crate for polynomial commitment schemes.
#![forbid(unsafe_code)]
#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

/// The core [\[KZG10\]][kzg] construction.
///
/// [kzg]: http://cacr.uwaterloo.ca/techreports/2010/cacr2010-10.pdf
pub mod kzg10;

/// Polynomial commitment scheme based on the construction in [\[KZG10\]][kzg],
/// modified to obtain batching and to enforce strict
/// degree bounds by following the approach outlined in [[MBKM19,
/// “Sonic”]][sonic] (more precisely, via the variant in
/// [[Gabizon19, “AuroraLight”]][al] that avoids negative G1 powers).
///
/// [kzg]: http://cacr.uwaterloo.ca/techreports/2010/cacr2010-10.pdf
/// [sonic]: https://eprint.iacr.org/2019/099
/// [al]: https://eprint.iacr.org/2019/601
/// [marlin]: https://eprint.iacr.org/2019/1047
pub mod sonic_pc;

/// Errors pertaining to query sets.
pub mod error;
pub use error::*;

/// A random number generator that bypasses some limitations of the Rust borrow
/// checker.
pub mod optional_rng;

#[cfg(test)]
pub mod test_templates;

pub mod powers;
