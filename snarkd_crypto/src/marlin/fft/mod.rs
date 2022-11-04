//! This crate implements functions for manipulating polynomials over finite fields,
//! including FFTs.

pub mod domain;
pub use domain::EvaluationDomain;

pub mod evaluations;
pub use evaluations::Evaluations;

pub mod polynomial;
pub use polynomial::{DensePolynomial, Polynomial, SparsePolynomial};

#[cfg(test)]
mod tests;

/// Types that can be FFT-ed must implement this trait.
pub trait DomainCoeff<F: FftField>:
    Copy
    + Send
    + Sync
    + core::ops::Add<Output = Self>
    + core::ops::Sub<Output = Self>
    + core::ops::AddAssign
    + core::ops::SubAssign
    + snarkvm_fields::Zero
    + core::ops::MulAssign<F>
{
}

impl<T, F> DomainCoeff<F> for T
where
    F: FftField,
    T: Copy
        + Send
        + Sync
        + snarkvm_fields::Zero
        + core::ops::AddAssign
        + core::ops::SubAssign
        + core::ops::MulAssign<F>
        + core::ops::Add<Output = Self>
        + core::ops::Sub<Output = Self>,
{
}
