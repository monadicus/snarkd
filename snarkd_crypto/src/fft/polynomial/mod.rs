//! Work with sparse and dense polynomials.

use crate::{
    bls12_377::{Field, Scalar},
    fft::{EvaluationDomain, Evaluations},
};
use rayon::prelude::*;
use std::{borrow::Cow, convert::TryInto};
use Polynomial::*;

mod dense;
pub use dense::DensePolynomial;

mod sparse;
pub use sparse::SparsePolynomial;

mod multiplier;
pub use multiplier::*;

/// Represents either a sparse polynomial or a dense one.
#[derive(Clone, Debug)]
pub enum Polynomial<'a> {
    /// Represents the case where `self` is a sparse polynomial
    Sparse(Cow<'a, SparsePolynomial>),
    /// Represents the case where `self` is a dense polynomial
    Dense(Cow<'a, DensePolynomial>),
}

impl From<DensePolynomial> for Polynomial<'_> {
    fn from(other: DensePolynomial) -> Self {
        Dense(Cow::Owned(other))
    }
}

impl<'a> From<&'a DensePolynomial> for Polynomial<'a> {
    fn from(other: &'a DensePolynomial) -> Self {
        Dense(Cow::Borrowed(other))
    }
}

impl From<SparsePolynomial> for Polynomial<'_> {
    fn from(other: SparsePolynomial) -> Self {
        Sparse(Cow::Owned(other))
    }
}

impl<'a> From<&'a SparsePolynomial> for Polynomial<'a> {
    fn from(other: &'a SparsePolynomial) -> Self {
        Sparse(Cow::Borrowed(other))
    }
}

#[allow(clippy::from_over_into)]
impl Into<DensePolynomial> for Polynomial<'_> {
    fn into(self) -> DensePolynomial {
        match self {
            Dense(p) => p.into_owned(),
            Sparse(p) => p.into_owned().into(),
        }
    }
}

impl TryInto<SparsePolynomial> for Polynomial<'_> {
    type Error = ();

    fn try_into(self) -> Result<SparsePolynomial, ()> {
        match self {
            Sparse(p) => Ok(p.into_owned()),
            _ => Err(()),
        }
    }
}

impl<'a> Polynomial<'a> {
    /// Checks if the given polynomial is zero.
    pub fn is_zero(&self) -> bool {
        match self {
            Sparse(s) => s.is_zero(),
            Dense(d) => d.is_zero(),
        }
    }

    /// Return the degree of `self.
    pub fn degree(&self) -> usize {
        match self {
            Sparse(s) => s.degree(),
            Dense(d) => d.degree(),
        }
    }

    #[inline]
    pub fn leading_coefficient(&self) -> Option<&Scalar> {
        match self {
            Sparse(p) => p.coeffs().last().map(|(_, c)| c),
            Dense(p) => p.last(),
        }
    }

    #[inline]
    pub fn as_dense(&self) -> Option<&DensePolynomial> {
        match self {
            Dense(p) => Some(p.as_ref()),
            _ => None,
        }
    }

    #[inline]
    pub fn as_dense_mut(&mut self) -> Option<&mut DensePolynomial> {
        match self {
            Dense(p) => Some(p.to_mut()),
            _ => None,
        }
    }

    #[inline]
    pub fn as_sparse(&self) -> Option<&SparsePolynomial> {
        match self {
            Sparse(p) => Some(p.as_ref()),
            _ => None,
        }
    }

    #[inline]
    pub fn into_dense(&self) -> DensePolynomial {
        self.clone().into()
    }

    #[inline]
    pub fn evaluate(&self, point: Scalar) -> Scalar {
        match self {
            Sparse(p) => p.evaluate(point),
            Dense(p) => p.evaluate(point),
        }
    }

    pub fn coeffs(&'a self) -> Box<dyn Iterator<Item = (usize, &'a Scalar)> + 'a> {
        match self {
            Sparse(p) => Box::new(p.coeffs().map(|(c, f)| (*c, f))),
            Dense(p) => Box::new(p.coeffs.iter().enumerate()),
        }
    }

    /// Divide self by another (sparse or dense) polynomial, and returns the quotient and remainder.
    pub fn divide_with_q_and_r(
        &self,
        divisor: &Self,
    ) -> Option<(DensePolynomial, DensePolynomial)> {
        if self.is_zero() {
            Some((DensePolynomial::zero(), DensePolynomial::zero()))
        } else if divisor.is_zero() {
            panic!("Dividing by zero polynomial")
        } else if self.degree() < divisor.degree() {
            Some((DensePolynomial::zero(), self.clone().into()))
        } else {
            // Now we know that self.degree() >= divisor.degree();
            let mut quotient = vec![Scalar::ZERO; self.degree() - divisor.degree() + 1];
            let mut remainder: DensePolynomial = self.clone().into();
            // Can unwrap here because we know self is not zero.
            let divisor_leading_inv = divisor.leading_coefficient().unwrap().inverse().unwrap();
            while !remainder.is_zero() && remainder.degree() >= divisor.degree() {
                let cur_q_coeff = *remainder.coeffs.last().unwrap() * divisor_leading_inv;
                let cur_q_degree = remainder.degree() - divisor.degree();
                quotient[cur_q_degree] = cur_q_coeff;

                if let Sparse(p) = divisor {
                    for (i, div_coeff) in p.coeffs() {
                        remainder[cur_q_degree + i] -= &(cur_q_coeff * div_coeff);
                    }
                } else if let Dense(p) = divisor {
                    for (i, div_coeff) in p.iter().enumerate() {
                        remainder[cur_q_degree + i] -= &(cur_q_coeff * div_coeff);
                    }
                }

                while let Some(true) = remainder.coeffs.last().map(|c| c.is_zero()) {
                    remainder.coeffs.pop();
                }
            }
            Some((DensePolynomial::from_coefficients_vec(quotient), remainder))
        }
    }
}
impl Polynomial<'_> {
    /// Construct `Evaluations` by evaluating a polynomial over the domain `domain`.
    pub fn evaluate_over_domain(poly: impl Into<Self>, domain: EvaluationDomain) -> Evaluations {
        let poly = poly.into();
        poly.eval_over_domain_helper(domain)
    }

    fn eval_over_domain_helper(self, domain: EvaluationDomain) -> Evaluations {
        match self {
            Sparse(Cow::Borrowed(s)) => {
                let evals = domain.elements().map(|elem| s.evaluate(elem)).collect();
                Evaluations::from_vec_and_domain(evals, domain)
            }
            Sparse(Cow::Owned(s)) => {
                let evals = domain.elements().map(|elem| s.evaluate(elem)).collect();
                Evaluations::from_vec_and_domain(evals, domain)
            }
            Dense(Cow::Borrowed(d)) => {
                if d.degree() >= domain.size() {
                    d.coeffs
                        .chunks(domain.size())
                        .map(|d| Evaluations::from_vec_and_domain(domain.fft(d), domain))
                        .fold(
                            Evaluations::from_vec_and_domain(
                                vec![Scalar::ZERO; domain.size()],
                                domain,
                            ),
                            |mut acc, e| {
                                cfg_iter_mut!(acc.evaluations)
                                    .zip(e.evaluations)
                                    .for_each(|(a, e)| *a += e);
                                acc
                            },
                        )
                } else {
                    Evaluations::from_vec_and_domain(domain.fft(&d.coeffs), domain)
                }
            }
            Dense(Cow::Owned(mut d)) => {
                if d.degree() >= domain.size() {
                    d.coeffs
                        .chunks(domain.size())
                        .map(|d| Evaluations::from_vec_and_domain(domain.fft(d), domain))
                        .fold(
                            Evaluations::from_vec_and_domain(
                                vec![Scalar::ZERO; domain.size()],
                                domain,
                            ),
                            |mut acc, e| {
                                cfg_iter_mut!(acc.evaluations)
                                    .zip(e.evaluations)
                                    .for_each(|(a, e)| *a += e);
                                acc
                            },
                        )
                } else {
                    domain.fft_in_place(&mut d.coeffs);
                    Evaluations::from_vec_and_domain(d.coeffs, domain)
                }
            }
        }
    }
}
