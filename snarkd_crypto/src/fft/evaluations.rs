//! A polynomial represented in evaluations form.

use crate::{
    bls12_377::{Field, Scalar},
    fft::{DensePolynomial, EvaluationDomain},
    utils::*,
};
use itertools::Itertools;
use rayon::prelude::*;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::domain::IFFTPrecomputation;

/// Stores a polynomial in evaluation form.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Evaluations {
    /// The evaluations of a polynomial over the domain `D`
    pub evaluations: Vec<Scalar>,
    #[doc(hidden)]
    domain: EvaluationDomain,
}

impl Evaluations {
    /// Construct `Self` from evaluations and a domain.
    pub fn from_vec_and_domain(evaluations: Vec<Scalar>, domain: EvaluationDomain) -> Self {
        Self {
            evaluations,
            domain,
        }
    }

    /// Interpolate a polynomial from a list of evaluations
    pub fn interpolate_by_ref(&self) -> DensePolynomial {
        DensePolynomial::from_coefficients_vec(self.domain.ifft(&self.evaluations))
    }

    /// Interpolate a polynomial from a list of evaluations
    pub fn interpolate_with_pc_by_ref(&self, pc: &IFFTPrecomputation) -> DensePolynomial {
        let mut evals = self.evaluations.clone();
        evals.resize(self.domain.size(), Scalar::ZERO);
        self.domain.in_order_ifft_in_place_with_pc(&mut evals, pc);
        DensePolynomial::from_coefficients_vec(evals)
    }

    /// Interpolate a polynomial from a list of evaluations
    pub fn interpolate(self) -> DensePolynomial {
        let Self {
            evaluations: mut evals,
            domain,
        } = self;
        domain.ifft_in_place(&mut evals);
        DensePolynomial::from_coefficients_vec(evals)
    }

    /// Interpolate a polynomial from a list of evaluations
    pub fn interpolate_with_pc(self, pc: &IFFTPrecomputation) -> DensePolynomial {
        let Self {
            evaluations: mut evals,
            domain,
        } = self;
        evals.resize(self.domain.size(), Scalar::ZERO);
        domain.in_order_ifft_in_place_with_pc(&mut evals, pc);
        DensePolynomial::from_coefficients_vec(evals)
    }

    pub fn domain(&self) -> EvaluationDomain {
        self.domain
    }

    pub fn evaluate(&self, point: &Scalar) -> Scalar {
        let coeffs = self.domain.evaluate_all_lagrange_coefficients(*point);
        self.evaluate_with_coeffs(&coeffs)
    }

    pub fn evaluate_with_coeffs(&self, lagrange_coefficients_at_point: &[Scalar]) -> Scalar {
        cfg_iter!(self.evaluations)
            .zip_eq(lagrange_coefficients_at_point)
            .map(|(a, b)| *a * b)
            .sum()
    }
}

impl std::ops::Index<usize> for Evaluations {
    type Output = Scalar;

    fn index(&self, index: usize) -> &Scalar {
        &self.evaluations[index]
    }
}

impl<'a, 'b> Mul<&'a Evaluations> for &'b Evaluations {
    type Output = Evaluations;

    #[inline]
    fn mul(self, other: &'a Evaluations) -> Evaluations {
        let mut result = self.clone();
        result *= other;
        result
    }
}

impl<'a> MulAssign<&'a Evaluations> for Evaluations {
    #[inline]
    fn mul_assign(&mut self, other: &'a Evaluations) {
        assert_eq!(self.domain, other.domain, "domains are unequal");
        self.evaluations
            .par_iter_mut()
            .zip_eq(&other.evaluations)
            .for_each(|(a, b)| *a *= b);
    }
}

impl<'a, 'b> Add<&'a Evaluations> for &'b Evaluations {
    type Output = Evaluations;

    #[inline]
    fn add(self, other: &'a Evaluations) -> Evaluations {
        let mut result = self.clone();
        result += other;
        result
    }
}

impl<'a> AddAssign<&'a Evaluations> for Evaluations {
    #[inline]
    fn add_assign(&mut self, other: &'a Evaluations) {
        assert_eq!(self.domain, other.domain, "domains are unequal");
        self.evaluations
            .par_iter_mut()
            .zip_eq(&other.evaluations)
            .for_each(|(a, b)| *a += b);
    }
}

impl<'a, 'b> Sub<&'a Evaluations> for &'b Evaluations {
    type Output = Evaluations;

    #[inline]
    fn sub(self, other: &'a Evaluations) -> Evaluations {
        let mut result = self.clone();
        result -= other;
        result
    }
}

impl<'a> SubAssign<&'a Evaluations> for Evaluations {
    #[inline]
    fn sub_assign(&mut self, other: &'a Evaluations) {
        assert_eq!(self.domain, other.domain, "domains are unequal");
        self.evaluations
            .par_iter_mut()
            .zip_eq(&other.evaluations)
            .for_each(|(a, b)| *a -= b);
    }
}

impl<'a, 'b> Div<&'a Evaluations> for &'b Evaluations {
    type Output = Evaluations;

    #[inline]
    fn div(self, other: &'a Evaluations) -> Evaluations {
        let mut result = self.clone();
        result /= other;
        result
    }
}

impl<'a> DivAssign<&'a Evaluations> for Evaluations {
    #[inline]
    fn div_assign(&mut self, other: &'a Evaluations) {
        assert_eq!(self.domain, other.domain, "domains are unequal");
        self.evaluations
            .par_iter_mut()
            .zip_eq(&other.evaluations)
            .for_each(|(a, b)| *a /= b);
    }
}
