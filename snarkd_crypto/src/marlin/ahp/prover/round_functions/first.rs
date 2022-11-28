use std::{collections::BTreeMap, sync::Arc};

use crate::{
    bls12_377::{Field, Scalar},
    fft::{
        DensePolynomial, EvaluationDomain, Evaluations as EvaluationsOnDomain, SparsePolynomial,
    },
    marlin::{
        ahp::{AHPError, AHPForR1CS},
        prover, witness_label,
    },
    polycommit::sonic_pc::{
        LabeledPolynomial, LabeledPolynomialWithBasis, PolynomialInfo, PolynomialLabel,
        PolynomialWithBasis,
    },
};
use itertools::Itertools;
use rayon::prelude::*;

impl AHPForR1CS {
    /// Output the number of oracles sent by the prover in the first round.
    pub fn num_first_round_oracles(&self, batch_size: usize) -> usize {
        3 * batch_size + (self.mode as usize)
    }

    /// Output the degree bounds of oracles in the first round.
    pub fn first_round_polynomial_info(
        &self,
        batch_size: usize,
    ) -> BTreeMap<PolynomialLabel, PolynomialInfo> {
        let mut polynomials = Vec::new();

        for i in 0..batch_size {
            polynomials.push(PolynomialInfo::new(
                witness_label("w", i),
                None,
                self.zk_bound(),
            ));
            polynomials.push(PolynomialInfo::new(
                witness_label("z_a", i),
                None,
                self.zk_bound(),
            ));
            polynomials.push(PolynomialInfo::new(
                witness_label("z_b", i),
                None,
                self.zk_bound(),
            ));
        }
        if self.mode {
            polynomials.push(PolynomialInfo::new("mask_poly".to_string(), None, None));
        }
        polynomials
            .into_iter()
            .map(|info| (info.label().into(), info))
            .collect()
    }

    /// Output the first round message and the next state.
    #[allow(clippy::type_complexity)]
    pub fn prover_first_round<'a>(
        &self,
        mut state: prover::State<'a>,
    ) -> Result<prover::State<'a>, AHPError> {
        let constraint_domain = state.constraint_domain;
        let batch_size = state.batch_size;

        let z_a = state.z_a.take().unwrap();
        let z_b = state.z_b.take().unwrap();
        let private_variables = core::mem::take(&mut state.private_variables);
        assert_eq!(z_a.len(), batch_size);
        assert_eq!(z_b.len(), batch_size);
        assert_eq!(private_variables.len(), batch_size);
        let mut r_b_s = Vec::with_capacity(batch_size);

        let mut job_pool = crate::utils::ExecutionPool::with_capacity(3 * batch_size);
        let state_ref = &state;
        for (i, (z_a, z_b, private_variables, x_poly)) in
            itertools::izip!(z_a, z_b, private_variables, &state.x_poly).enumerate()
        {
            job_pool.add_job(move || {
                self.calculate_w(witness_label("w", i), private_variables, x_poly, state_ref)
            });
            job_pool.add_job(move || {
                self.calculate_z_m(witness_label("z_a", i), z_a, false, state_ref, None)
            });
            let r_b = Scalar::rand();
            job_pool.add_job(move || {
                self.calculate_z_m(witness_label("z_b", i), z_b, true, state_ref, Some(r_b))
            });
            if self.mode {
                r_b_s.push(r_b);
            }
        }

        let batches = job_pool
            .execute_all()
            .into_iter()
            .tuples()
            .map(|(w, z_a, z_b)| {
                let w_poly = w.witness().unwrap();
                let (z_a_poly, z_a) = z_a.z_m().unwrap();
                let (z_b_poly, z_b) = z_b.z_m().unwrap();

                prover::SingleEntry {
                    z_a,
                    z_b,
                    w_poly,
                    z_a_poly,
                    z_b_poly,
                }
            })
            .collect::<Vec<_>>();
        assert_eq!(batches.len(), batch_size);

        let mask_poly = self.calculate_mask_poly(constraint_domain);

        let oracles = prover::FirstOracles { batches, mask_poly };
        assert!(oracles.matches_info(&self.first_round_polynomial_info(batch_size)));
        state.first_round_oracles = Some(Arc::new(oracles));
        state.mz_poly_randomizer = self.mode.then_some(r_b_s);

        Ok(state)
    }

    fn calculate_mask_poly(
        &self,
        constraint_domain: EvaluationDomain,
    ) -> Option<LabeledPolynomial> {
        self.mode
            .then(|| {
                // We'll use the masking technique from Lunar (https://eprint.iacr.org/2020/1069.pdf, pgs 20-22).
                let h_1_mask = DensePolynomial::rand(3).coeffs; // selected arbitrarily.
                let h_1_mask =
                    SparsePolynomial::from_coefficients(h_1_mask.into_iter().enumerate())
                        .mul(&constraint_domain.vanishing_polynomial());
                assert_eq!(h_1_mask.degree(), constraint_domain.size() + 3);
                // multiply g_1_mask by X
                let mut g_1_mask = DensePolynomial::rand(5);
                g_1_mask.coeffs[0] = Scalar::ZERO;
                let g_1_mask = SparsePolynomial::from_coefficients(
                    g_1_mask
                        .coeffs
                        .into_iter()
                        .enumerate()
                        .filter(|(_, coeff)| !coeff.is_zero()),
                );

                let mut mask_poly = h_1_mask;
                mask_poly += &g_1_mask;
                debug_assert!(constraint_domain
                    .elements()
                    .map(|z| mask_poly.evaluate(z))
                    .sum::<Scalar>()
                    .is_zero());
                assert_eq!(mask_poly.degree(), constraint_domain.size() + 3);
                assert!(
                    mask_poly.degree()
                        <= 3 * constraint_domain.size() + 2 * self.zk_bound().unwrap() - 3
                );

                mask_poly
            })
            .map(|mask_poly| LabeledPolynomial::new("mask_poly".to_string(), mask_poly, None, None))
    }

    fn calculate_w<'a>(
        &self,
        label: String,
        private_variables: Vec<Scalar>,
        x_poly: &DensePolynomial,
        state: &prover::State<'a>,
    ) -> PoolResult<'a> {
        let constraint_domain = state.constraint_domain;
        let input_domain = state.input_domain;

        let mut w_extended = private_variables;
        let ratio = constraint_domain.size() / input_domain.size();
        w_extended.resize(constraint_domain.size() - input_domain.size(), Scalar::ZERO);

        let x_evals = {
            let mut coeffs = x_poly.coeffs.clone();
            coeffs.resize(constraint_domain.size(), Scalar::ZERO);
            constraint_domain
                .in_order_fft_in_place_with_pc(&mut coeffs, state.fft_precomputation());
            coeffs
        };

        let w_poly_evals = cfg_into_iter!(0..constraint_domain.size())
            .map(|k| match k % ratio {
                0 => Scalar::ZERO,
                _ => w_extended[k - (k / ratio) - 1] - x_evals[k],
            })
            .collect();
        let w_poly = EvaluationsOnDomain::from_vec_and_domain(w_poly_evals, constraint_domain)
            .interpolate_with_pc(state.ifft_precomputation());
        let (w_poly, remainder) = w_poly.divide_by_vanishing_poly(input_domain).unwrap();
        assert!(remainder.is_zero());

        assert!(w_poly.degree() < constraint_domain.size() - input_domain.size());
        PoolResult::Witness(LabeledPolynomial::new(label, w_poly, None, self.zk_bound()))
    }

    fn calculate_z_m<'a>(
        &self,
        label: impl ToString,
        evaluations: Vec<Scalar>,
        will_be_evaluated: bool,
        state: &prover::State<'a>,
        r: Option<Scalar>,
    ) -> PoolResult<'a> {
        let constraint_domain = state.constraint_domain;
        let v_H = constraint_domain.vanishing_polynomial();
        let should_randomize = self.mode && will_be_evaluated;
        let label = label.to_string();

        let evals = EvaluationsOnDomain::from_vec_and_domain(evaluations, constraint_domain);

        let mut poly = evals.interpolate_with_pc_by_ref(state.ifft_precomputation());
        if should_randomize {
            poly += &(&v_H * r.unwrap());
        }

        debug_assert!(
            poly.evaluate_over_domain_by_ref(constraint_domain)
                .evaluations
                .into_iter()
                .zip(&evals.evaluations)
                .all(|(z, e)| *e == z),
            "Label: {label}\n1: {:#?}\n2: {:#?}",
            poly.evaluate_over_domain_by_ref(constraint_domain)
                .evaluations,
            &evals.evaluations,
        );

        let poly_for_opening =
            LabeledPolynomial::new(label.to_string(), poly, None, self.zk_bound());
        if should_randomize {
            assert!(
                poly_for_opening.degree() < constraint_domain.size() + self.zk_bound().unwrap()
            );
        } else {
            assert!(poly_for_opening.degree() < constraint_domain.size());
        }

        let poly_for_committing = if should_randomize {
            let poly_terms = vec![
                (Scalar::ONE, PolynomialWithBasis::new_lagrange_basis(evals)),
                (
                    Scalar::ONE,
                    PolynomialWithBasis::new_sparse_monomial_basis(&v_H * r.unwrap(), None),
                ),
            ];
            LabeledPolynomialWithBasis::new_linear_combination(label, poly_terms, self.zk_bound())
        } else {
            LabeledPolynomialWithBasis::new_lagrange_basis(label, evals, self.zk_bound())
        };

        PoolResult::MatrixPoly(poly_for_opening, poly_for_committing)
    }
}

#[derive(Debug)]
pub enum PoolResult<'a> {
    Witness(LabeledPolynomial),
    MatrixPoly(LabeledPolynomial, LabeledPolynomialWithBasis<'a>),
}

impl<'a> PoolResult<'a> {
    fn witness(self) -> Option<LabeledPolynomial> {
        match self {
            Self::Witness(poly) => Some(poly),
            _ => None,
        }
    }

    fn z_m(self) -> Option<(LabeledPolynomial, LabeledPolynomialWithBasis<'a>)> {
        match self {
            Self::MatrixPoly(p1, p2) => Some((p1, p2)),
            _ => None,
        }
    }
}
