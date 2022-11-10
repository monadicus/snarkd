use super::PolynomialLabel;
use crate::{
    bls12_377::Scalar,
    fft::{
        DensePolynomial, EvaluationDomain, Evaluations as EvaluationsOnDomain, Polynomial,
        SparsePolynomial,
    },
    Field,
};
use hashbrown::HashMap;
use std::borrow::Cow;

#[cfg(not(feature = "parallel"))]
use itertools::Itertools;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolynomialInfo {
    label: PolynomialLabel,
    degree_bound: Option<usize>,
    hiding_bound: Option<usize>,
}

impl PolynomialInfo {
    /// Construct a new labeled polynomial by consuming `polynomial`.
    pub fn new(
        label: PolynomialLabel,
        degree_bound: Option<usize>,
        hiding_bound: Option<usize>,
    ) -> Self {
        Self {
            label,
            degree_bound,
            hiding_bound,
        }
    }

    /// Return the label for `self`.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Retrieve the degree bound in `self`.
    pub fn degree_bound(&self) -> Option<usize> {
        self.degree_bound
    }

    /// Retrieve whether the polynomial in `self` should be hidden.
    pub fn is_hiding(&self) -> bool {
        self.hiding_bound.is_some()
    }

    /// Retrieve the hiding bound for the polynomial in `self`.
    pub fn hiding_bound(&self) -> Option<usize> {
        self.hiding_bound
    }
}

/// A polynomial along with information about its degree bound (if any), and the
/// maximum number of queries that will be made to it. This latter number determines
/// the amount of protection that will be provided to a commitment for this polynomial.
#[derive(Debug, Clone)]
pub struct LabeledPolynomial {
    pub info: PolynomialInfo,
    pub polynomial: Polynomial<'static>,
}

impl core::ops::Deref for LabeledPolynomial {
    type Target = Polynomial<'static>;

    fn deref(&self) -> &Self::Target {
        &self.polynomial
    }
}

impl LabeledPolynomial {
    /// Construct a new labeled polynomial by consuming `polynomial`.
    pub fn new(
        label: PolynomialLabel,
        polynomial: impl Into<Polynomial<'static>>,
        degree_bound: Option<usize>,
        hiding_bound: Option<usize>,
    ) -> Self {
        let info = PolynomialInfo::new(label, degree_bound, hiding_bound);
        Self {
            info,
            polynomial: polynomial.into(),
        }
    }

    pub fn info(&self) -> &PolynomialInfo {
        &self.info
    }

    /// Return the label for `self`.
    pub fn label(&self) -> &str {
        &self.info.label
    }

    /// Retrieve the polynomial from `self`.
    pub fn polynomial(&self) -> &Polynomial {
        &self.polynomial
    }

    /// Retrieve a mutable reference to the enclosed polynomial.
    pub fn polynomial_mut(&mut self) -> &mut Polynomial<'static> {
        &mut self.polynomial
    }

    /// Evaluate the polynomial in `self`.
    pub fn evaluate(&self, point: Scalar) -> Scalar {
        self.polynomial.evaluate(point)
    }

    /// Retrieve the degree bound in `self`.
    pub fn degree_bound(&self) -> Option<usize> {
        self.info.degree_bound
    }

    /// Retrieve whether the polynomial in `self` should be hidden.
    pub fn is_hiding(&self) -> bool {
        self.info.hiding_bound.is_some()
    }

    /// Retrieve the hiding bound for the polynomial in `self`.
    pub fn hiding_bound(&self) -> Option<usize> {
        self.info.hiding_bound
    }
}

/////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct LabeledPolynomialWithBasis<'a> {
    pub info: PolynomialInfo,
    pub polynomial: Vec<(Scalar, PolynomialWithBasis<'a>)>,
}

impl<'a> LabeledPolynomialWithBasis<'a> {
    /// Construct a new labeled polynomial by consuming `polynomial`.
    pub fn new_monomial_basis(
        label: PolynomialLabel,
        polynomial: &'a Polynomial,
        degree_bound: Option<usize>,
        hiding_bound: Option<usize>,
    ) -> Self {
        let polynomial = PolynomialWithBasis::new_monomial_basis_ref(polynomial, degree_bound);
        let info = PolynomialInfo::new(label, degree_bound, hiding_bound);
        Self {
            info,
            polynomial: vec![(Scalar::one(), polynomial)],
        }
    }

    /// Construct a new labeled polynomial by consuming `polynomial`.
    pub fn new_linear_combination(
        label: PolynomialLabel,
        polynomial: Vec<(Scalar, PolynomialWithBasis<'a>)>,
        hiding_bound: Option<usize>,
    ) -> Self {
        let info = PolynomialInfo::new(label, None, hiding_bound);
        Self { info, polynomial }
    }

    pub fn new_lagrange_basis(
        label: PolynomialLabel,
        polynomial: EvaluationsOnDomain,
        hiding_bound: Option<usize>,
    ) -> Self {
        let polynomial = PolynomialWithBasis::new_lagrange_basis(polynomial);
        let info = PolynomialInfo::new(label, None, hiding_bound);
        Self {
            info,
            polynomial: vec![(Scalar::one(), polynomial)],
        }
    }

    pub fn new_lagrange_basis_ref(
        label: PolynomialLabel,
        polynomial: &'a EvaluationsOnDomain,
        hiding_bound: Option<usize>,
    ) -> Self {
        let polynomial = PolynomialWithBasis::new_lagrange_basis_ref(polynomial);
        let info = PolynomialInfo::new(label, None, hiding_bound);
        Self {
            info,
            polynomial: vec![(Scalar::one(), polynomial)],
        }
    }

    /// Return the label for `self`.
    pub fn label(&self) -> &str {
        &self.info.label
    }

    /// Return the information about the label, degree bound, and hiding bound of `self`.
    pub fn info(&self) -> &PolynomialInfo {
        &self.info
    }

    pub fn degree(&self) -> usize {
        self.polynomial
            .iter()
            .map(|(_, p)| match p {
                PolynomialWithBasis::Lagrange { evaluations } => evaluations.domain().size() - 1,
                PolynomialWithBasis::Monomial { polynomial, .. } => polynomial.degree(),
            })
            .max()
            .unwrap_or(0)
    }

    /// Evaluate the polynomial in `self`.
    pub fn evaluate(&self, point: Scalar) -> Scalar {
        self.polynomial
            .iter()
            .map(|(coeff, p)| p.evaluate(point) * coeff)
            .sum()
    }

    /// Compute a linear combination of the terms in `self.polynomial`, producing an iterator
    /// over polynomials of the same time.
    pub fn sum(&self) -> impl Iterator<Item = PolynomialWithBasis<'a>> {
        if self.polynomial.len() == 1 && self.polynomial[0].0.is_one() {
            vec![self.polynomial[0].1.clone()].into_iter()
        } else {
            use PolynomialWithBasis::*;
            let mut lagrange_polys = HashMap::<usize, Vec<_>>::new();
            let mut dense_polys = HashMap::<_, DensePolynomial>::new();
            let mut sparse_poly = SparsePolynomial::zero();
            // We have sets of polynomials divided along three critera:
            // 1. All `Lagrange` polynomials are in the set corresponding to their domain.
            // 2. All `Dense` polynomials are in the set corresponding to their degree bound.
            // 3. All `Sparse` polynomials are in the set corresponding to their degree bound.
            for (c, poly) in self.polynomial.iter() {
                match poly {
                    Monomial {
                        polynomial,
                        degree_bound,
                    } => {
                        use Polynomial::*;
                        match polynomial.as_ref() {
                            Dense(p) => {
                                if let Some(e) = dense_polys.get_mut(degree_bound) {
                                    // Zip safety: `p` could be of smaller degree than `e` (or vice versa),
                                    // so it's okay to just use `zip` here.
                                    cfg_iter_mut!(e)
                                        .zip(&p.coeffs)
                                        .for_each(|(e, f)| *e += *c * f)
                                } else {
                                    let mut e: DensePolynomial = p.clone().into_owned();
                                    cfg_iter_mut!(e).for_each(|e| *e *= c);
                                    dense_polys.insert(degree_bound, e);
                                }
                            }
                            Sparse(p) => sparse_poly += (*c, p.as_ref()),
                        }
                    }
                    Lagrange { evaluations } => {
                        let domain = evaluations.domain().size();
                        if let Some(e) = lagrange_polys.get_mut(&domain) {
                            cfg_iter_mut!(e)
                                .zip_eq(&evaluations.evaluations)
                                .for_each(|(e, f)| *e += *c * f)
                        } else {
                            let mut e = evaluations.clone().into_owned().evaluations;
                            cfg_iter_mut!(e).for_each(|e| *e *= c);
                            lagrange_polys.insert(domain, e);
                        }
                    }
                }
            }
            let sparse_poly = Polynomial::from(sparse_poly);
            let sparse_poly = Monomial {
                polynomial: Cow::Owned(sparse_poly),
                degree_bound: None,
            };
            lagrange_polys
                .into_iter()
                .map(|(k, v)| {
                    let domain = EvaluationDomain::new(k).unwrap();
                    Lagrange {
                        evaluations: Cow::Owned(EvaluationsOnDomain::from_vec_and_domain(
                            v, domain,
                        )),
                    }
                })
                .chain({
                    dense_polys.into_iter().map(|(degree_bound, p)| {
                        PolynomialWithBasis::new_dense_monomial_basis(p, *degree_bound)
                    })
                })
                .chain([sparse_poly])
                .collect::<Vec<_>>()
                .into_iter()
        }
    }

    /// Retrieve the degree bound in `self`.
    pub fn degree_bound(&self) -> Option<usize> {
        self.polynomial
            .iter()
            .filter_map(|(_, p)| match p {
                PolynomialWithBasis::Monomial { degree_bound, .. } => *degree_bound,
                _ => None,
            })
            .max()
    }

    /// Retrieve whether the polynomial in `self` should be hidden.
    pub fn is_hiding(&self) -> bool {
        self.info.hiding_bound.is_some()
    }

    /// Retrieve the hiding bound for the polynomial in `self`.
    pub fn hiding_bound(&self) -> Option<usize> {
        self.info.hiding_bound
    }
}

impl<'a> From<&'a LabeledPolynomial> for LabeledPolynomialWithBasis<'a> {
    fn from(other: &'a LabeledPolynomial) -> Self {
        let polynomial = PolynomialWithBasis::Monomial {
            polynomial: Cow::Borrowed(other.polynomial()),
            degree_bound: other.degree_bound(),
        };
        Self {
            info: other.info.clone(),
            polynomial: vec![(Scalar::one(), polynomial)],
        }
    }
}

impl<'a> From<LabeledPolynomial> for LabeledPolynomialWithBasis<'a> {
    fn from(other: LabeledPolynomial) -> Self {
        let polynomial = PolynomialWithBasis::Monomial {
            polynomial: Cow::Owned(other.polynomial),
            degree_bound: other.info.degree_bound,
        };
        Self {
            info: other.info.clone(),
            polynomial: vec![(Scalar::one(), polynomial)],
        }
    }
}

#[derive(Debug, Clone)]
pub enum PolynomialWithBasis<'a> {
    /// A polynomial in monomial basis, along with information about
    /// its degree bound (if any).
    Monomial {
        polynomial: Cow<'a, Polynomial<'a>>,
        degree_bound: Option<usize>,
    },

    /// A polynomial in Lagrange basis, along with information about
    /// its degree bound (if any).
    Lagrange {
        evaluations: Cow<'a, EvaluationsOnDomain>,
    },
}

impl<'a> PolynomialWithBasis<'a> {
    pub fn new_monomial_basis_ref(polynomial: &'a Polynomial, degree_bound: Option<usize>) -> Self {
        Self::Monomial {
            polynomial: Cow::Borrowed(polynomial),
            degree_bound,
        }
    }

    pub fn new_monomial_basis(polynomial: Polynomial<'a>, degree_bound: Option<usize>) -> Self {
        Self::Monomial {
            polynomial: Cow::Owned(polynomial),
            degree_bound,
        }
    }

    pub fn new_dense_monomial_basis_ref(
        polynomial: &'a DensePolynomial,
        degree_bound: Option<usize>,
    ) -> Self {
        let polynomial = Polynomial::Dense(Cow::Borrowed(polynomial));
        Self::Monomial {
            polynomial: Cow::Owned(polynomial),
            degree_bound,
        }
    }

    pub fn new_dense_monomial_basis(
        polynomial: DensePolynomial,
        degree_bound: Option<usize>,
    ) -> Self {
        let polynomial = Polynomial::from(polynomial);
        Self::Monomial {
            polynomial: Cow::Owned(polynomial),
            degree_bound,
        }
    }

    pub fn new_sparse_monomial_basis_ref(
        polynomial: &'a SparsePolynomial,
        degree_bound: Option<usize>,
    ) -> Self {
        let polynomial = Polynomial::Sparse(Cow::Borrowed(polynomial));
        Self::Monomial {
            polynomial: Cow::Owned(polynomial),
            degree_bound,
        }
    }

    pub fn new_sparse_monomial_basis(
        polynomial: SparsePolynomial,
        degree_bound: Option<usize>,
    ) -> Self {
        let polynomial = Polynomial::from(polynomial);
        Self::Monomial {
            polynomial: Cow::Owned(polynomial),
            degree_bound,
        }
    }

    pub fn new_lagrange_basis(evaluations: EvaluationsOnDomain) -> Self {
        Self::Lagrange {
            evaluations: Cow::Owned(evaluations),
        }
    }

    pub fn new_lagrange_basis_ref(evaluations: &'a EvaluationsOnDomain) -> Self {
        Self::Lagrange {
            evaluations: Cow::Borrowed(evaluations),
        }
    }

    pub fn is_in_monomial_basis(&self) -> bool {
        matches!(self, Self::Monomial { .. })
    }

    /// Retrieve the degree bound in `self`.
    pub fn degree_bound(&self) -> Option<usize> {
        match self {
            Self::Monomial { degree_bound, .. } => *degree_bound,
            _ => None,
        }
    }

    /// Retrieve the degree bound in `self`.
    pub fn is_sparse(&self) -> bool {
        match self {
            Self::Monomial { polynomial, .. } => {
                matches!(polynomial.as_ref(), Polynomial::Sparse(_))
            }
            _ => false,
        }
    }

    pub fn is_in_lagrange_basis(&self) -> bool {
        matches!(self, Self::Lagrange { .. })
    }

    pub fn domain(&self) -> Option<EvaluationDomain> {
        match self {
            Self::Lagrange { evaluations } => Some(evaluations.domain()),
            _ => None,
        }
    }

    pub fn evaluate(&self, point: Scalar) -> Scalar {
        match self {
            Self::Monomial { polynomial, .. } => polynomial.evaluate(point),
            Self::Lagrange { evaluations } => {
                let domain = evaluations.domain();
                let degree = domain.size() as u64;
                let multiplier = (point.pow([degree]) - Scalar::one()) / Scalar::from(degree);
                let powers: Vec<_> = domain.elements().collect();
                let mut denominators = cfg_iter!(powers).map(|pow| point - pow).collect::<Vec<_>>();
                Scalar::batch_inversion(&mut denominators);
                cfg_iter_mut!(denominators)
                    .zip_eq(powers)
                    .zip_eq(&evaluations.evaluations)
                    .map(|((denom, power), coeff)| *denom * power * coeff)
                    .sum::<Scalar>()
                    * multiplier
            }
        }
    }
}
