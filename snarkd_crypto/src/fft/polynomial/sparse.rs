//! A sparse polynomial represented in coefficient form.

use crate::{
    bls12_377::{Field, Scalar},
    fft::{EvaluationDomain, Evaluations, Polynomial},
};
use std::{collections::BTreeMap, fmt};

/// Stores a sparse polynomial in coefficient form.
#[derive(Clone, PartialEq, Eq, Hash, Default)]
#[must_use]
pub struct SparsePolynomial {
    /// The coefficient a_i of `x^i` is stored as (i, a_i) in `self.coeffs`.
    /// the entries in `self.coeffs` are sorted in increasing order of `i`.
    coeffs: BTreeMap<usize, Scalar>,
}

impl fmt::Debug for SparsePolynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for (i, coeff) in self.coeffs.iter().filter(|(_, c)| !c.is_zero()) {
            if *i == 0 {
                write!(f, "\n{:?}", coeff)?;
            } else if *i == 1 {
                write!(f, " + \n{:?} * x", coeff)?;
            } else {
                write!(f, " + \n{:?} * x^{}", coeff, i)?;
            }
        }
        Ok(())
    }
}

impl SparsePolynomial {
    /// Returns the zero polynomial.
    pub fn zero() -> Self {
        Self {
            coeffs: BTreeMap::new(),
        }
    }

    /// Checks if the given polynomial is zero.
    pub fn is_zero(&self) -> bool {
        self.coeffs.is_empty() || self.coeffs.iter().all(|(_, c)| c.is_zero())
    }

    /// Constructs a new polynomial from a list of coefficients.
    pub fn from_coefficients_slice(coeffs: &[(usize, Scalar)]) -> Self {
        Self::from_coefficients(coeffs.iter().copied())
    }

    /// Constructs a new polynomial from a list of coefficients.
    pub fn from_coefficients(coeffs: impl IntoIterator<Item = (usize, Scalar)>) -> Self {
        let coeffs: BTreeMap<_, _> = coeffs.into_iter().filter(|(_, c)| !c.is_zero()).collect();
        Self { coeffs }
    }

    pub fn coeffs(&self) -> impl Iterator<Item = (&usize, &Scalar)> {
        self.coeffs.iter()
    }

    /// Returns the degree of the polynomial.
    pub fn degree(&self) -> usize {
        if self.is_zero() {
            0
        } else {
            let last = self.coeffs.iter().max();
            assert!(last.map_or(false, |(_, c)| !c.is_zero()));
            *last.unwrap().0
        }
    }

    /// Evaluates `self` at the given `point` in the field.
    pub fn evaluate(&self, point: Scalar) -> Scalar {
        if self.is_zero() {
            return Scalar::ZERO;
        }
        let mut total = Scalar::ZERO;
        for (i, c) in &self.coeffs {
            total += *c * point.pow(&[*i as u64]);
        }
        total
    }

    /// Perform a naive n^2 multiplicatoin of `self` by `other`.
    pub fn mul(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            SparsePolynomial::zero()
        } else {
            let mut result = std::collections::BTreeMap::new();
            for (i, self_coeff) in self.coeffs.iter() {
                for (j, other_coeff) in other.coeffs.iter() {
                    let cur_coeff = result.entry(i + j).or_insert_with(|| Scalar::ZERO);
                    *cur_coeff += *self_coeff * other_coeff;
                }
            }
            SparsePolynomial::from_coefficients(result.into_iter())
        }
    }
}

impl SparsePolynomial {
    /// Evaluate `self` over `domain`.
    pub fn evaluate_over_domain_by_ref(&self, domain: EvaluationDomain) -> Evaluations {
        let poly: Polynomial<'_> = self.into();
        Polynomial::evaluate_over_domain(poly, domain)
        // unimplemented!("current implementation does not produce evals in correct order")
    }

    /// Evaluate `self` over `domain`.
    pub fn evaluate_over_domain(self, domain: EvaluationDomain) -> Evaluations {
        let poly: Polynomial<'_> = self.into();
        Polynomial::evaluate_over_domain(poly, domain)
        // unimplemented!("current implementation does not produce evals in correct order")
    }
}
impl core::ops::MulAssign<Scalar> for SparsePolynomial {
    fn mul_assign(&mut self, other: Scalar) {
        if other.is_zero() {
            *self = Self::zero()
        } else {
            for coeff in self.coeffs.values_mut() {
                *coeff *= other;
            }
        }
    }
}

impl<'a> core::ops::Mul<Scalar> for &'a SparsePolynomial {
    type Output = SparsePolynomial;

    fn mul(self, other: Scalar) -> Self::Output {
        let mut result = self.clone();
        result *= other;
        result
    }
}

impl<'a> core::ops::AddAssign<&'a Self> for SparsePolynomial {
    fn add_assign(&mut self, other: &'a Self) {
        let mut result = other.clone();
        for (i, coeff) in self.coeffs.iter() {
            let cur_coeff = result.coeffs.entry(*i).or_insert_with(|| Scalar::ZERO);
            *cur_coeff += coeff;
        }
        *self = Self::from_coefficients(result.coeffs.into_iter().filter(|(_, f)| !f.is_zero()));
    }
}

impl<'a> core::ops::AddAssign<(Scalar, &'a Self)> for SparsePolynomial {
    fn add_assign(&mut self, (f, other): (Scalar, &'a Self)) {
        let mut result = other.clone();
        for (i, coeff) in self.coeffs.iter() {
            let cur_coeff = result.coeffs.entry(*i).or_insert_with(|| Scalar::ZERO);
            *cur_coeff += f * coeff;
        }
        *self = Self::from_coefficients(result.coeffs.into_iter().filter(|(_, f)| !f.is_zero()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fft::DensePolynomial;

    #[test]
    fn evaluate_over_domain() {
        for size in 2..10 {
            let domain_size = 1 << size;
            let domain = EvaluationDomain::new(domain_size).unwrap();
            let two = Scalar::ONE + Scalar::ONE;
            let sparse_poly = SparsePolynomial::from_coefficients(vec![(0, two), (1, two)]);
            let evals1 = sparse_poly.evaluate_over_domain_by_ref(domain);

            let dense_poly: DensePolynomial = sparse_poly.into();
            let evals2 = dense_poly.clone().evaluate_over_domain(domain);
            assert_eq!(evals1.clone().interpolate(), evals2.clone().interpolate());
            assert_eq!(evals1.interpolate(), dense_poly);
            assert_eq!(evals2.interpolate(), dense_poly);
        }
    }
}
