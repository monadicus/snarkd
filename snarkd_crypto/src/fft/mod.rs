//! This crate implements functions for manipulating polynomials over finite fields,
//! including FFTs.

use crate::bls12_377::Field;

pub mod domain;
pub use domain::EvaluationDomain;

pub mod evaluations;
pub use evaluations::Evaluations;

pub mod polynomial;
pub use polynomial::{DensePolynomial, Polynomial, SparsePolynomial};

#[cfg(test)]
mod tests;
