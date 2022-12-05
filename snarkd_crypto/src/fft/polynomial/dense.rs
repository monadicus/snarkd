//! A polynomial represented in coefficient form.

use crate::{
    bls12_377::{Field, Scalar},
    fft::{EvaluationDomain, Evaluations, Polynomial},
};
use std::{
    fmt,
    ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, MulAssign, Neg, Sub, SubAssign},
};

use rayon::prelude::*;

use super::PolyMultiplier;

/// Stores a polynomial in coefficient form.
#[derive(Clone, PartialEq, Eq, Hash, Default)]
#[must_use]
pub struct DensePolynomial {
    /// The coefficient of `x^i` is stored at location `i` in `self.coeffs`.
    pub coeffs: Vec<Scalar>,
}

impl fmt::Debug for DensePolynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for (i, coeff) in self.coeffs.iter().enumerate().filter(|(_, c)| !c.is_zero()) {
            if i == 0 {
                write!(f, "\n{:?}", coeff)?;
            } else if i == 1 {
                write!(f, " + \n{:?} * x", coeff)?;
            } else {
                write!(f, " + \n{:?} * x^{}", coeff, i)?;
            }
        }
        Ok(())
    }
}

impl DensePolynomial {
    /// Returns the zero polynomial.
    pub fn zero() -> Self {
        Self { coeffs: Vec::new() }
    }

    /// Checks if the given polynomial is zero.
    pub fn is_zero(&self) -> bool {
        self.coeffs.is_empty() || self.coeffs.iter().all(|coeff| coeff.is_zero())
    }

    /// Constructs a new polynomial from a list of coefficients.
    pub fn from_coefficients_slice(coeffs: &[Scalar]) -> Self {
        Self::from_coefficients_vec(coeffs.to_vec())
    }

    /// Constructs a new polynomial from a list of coefficients.
    pub fn from_coefficients_vec(mut coeffs: Vec<Scalar>) -> Self {
        // While there are zeros at the end of the coefficient vector, pop them off.
        while coeffs.last().map_or(false, |c| c.is_zero()) {
            coeffs.pop();
        }
        // Check that either the coefficients vec is empty or that the last coeff is non-zero.
        assert!(coeffs.last().map_or(true, |coeff| !coeff.is_zero()));

        Self { coeffs }
    }

    /// Returns the degree of the polynomial.
    pub fn degree(&self) -> usize {
        if self.is_zero() {
            0
        } else {
            assert!(self.coeffs.last().map_or(false, |coeff| !coeff.is_zero()));
            self.coeffs.len() - 1
        }
    }

    /// Evaluates `self` at the given `point` in the field.
    pub fn evaluate(&self, point: Scalar) -> Scalar {
        if self.is_zero() {
            return Scalar::ZERO;
        } else if point.is_zero() {
            return self.coeffs[0];
        }
        let mut powers_of_point = vec![Scalar::ONE];
        let mut cur = point;
        for _ in 0..self.degree() {
            powers_of_point.push(cur);
            cur *= point;
        }
        let zero = Scalar::ZERO;
        let mapping = cfg_iter_mut!(powers_of_point)
            .zip_eq(&self.coeffs)
            .map(|(power, coeff)| *power * coeff);
        crate::cfg_reduce!(mapping, || zero, |a, b| a + b)
    }

    /// Outputs a polynomial of degree `d` where each coefficient is sampled uniformly at random
    /// from the field `F`.
    pub fn rand(d: usize) -> Self {
        let random_coeffs = (0..(d + 1)).map(|_| Scalar::rand()).collect();
        Self::from_coefficients_vec(random_coeffs)
    }

    /// Returns the coefficients of `self`.
    pub fn coeffs(&self) -> &[Scalar] {
        &self.coeffs
    }

    /// Perform a naive n^2 multiplication of `self` by `other`.
    #[cfg(test)]
    fn naive_mul(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            DensePolynomial::zero()
        } else {
            let mut result = vec![Scalar::ZERO; self.degree() + other.degree() + 1];
            for (i, self_coeff) in self.coeffs.iter().enumerate() {
                for (j, other_coeff) in other.coeffs.iter().enumerate() {
                    result[i + j] += *self_coeff * other_coeff;
                }
            }
            DensePolynomial::from_coefficients_vec(result)
        }
    }
}

impl DensePolynomial {
    /// Multiply `self` by the vanishing polynomial for the domain `domain`.
    pub fn mul_by_vanishing_poly(&self, domain: EvaluationDomain) -> DensePolynomial {
        let mut shifted = vec![Scalar::ZERO; domain.size()];
        shifted.extend_from_slice(&self.coeffs);
        cfg_iter_mut!(shifted[..self.coeffs.len()])
            .zip_eq(&self.coeffs)
            .for_each(|(s, c)| *s -= c);
        DensePolynomial::from_coefficients_vec(shifted)
    }

    /// Divide `self` by the vanishing polynomial for the domain `domain`.
    /// Returns the quotient and remainder of the division.
    pub fn divide_by_vanishing_poly(
        &self,
        domain: EvaluationDomain,
    ) -> Option<(DensePolynomial, DensePolynomial)> {
        let self_poly = Polynomial::from(self);
        let vanishing_poly = Polynomial::from(domain.vanishing_polynomial());
        self_poly.divide_with_q_and_r(&vanishing_poly)
    }

    /// Evaluate `self` over `domain`.
    pub fn evaluate_over_domain_by_ref(&self, domain: EvaluationDomain) -> Evaluations {
        let poly: Polynomial<'_> = self.into();
        Polynomial::evaluate_over_domain(poly, domain)
    }

    /// Evaluate `self` over `domain`.
    pub fn evaluate_over_domain(self, domain: EvaluationDomain) -> Evaluations {
        let poly: Polynomial<'_> = self.into();
        Polynomial::evaluate_over_domain(poly, domain)
    }
}

impl From<super::SparsePolynomial> for DensePolynomial {
    fn from(other: super::SparsePolynomial) -> Self {
        let mut result = vec![Scalar::ZERO; other.degree() + 1];
        for (i, coeff) in other.coeffs() {
            result[*i] = *coeff;
        }
        DensePolynomial::from_coefficients_vec(result)
    }
}

impl<'a, 'b> Add<&'a DensePolynomial> for &'b DensePolynomial {
    type Output = DensePolynomial;

    fn add(self, other: &'a DensePolynomial) -> DensePolynomial {
        if self.is_zero() {
            other.clone()
        } else if other.is_zero() {
            self.clone()
        } else if self.degree() >= other.degree() {
            let mut result = self.clone();
            // Zip safety: `result` and `other` could have different lengths.
            cfg_iter_mut!(result.coeffs)
                .zip(&other.coeffs)
                .for_each(|(a, b)| *a += b);
            result
        } else {
            let mut result = other.clone();
            // Zip safety: `result` and `other` could have different lengths.
            cfg_iter_mut!(result.coeffs)
                .zip(&self.coeffs)
                .for_each(|(a, b)| *a += b);
            // If the leading coefficient ends up being zero, pop it off.
            while result.coeffs.last().unwrap().is_zero() {
                result.coeffs.pop();
            }
            result
        }
    }
}

impl<'a> AddAssign<&'a DensePolynomial> for DensePolynomial {
    fn add_assign(&mut self, other: &'a DensePolynomial) {
        if self.is_zero() {
            self.coeffs.clear();
            self.coeffs.extend_from_slice(&other.coeffs);
        } else if other.is_zero() {
            // return
        } else if self.degree() >= other.degree() {
            // Zip safety: `self` and `other` could have different lengths.
            cfg_iter_mut!(self.coeffs)
                .zip(&other.coeffs)
                .for_each(|(a, b)| *a += b);
        } else {
            // Add the necessary number of zero coefficients.
            self.coeffs.resize(other.coeffs.len(), Scalar::ZERO);
            // Zip safety: `self` and `other` have the same length.
            cfg_iter_mut!(self.coeffs)
                .zip(&other.coeffs)
                .for_each(|(a, b)| *a += b);
            // If the leading coefficient ends up being zero, pop it off.
            while self.coeffs.last().unwrap().is_zero() {
                self.coeffs.pop();
            }
        }
    }
}

impl<'a> AddAssign<&'a Polynomial<'a>> for DensePolynomial {
    fn add_assign(&mut self, other: &'a Polynomial) {
        match other {
            Polynomial::Sparse(p) => *self += &Self::from(p.clone().into_owned()),
            Polynomial::Dense(p) => *self += p.as_ref(),
        }
    }
}

impl<'a> AddAssign<(Scalar, &'a Polynomial<'a>)> for DensePolynomial {
    fn add_assign(&mut self, (f, other): (Scalar, &'a Polynomial)) {
        match other {
            Polynomial::Sparse(p) => *self += (f, &Self::from(p.clone().into_owned())),
            Polynomial::Dense(p) => *self += (f, p.as_ref()),
        }
    }
}

impl<'a> AddAssign<(Scalar, &'a DensePolynomial)> for DensePolynomial {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn add_assign(&mut self, (f, other): (Scalar, &'a DensePolynomial)) {
        if self.is_zero() {
            self.coeffs.clear();
            self.coeffs.extend_from_slice(&other.coeffs);
            self.coeffs.iter_mut().for_each(|c| *c *= &f);
        } else if other.is_zero() {
            // return
        } else if self.degree() >= other.degree() {
            // Zip safety: `self` and `other` could have different lengths.
            cfg_iter_mut!(self.coeffs)
                .zip(&other.coeffs)
                .for_each(|(a, b)| {
                    *a += f * b;
                });
        } else {
            // Add the necessary number of zero coefficients.
            self.coeffs.resize(other.coeffs.len(), Scalar::ZERO);
            // Zip safety: `self` and `other` have the same length after the resize.
            cfg_iter_mut!(self.coeffs)
                .zip(&other.coeffs)
                .for_each(|(a, b)| {
                    *a += f * b;
                });
            // If the leading coefficient ends up being zero, pop it off.
            while self.coeffs.last().unwrap().is_zero() {
                self.coeffs.pop();
            }
        }
    }
}

impl Neg for DensePolynomial {
    type Output = DensePolynomial;

    #[inline]
    fn neg(mut self) -> DensePolynomial {
        for coeff in &mut self.coeffs {
            *coeff = -*coeff;
        }
        self
    }
}

impl<'a, 'b> Sub<&'a DensePolynomial> for &'b DensePolynomial {
    type Output = DensePolynomial;

    #[inline]
    fn sub(self, other: &'a DensePolynomial) -> DensePolynomial {
        if self.is_zero() {
            let mut result = other.clone();
            for coeff in &mut result.coeffs {
                *coeff = -(*coeff);
            }
            result
        } else if other.is_zero() {
            self.clone()
        } else if self.degree() >= other.degree() {
            let mut result = self.clone();
            // Zip safety: `result` and `other` could have different degrees.
            cfg_iter_mut!(result.coeffs)
                .zip(&other.coeffs)
                .for_each(|(a, b)| *a -= b);
            result
        } else {
            let mut result = self.clone();
            result.coeffs.resize(other.coeffs.len(), Scalar::ZERO);
            // Zip safety: `result` and `other` have the same length after the resize.
            cfg_iter_mut!(result.coeffs)
                .zip(&other.coeffs)
                .for_each(|(a, b)| {
                    *a -= b;
                });
            if !result.is_zero() {
                // If the leading coefficient ends up being zero, pop it off.
                while result.coeffs.last().unwrap().is_zero() {
                    result.coeffs.pop();
                }
            }

            result
        }
    }
}

impl<'a> SubAssign<&'a DensePolynomial> for DensePolynomial {
    #[inline]
    fn sub_assign(&mut self, other: &'a DensePolynomial) {
        if self.is_zero() {
            self.coeffs.resize(other.coeffs.len(), Scalar::ZERO);
            for (i, coeff) in other.coeffs.iter().enumerate() {
                self.coeffs[i] -= coeff;
            }
        } else if other.is_zero() {
            // return
        } else if self.degree() >= other.degree() {
            // Zip safety: self and other could have different lengths.
            cfg_iter_mut!(self.coeffs)
                .zip(&other.coeffs)
                .for_each(|(a, b)| *a -= b);
        } else {
            // Add the necessary number of zero coefficients.
            self.coeffs.resize(other.coeffs.len(), Scalar::ZERO);
            // Zip safety: self and other have the same length after the resize.
            cfg_iter_mut!(self.coeffs)
                .zip(&other.coeffs)
                .for_each(|(a, b)| *a -= b);
            // If the leading coefficient ends up being zero, pop it off.
            while self.coeffs.last().unwrap().is_zero() {
                self.coeffs.pop();
            }
        }
    }
}

impl<'a> AddAssign<&'a super::SparsePolynomial> for DensePolynomial {
    #[inline]
    fn add_assign(&mut self, other: &'a super::SparsePolynomial) {
        if self.degree() < other.degree() {
            self.coeffs.resize(other.degree() + 1, Scalar::ZERO);
        }
        for (i, b) in other.coeffs() {
            self.coeffs[*i] += b;
        }
        // If the leading coefficient ends up being zero, pop it off.
        while let Some(true) = self.coeffs.last().map(|c| c.is_zero()) {
            self.coeffs.pop();
        }
    }
}

impl<'a> Sub<&'a super::SparsePolynomial> for DensePolynomial {
    type Output = Self;

    #[inline]
    fn sub(mut self, other: &'a super::SparsePolynomial) -> Self::Output {
        if self.degree() < other.degree() {
            self.coeffs.resize(other.degree() + 1, Scalar::ZERO);
        }
        for (i, b) in other.coeffs() {
            self.coeffs[*i] -= b;
        }
        // If the leading coefficient ends up being zero, pop it off.
        while let Some(true) = self.coeffs.last().map(|c| c.is_zero()) {
            self.coeffs.pop();
        }
        self
    }
}

impl<'a, 'b> Div<&'a DensePolynomial> for &'b DensePolynomial {
    type Output = DensePolynomial;

    #[inline]
    fn div(self, divisor: &'a DensePolynomial) -> DensePolynomial {
        let a: Polynomial = self.into();
        let b: Polynomial = divisor.into();
        a.divide_with_q_and_r(&b).expect("division failed").0
    }
}

/// Performs O(nlogn) multiplication of polynomials if F is smooth.
impl<'a, 'b> Mul<&'a DensePolynomial> for &'b DensePolynomial {
    type Output = DensePolynomial;

    #[inline]
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, other: &'a DensePolynomial) -> DensePolynomial {
        if self.is_zero() || other.is_zero() {
            DensePolynomial::zero()
        } else {
            let mut m = PolyMultiplier::new();
            m.add_polynomial_ref(self, "");
            m.add_polynomial_ref(other, "");
            m.multiply().unwrap()
        }
    }
}

/// Multiplies `self` by `other: F`.
impl Mul<Scalar> for DensePolynomial {
    type Output = Self;

    #[inline]
    fn mul(mut self, other: Scalar) -> Self {
        self.iter_mut().for_each(|c| *c *= other);
        self
    }
}

/// Multiplies `self` by `other: F`.
impl<'a> Mul<Scalar> for &'a DensePolynomial {
    type Output = DensePolynomial;

    #[inline]
    fn mul(self, other: Scalar) -> Self::Output {
        let result = self.clone();
        result * other
    }
}

/// Multiplies `self` by `other: F`.
impl MulAssign<Scalar> for DensePolynomial {
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul_assign(&mut self, other: Scalar) {
        cfg_iter_mut!(self).for_each(|c| *c *= other);
    }
}

/// Multiplies `self` by `other: F`.
impl std::iter::Sum for DensePolynomial {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(DensePolynomial::zero(), |a, b| &a + &b)
    }
}

impl Deref for DensePolynomial {
    type Target = [Scalar];

    fn deref(&self) -> &[Scalar] {
        &self.coeffs
    }
}

impl DerefMut for DensePolynomial {
    fn deref_mut(&mut self) -> &mut [Scalar] {
        &mut self.coeffs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_polynomials_random() {
        for degree in 0..70 {
            let p = DensePolynomial::rand(degree);
            let p_double = &p + &p;
            let p_quad = &p_double + &p_double;
            assert_eq!(&(&(&p + &p) + &p) + &p, p_quad);
        }
    }

    #[test]
    fn add_polynomials() {
        for a_degree in 0..70 {
            for b_degree in 0..70 {
                let p1 = DensePolynomial::rand(a_degree);
                let p2 = DensePolynomial::rand(b_degree);
                let res1 = &p1 + &p2;
                let res2 = &p2 + &p1;
                assert_eq!(res1, res2);
            }
        }
    }

    #[test]
    fn add_polynomials_with_mul() {
        for a_degree in 0..70 {
            for b_degree in 0..70 {
                let mut p1 = DensePolynomial::rand(a_degree);
                let p2 = DensePolynomial::rand(b_degree);
                let f = Scalar::rand();
                let f_p2 = DensePolynomial::from_coefficients_vec(
                    p2.coeffs.iter().map(|c| f * c).collect(),
                );
                let res2 = &f_p2 + &p1;
                p1 += (f, &p2);
                let res1 = p1;
                assert_eq!(res1, res2);
            }
        }
    }

    #[test]
    fn sub_polynomials() {
        let p1 = DensePolynomial::rand(5);
        let p2 = DensePolynomial::rand(3);
        let res1 = &p1 - &p2;
        let res2 = &p2 - &p1;
        assert_eq!(
            &res1 + &p2,
            p1,
            "Subtraction should be inverse of addition!"
        );
        assert_eq!(res1, -res2, "p2 - p1 = -(p1 - p2)");
    }

    #[test]
    fn divide_polynomials_fixed() {
        let dividend = DensePolynomial::from_coefficients_slice(&[
            Scalar::from(4),
            Scalar::from(8),
            Scalar::from(5),
            Scalar::from(1),
        ]);
        let divisor = DensePolynomial::from_coefficients_slice(&[Scalar::ONE, Scalar::ONE]); // Construct a monic linear polynomial.
        let result = &dividend / &divisor;
        let expected_result = DensePolynomial::from_coefficients_slice(&[
            Scalar::from(4),
            Scalar::from(4),
            Scalar::from(1),
        ]);
        assert_eq!(expected_result, result);
    }

    #[test]
    #[allow(clippy::needless_borrow)]
    fn divide_polynomials_random() {
        for a_degree in 0..70 {
            for b_degree in 0..70 {
                let dividend = DensePolynomial::rand(a_degree);
                let divisor = DensePolynomial::rand(b_degree);
                if let Some((quotient, remainder)) =
                    Polynomial::divide_with_q_and_r(&(&dividend).into(), &(&divisor).into())
                {
                    assert_eq!(dividend, &(&divisor * &quotient) + &remainder)
                }
            }
        }
    }

    #[test]
    fn evaluate_polynomials() {
        for a_degree in 0..70 {
            let p = DensePolynomial::rand(a_degree);
            let point: Scalar = Scalar::from(10u64);
            let mut total = Scalar::ZERO;
            for (i, coeff) in p.coeffs.iter().enumerate() {
                total += point.pow(&[i as u64]) * coeff;
            }
            assert_eq!(p.evaluate(point), total);
        }
    }

    #[test]
    fn mul_polynomials_random() {
        for a_degree in 0..70 {
            for b_degree in 0..70 {
                // dbg!(a_degree);
                // dbg!(b_degree);
                let a = DensePolynomial::rand(a_degree);
                let b = DensePolynomial::rand(b_degree);
                assert_eq!(&a * &b, a.naive_mul(&b))
            }
        }
    }

    #[test]
    fn mul_by_vanishing_poly() {
        for size in 1..10 {
            let domain = EvaluationDomain::new(1 << size).unwrap();
            for degree in 0..70 {
                let p = DensePolynomial::rand(degree);
                let ans1 = p.mul_by_vanishing_poly(domain);
                let ans2 = &p * &domain.vanishing_polynomial().into();
                assert_eq!(ans1, ans2);
            }
        }
    }
}
