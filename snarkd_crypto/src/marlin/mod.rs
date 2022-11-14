//! A crate for the Marlin preprocessing zkSNARK for R1CS.
//!
//! # Note
//!
//! Currently, Marlin only supports R1CS instances where the number of inputs
//! is the same as the number of constraints (i.e., where the constraint
//! matrices are square). Furthermore, Marlin only supports instances where the
//! public inputs are of size one less than a power of 2 (i.e., 2^n - 1).
#![forbid(unsafe_code)]
#![allow(clippy::module_inception)]
#![allow(clippy::type_complexity)]

/// Implements an Algebraic Holographic Proof (AHP) for the R1CS indexed relation.
pub mod ahp;
pub use ahp::*;

pub(crate) mod data_structures;
pub use data_structures::*;

/// Errors.
mod errors;
pub use errors::*;

/// Implements the base Marlin zkSNARK proof system.
mod marlin;
pub use marlin::*;

/// Specifies the Marlin mode.
mod mode;
pub use mode::*;

#[cfg(test)]
pub mod tests;
