use core::convert::TryInto;
use std::collections::BTreeMap;

use crate::{
    bls12_377::Scalar,
    fft,
    fft::{
        domain::IFFTPrecomputation, polynomial::PolyMultiplier, DensePolynomial, EvaluationDomain,
        SparsePolynomial,
    },
    marlin::{
        ahp::{
            indexer::{CircuitInfo, Matrix},
            verifier, AHPForR1CS, UnnormalizedBivariateLagrangePoly,
        },
        prover, MarlinMode,
    },
    polycommit::sonic_pc::{LabeledPolynomial, PolynomialInfo, PolynomialLabel},
    utils::*,
};
use itertools::Itertools;
use rand_core::RngCore;
use rayon::prelude::*;

impl AHPForR1CS {
    /// Output the number of oracles sent by the prover in the second round.
    pub fn num_second_round_oracles() -> usize {
        2
    }

    /// Output the degree bounds of oracles in the first round.
    pub fn second_round_polynomial_info(
        info: &CircuitInfo,
    ) -> BTreeMap<PolynomialLabel, PolynomialInfo> {
        let constraint_domain_size =
            EvaluationDomain::compute_size_of_domain(info.num_constraints).unwrap();
        [
            PolynomialInfo::new(
                "g_1".into(),
                Some(constraint_domain_size - 2),
                Self::zk_bound(),
            ),
            PolynomialInfo::new("h_1".into(), None, None),
        ]
        .into_iter()
        .map(|info| (info.label().into(), info))
        .collect()
    }

    /// Output the second round message and the next state.
    pub fn prover_second_round<'a, R: RngCore>(
        verifier_message: &verifier::FirstMessage,
        mut state: prover::State<'a>,
        _r: &mut R,
    ) -> (prover::SecondOracles, prover::State<'a>) {
        let constraint_domain = state.constraint_domain;
        let zk_bound = Self::zk_bound();

        let verifier::FirstMessage {
            alpha,
            eta_b,
            eta_c,
            batch_combiners,
        } = verifier_message;

        let (summed_z_m, t) =
            Self::calculate_summed_z_m_and_t(&state, *alpha, *eta_b, *eta_c, batch_combiners);

        let z = cfg_iter!(state.first_round_oracles.as_ref().unwrap().batches)
            .zip_eq(batch_combiners)
            .zip(&state.x_poly)
            .map(|((b, &coeff), x_poly)| {
                let mut z = b
                    .w_poly
                    .polynomial()
                    .as_dense()
                    .unwrap()
                    .mul_by_vanishing_poly(state.input_domain);
                // Zip safety: `x_poly` is smaller than `z_poly`.
                z.coeffs
                    .iter_mut()
                    .zip(&x_poly.coeffs)
                    .for_each(|(z, x)| *z += x);
                cfg_iter_mut!(z.coeffs).for_each(|z| *z *= &coeff);
                z
            })
            .sum::<DensePolynomial>();
        assert!(z.degree() <= constraint_domain.size());

        let sumcheck_lhs = Self::calculate_lhs(&state, t, summed_z_m, z, *alpha);

        debug_assert!(sumcheck_lhs
            .evaluate_over_domain_by_ref(constraint_domain)
            .evaluations
            .into_iter()
            .sum::<Scalar>()
            .is_zero());

        let (h_1, x_g_1) = sumcheck_lhs
            .divide_by_vanishing_poly(constraint_domain)
            .unwrap();
        let g_1 = DensePolynomial::from_coefficients_slice(&x_g_1.coeffs[1..]);
        drop(x_g_1);

        assert!(g_1.degree() <= constraint_domain.size() - 2);
        assert!(h_1.degree() <= 2 * constraint_domain.size() + 2 * zk_bound.unwrap_or(0) - 2);

        let oracles = prover::SecondOracles {
            g_1: LabeledPolynomial::new(
                "g_1".into(),
                g_1,
                Some(constraint_domain.size() - 2),
                zk_bound,
            ),
            h_1: LabeledPolynomial::new("h_1".into(), h_1, None, None),
        };
        assert!(oracles.matches_info(&Self::second_round_polynomial_info(&state.index.index_info)));

        state.verifier_first_message = Some(verifier_message.clone());

        (oracles, state)
    }

    fn calculate_lhs(
        state: &prover::State,
        t: DensePolynomial,
        summed_z_m: DensePolynomial,
        z: DensePolynomial,
        alpha: Scalar,
    ) -> DensePolynomial {
        let constraint_domain = state.constraint_domain;

        let mask_poly = state
            .first_round_oracles
            .as_ref()
            .unwrap()
            .mask_poly
            .as_ref();
        assert_eq!(MM::ZK, mask_poly.is_some());

        let mul_domain_size =
            (constraint_domain.size() + summed_z_m.coeffs.len()).max(t.coeffs.len() + z.len());
        let mul_domain = EvaluationDomain::new(mul_domain_size)
            .expect("field is not smooth enough to construct domain");
        let mut multiplier = PolyMultiplier::new();
        multiplier.add_precomputation(state.fft_precomputation(), state.ifft_precomputation());
        multiplier.add_polynomial(summed_z_m, "summed_z_m");
        multiplier.add_polynomial(z, "z");
        multiplier.add_polynomial(t, "t");
        let r_alpha_x_evals = {
            let r_alpha_x_evals = constraint_domain
                .batch_eval_unnormalized_bivariate_lagrange_poly_with_diff_inputs_over_domain(
                    alpha,
                    &mul_domain,
                );
            fft::Evaluations::from_vec_and_domain(r_alpha_x_evals, mul_domain)
        };
        multiplier.add_evaluation(r_alpha_x_evals, "r_alpha_x");
        let mut lhs = multiplier
            .element_wise_arithmetic_4_over_domain(
                mul_domain,
                ["r_alpha_x", "summed_z_m", "z", "t"],
                |a, b, c, d| a * b - c * d,
            )
            .unwrap();

        lhs += &mask_poly.map_or(SparsePolynomial::zero(), |p| {
            p.polynomial().as_sparse().unwrap().clone()
        });
        lhs
    }

    fn calculate_summed_z_m_and_t(
        state: &prover::State,
        alpha: Scalar,
        eta_b: Scalar,
        eta_c: Scalar,
        batch_combiners: &[Scalar],
    ) -> (DensePolynomial, DensePolynomial) {
        let constraint_domain = state.constraint_domain;

        let fft_precomputation = &state.index.fft_precomputation;
        let ifft_precomputation = &state.index.ifft_precomputation;
        let first_msg = state.first_round_oracles.as_ref().unwrap();
        let mut job_pool = ExecutionPool::with_capacity(2 * state.batch_size);
        let eta_b_over_eta_c = eta_b * eta_c.inverse().unwrap();
        job_pool.add_job(|| {
            cfg_iter!(first_msg.batches)
                .zip_eq(batch_combiners)
                .map(|(entry, combiner)| {
                    let z_a = entry.z_a_poly.polynomial().as_dense().unwrap();
                    let mut z_b = entry.z_b_poly.polynomial().as_dense().unwrap().clone();
                    assert!(z_a.degree() < constraint_domain.size());
                    if MM::ZK {
                        assert_eq!(z_b.degree(), constraint_domain.size());
                    } else {
                        assert!(z_b.degree() < constraint_domain.size());
                    }

                    // we want to calculate r_i * (z_a + eta_b * z_b + eta_c * z_a * z_b);
                    // we rewrite this as  r_i * (z_a * (eta_c * z_b + 1) + eta_b * z_b);
                    // This is better since it reduces the number of required
                    // multiplications by `constraint_domain.size()`.
                    let mut summed_z_m = {
                        // Mutate z_b in place to compute eta_c * z_b + 1
                        // This saves us an additional memory allocation.
                        cfg_iter_mut!(z_b.coeffs).for_each(|b| *b *= eta_c);
                        z_b.coeffs[0] += Scalar::ONE;
                        let mut multiplier = PolyMultiplier::new();
                        multiplier.add_polynomial_ref(z_a, "z_a");
                        multiplier.add_polynomial_ref(&z_b, "eta_c_z_b_plus_one");
                        multiplier.add_precomputation(fft_precomputation, ifft_precomputation);
                        let result = multiplier.multiply().unwrap();
                        // Start undoing in place mutation, by first subtracting the 1 that we added...
                        z_b.coeffs[0] -= Scalar::ONE;
                        result
                    };
                    // ... and then multiplying by eta_b/eta_c, instead of just eta_b.
                    cfg_iter_mut!(summed_z_m.coeffs)
                        .zip(&z_b.coeffs)
                        .for_each(|(c, b)| *c += eta_b_over_eta_c * b);

                    // Multiply by linear combination coefficient.
                    cfg_iter_mut!(summed_z_m.coeffs).for_each(|c| *c *= *combiner);

                    assert_eq!(summed_z_m.degree(), z_a.degree() + z_b.degree());
                    summed_z_m
                })
                .sum::<DensePolynomial<_>>()
        });

        job_pool.add_job(|| {
            let r_alpha_x_evals = constraint_domain
                .batch_eval_unnormalized_bivariate_lagrange_poly_with_diff_inputs(alpha);
            let t = Self::calculate_t(
                &[&state.index.a, &state.index.b, &state.index.c],
                [Scalar::ONE, eta_b, eta_c],
                &state.input_domain,
                &state.constraint_domain,
                &r_alpha_x_evals,
                ifft_precomputation,
            );
            t
        });
        let [summed_z_m, t]: [DensePolynomial; 2] = job_pool.execute_all().try_into().unwrap();
        (summed_z_m, t)
    }

    fn calculate_t<'a>(
        matrices: &[&'a Matrix],
        matrix_randomizers: [Scalar; 3],
        input_domain: &EvaluationDomain,
        constraint_domain: &EvaluationDomain,
        r_alpha_x_on_h: &[Scalar],
        ifft_precomputation: &IFFTPrecomputation,
    ) -> DensePolynomial {
        let mut t_evals_on_h = vec![Scalar::ZERO; constraint_domain.size()];
        for (matrix, eta) in matrices.iter().zip_eq(matrix_randomizers) {
            for (r, row) in matrix.iter().enumerate() {
                for (coeff, c) in row.iter() {
                    let index = constraint_domain.reindex_by_subdomain(input_domain, *c);
                    t_evals_on_h[index] += &(eta * coeff * r_alpha_x_on_h[r]);
                }
            }
        }
        fft::Evaluations::from_vec_and_domain(t_evals_on_h, *constraint_domain)
            .interpolate_with_pc(ifft_precomputation)
    }
}
