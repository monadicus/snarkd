use crate::{
    bls12_377::{Field, Scalar},
    fft::domain::{FFTPrecomputation, IFFTPrecomputation},
    utils::*,
};
use rayon::prelude::*;
use std::{borrow::Borrow, collections::BTreeMap};

/// A struct that helps multiply a batch of polynomials
use super::*;

#[derive(Default)]
pub struct PolyMultiplier<'a> {
    polynomials: Vec<(String, Cow<'a, DensePolynomial>)>,
    evaluations: Vec<(String, Cow<'a, crate::fft::Evaluations>)>,
    fft_precomputation: Option<Cow<'a, FFTPrecomputation>>,
    ifft_precomputation: Option<Cow<'a, IFFTPrecomputation>>,
}

impl<'a> PolyMultiplier<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            polynomials: Vec::new(),
            evaluations: Vec::new(),
            fft_precomputation: None,
            ifft_precomputation: None,
        }
    }

    #[inline]
    pub fn add_precomputation(
        &mut self,
        fft_pc: &'a FFTPrecomputation,
        ifft_pc: &'a IFFTPrecomputation,
    ) {
        self.fft_precomputation = Some(Cow::Borrowed(fft_pc));
        self.ifft_precomputation = Some(Cow::Borrowed(ifft_pc));
    }

    #[inline]
    pub fn add_polynomial(&mut self, poly: DensePolynomial, label: impl ToString) {
        self.polynomials.push((label.to_string(), Cow::Owned(poly)))
    }

    #[inline]
    pub fn add_evaluation(&mut self, evals: Evaluations, label: impl ToString) {
        self.evaluations
            .push((label.to_string(), Cow::Owned(evals)))
    }

    #[inline]
    pub fn add_polynomial_ref(&mut self, poly: &'a DensePolynomial, label: impl ToString) {
        self.polynomials
            .push((label.to_string(), Cow::Borrowed(poly)))
    }

    #[inline]
    pub fn add_evaluation_ref(&mut self, evals: &'a Evaluations, label: impl ToString) {
        self.evaluations
            .push((label.to_string(), Cow::Borrowed(evals)))
    }

    /// Multiplies all polynomials stored in `self`.
    ///
    /// Returns `None` if any of the stored evaluations are over a domain that's
    /// insufficiently large to interpolate the product, or if `F` does not contain
    /// a sufficiently large subgroup for interpolation.
    pub fn multiply(mut self) -> Option<DensePolynomial> {
        if self.polynomials.is_empty() && self.evaluations.is_empty() {
            Some(DensePolynomial::zero())
        } else {
            let degree = self
                .polynomials
                .iter()
                .map(|(_, p)| p.degree() + 1)
                .sum::<usize>();
            let domain = EvaluationDomain::new(degree)?;
            if self.evaluations.iter().any(|(_, e)| e.domain() != domain) {
                None
            } else {
                if self.fft_precomputation.is_none() {
                    self.fft_precomputation = Some(Cow::Owned(domain.precompute_fft()));
                }
                if self.ifft_precomputation.is_none() {
                    self.ifft_precomputation = Some(Cow::Owned(
                        self.fft_precomputation
                            .as_ref()
                            .unwrap()
                            .to_ifft_precomputation(),
                    ));
                }
                let fft_pc = &self.fft_precomputation.unwrap();
                let ifft_pc = &self.ifft_precomputation.unwrap();
                let mut pool = ExecutionPool::new();
                for (_, p) in self.polynomials {
                    pool.add_job(move || {
                        let mut p = p.clone().into_owned().coeffs;
                        p.resize(domain.size(), Scalar::ZERO);
                        domain.out_order_fft_in_place_with_pc(&mut p, fft_pc);
                        p
                    })
                }
                for (_, e) in self.evaluations {
                    pool.add_job(move || {
                        let mut e = e.clone().into_owned().evaluations;
                        e.resize(domain.size(), Scalar::ZERO);
                        crate::fft::domain::derange(&mut e);
                        e
                    })
                }
                let results = pool.execute_all();
                let mut result = results
                    .into_par_iter()
                    .reduce_with(|mut a, b| {
                        cfg_iter_mut!(a).zip(b).for_each(|(a, b)| *a *= b);
                        a
                    })
                    .unwrap();
                domain.out_order_ifft_in_place_with_pc(&mut result, ifft_pc);
                Some(DensePolynomial::from_coefficients_vec(result.to_vec()))
            }
        }
    }

    pub fn element_wise_arithmetic_4_over_domain<T: Borrow<str>>(
        mut self,
        domain: EvaluationDomain,
        labels: [T; 4],
        f: impl Fn(Scalar, Scalar, Scalar, Scalar) -> Scalar + Sync,
    ) -> Option<DensePolynomial> {
        if self.fft_precomputation.is_none() {
            self.fft_precomputation = Some(Cow::Owned(domain.precompute_fft()));
        }
        if self.ifft_precomputation.is_none() {
            self.ifft_precomputation = Some(Cow::Owned(
                self.fft_precomputation
                    .as_ref()
                    .unwrap()
                    .to_ifft_precomputation(),
            ));
        }
        let fft_pc = self.fft_precomputation.as_ref().unwrap();
        let mut pool = ExecutionPool::new();
        for (l, p) in self.polynomials {
            pool.add_job(move || {
                let mut p = p.clone().into_owned().coeffs;
                p.resize(domain.size(), Scalar::ZERO);
                domain.out_order_fft_in_place_with_pc(&mut p, fft_pc);
                (l, p)
            })
        }
        for (l, e) in self.evaluations {
            pool.add_job(move || {
                let mut e = e.clone().into_owned().evaluations;
                e.resize(domain.size(), Scalar::ZERO);
                crate::fft::domain::derange(&mut e);
                (l, e)
            })
        }
        let p = pool.execute_all().into_iter().collect::<BTreeMap<_, _>>();
        assert_eq!(p.len(), 4);
        let mut result = cfg_iter!(p[labels[0].borrow()])
            .zip(&p[labels[1].borrow()])
            .zip(&p[labels[2].borrow()])
            .zip(&p[labels[3].borrow()])
            .map(|(((a, b), c), d)| f(*a, *b, *c, *d))
            .collect::<Vec<_>>();
        drop(p);
        domain.out_order_ifft_in_place_with_pc(&mut result, &self.ifft_precomputation.unwrap());
        Some(DensePolynomial::from_coefficients_vec(result.to_vec()))
    }
}
