use core::convert::TryInto;
use std::collections::BTreeMap;

use crate::{
    bls12_377::Scalar,
    fft::{
        domain::{FFTPrecomputation, IFFTPrecomputation},
        polynomial::PolyMultiplier,
        DensePolynomial, EvaluationDomain, Evaluations as EvaluationsOnDomain,
    },
    marlin::{
        ahp::{indexer::CircuitInfo, verifier, AHPError, AHPForR1CS},
        matrices::MatrixArithmetization,
        prover,
    },
    polycommit::sonic_pc::{LabeledPolynomial, PolynomialInfo, PolynomialLabel},
    utils::*,
};
use rayon::prelude::*;

use rand_core::RngCore;

impl AHPForR1CS {
    /// Output the number of oracles sent by the prover in the third round.
    pub fn num_third_round_oracles() -> usize {
        3
    }

    /// Output the degree bounds of oracles in the first round.
    pub fn third_round_polynomial_info(
        info: &CircuitInfo,
    ) -> BTreeMap<PolynomialLabel, PolynomialInfo> {
        let non_zero_a_size =
            EvaluationDomain::compute_size_of_domain(info.num_non_zero_a).unwrap();
        let non_zero_b_size =
            EvaluationDomain::compute_size_of_domain(info.num_non_zero_b).unwrap();
        let non_zero_c_size =
            EvaluationDomain::compute_size_of_domain(info.num_non_zero_c).unwrap();

        [
            PolynomialInfo::new("g_a".into(), Some(non_zero_a_size - 2), None),
            PolynomialInfo::new("g_b".into(), Some(non_zero_b_size - 2), None),
            PolynomialInfo::new("g_c".into(), Some(non_zero_c_size - 2), None),
        ]
        .into_iter()
        .map(|info| (info.label().into(), info))
        .collect()
    }

    /// Output the third round message and the next state.
    pub fn prover_third_round<'a, R: RngCore>(
        verifier_message: &verifier::SecondMessage,
        mut state: prover::State<'a>,
        _r: &mut R,
    ) -> Result<
        (
            prover::ThirdMessage,
            prover::ThirdOracles,
            prover::State<'a>,
        ),
        AHPError,
    > {
        let verifier::FirstMessage { alpha, .. } = state.verifier_first_message.as_ref().expect(
            "prover::State should include verifier_first_msg when prover_third_round is called",
        );

        let beta = verifier_message.beta;

        let v_H_at_alpha = state
            .constraint_domain
            .evaluate_vanishing_polynomial(*alpha);
        let v_H_at_beta = state.constraint_domain.evaluate_vanishing_polynomial(beta);

        let v_H_alpha_v_H_beta = v_H_at_alpha * v_H_at_beta;

        let largest_non_zero_domain_size =
            Self::max_non_zero_domain(&state.index.index_info).size_as_field_element;
        let mut pool = ExecutionPool::with_capacity(3);
        pool.add_job(|| {
            Self::matrix_sumcheck_helper(
                "a",
                state.non_zero_a_domain,
                &state.index.a_arith,
                *alpha,
                beta,
                v_H_alpha_v_H_beta,
                largest_non_zero_domain_size,
                state.fft_precomputation(),
                state.ifft_precomputation(),
            )
        });

        pool.add_job(|| {
            Self::matrix_sumcheck_helper(
                "b",
                state.non_zero_b_domain,
                &state.index.b_arith,
                *alpha,
                beta,
                v_H_alpha_v_H_beta,
                largest_non_zero_domain_size,
                state.fft_precomputation(),
                state.ifft_precomputation(),
            )
        });

        pool.add_job(|| {
            Self::matrix_sumcheck_helper(
                "c",
                state.non_zero_c_domain,
                &state.index.c_arith,
                *alpha,
                beta,
                v_H_alpha_v_H_beta,
                largest_non_zero_domain_size,
                state.fft_precomputation(),
                state.ifft_precomputation(),
            )
        });

        let [(sum_a, lhs_a, g_a), (sum_b, lhs_b, g_b), (sum_c, lhs_c, g_c)]: [_; 3] =
            pool.execute_all().try_into().unwrap();

        let msg = prover::ThirdMessage {
            sum_a,
            sum_b,
            sum_c,
        };
        let oracles = prover::ThirdOracles { g_a, g_b, g_c };
        state.lhs_polynomials = Some([lhs_a, lhs_b, lhs_c]);
        state.sums = Some([sum_a, sum_b, sum_c]);
        assert!(oracles.matches_info(&Self::third_round_polynomial_info(&state.index.index_info)));

        Ok((msg, oracles, state))
    }

    #[allow(clippy::too_many_arguments)]
    fn matrix_sumcheck_helper(
        label: &str,
        non_zero_domain: EvaluationDomain,
        arithmetization: &MatrixArithmetization,
        alpha: Scalar,
        beta: Scalar,
        v_H_alpha_v_H_beta: Scalar,
        largest_non_zero_domain_size: Scalar,
        fft_precomputation: &FFTPrecomputation,
        ifft_precomputation: &IFFTPrecomputation,
    ) -> (Scalar, DensePolynomial, LabeledPolynomial) {
        let mut job_pool = crate::utils::ExecutionPool::with_capacity(2);
        job_pool.add_job(|| {
            let a_poly = {
                let coeffs = cfg_iter!(arithmetization.val.as_dense().unwrap().coeffs())
                    .map(|a| v_H_alpha_v_H_beta * a)
                    .collect();
                DensePolynomial::from_coefficients_vec(coeffs)
            };
            a_poly
        });

        let (row_on_K, col_on_K, row_col_on_K) = (
            &arithmetization.evals_on_K.row,
            &arithmetization.evals_on_K.col,
            &arithmetization.evals_on_K.row_col,
        );

        job_pool.add_job(|| {
            let alpha_beta = alpha * beta;
            let b_poly = {
                let evals: Vec<Scalar> = cfg_iter!(row_on_K.evaluations)
                    .zip_eq(&col_on_K.evaluations)
                    .zip_eq(&row_col_on_K.evaluations)
                    .map(|((r, c), r_c)| alpha_beta - alpha * r - beta * c + r_c)
                    .collect();
                EvaluationsOnDomain::from_vec_and_domain(evals, non_zero_domain)
                    .interpolate_with_pc(ifft_precomputation)
            };
            b_poly
        });
        let [a_poly, b_poly]: [_; 2] = job_pool.execute_all().try_into().unwrap();

        let mut inverses: Vec<_> = cfg_iter!(row_on_K.evaluations)
            .zip_eq(&col_on_K.evaluations)
            .map(|(r, c)| (beta - r) * (alpha - c))
            .collect();
        Scalar::batch_inversion_and_mul(&mut inverses, &v_H_alpha_v_H_beta);

        cfg_iter_mut!(inverses)
            .zip_eq(&arithmetization.evals_on_K.val.evaluations)
            .for_each(|(inv, a)| *inv *= a);
        let f_evals_on_K = inverses;

        let f = EvaluationsOnDomain::from_vec_and_domain(f_evals_on_K, non_zero_domain)
            .interpolate_with_pc(ifft_precomputation);
        let g = DensePolynomial::from_coefficients_slice(&f.coeffs[1..]);
        let h = &a_poly
            - &{
                let mut multiplier = PolyMultiplier::new();
                multiplier.add_polynomial_ref(&b_poly, "b");
                multiplier.add_polynomial_ref(&f, "f");
                multiplier.add_precomputation(fft_precomputation, ifft_precomputation);
                multiplier.multiply().unwrap()
            };
        // Let K_max = largest_non_zero_domain;
        // Let K = non_zero_domain;
        // Let s := K_max.selector_polynomial(K) = (v_K_max / v_K) * (K.size() / K_max.size());
        // Let v_K_max := K_max.vanishing_polynomial();
        // Let v_K := K.vanishing_polynomial();

        // Later on, we multiply `h` by s, and divide by v_K_max.
        // Substituting in s, we get that h * s / v_K_max = h / v_K * (K.size() / K_max.size());
        // That's what we're computing here.
        let (mut h, remainder) = h.divide_by_vanishing_poly(non_zero_domain).unwrap();
        assert!(remainder.is_zero());
        let multiplier = non_zero_domain.size_as_field_element / largest_non_zero_domain_size;
        cfg_iter_mut!(h.coeffs).for_each(|c| *c *= multiplier);

        let g = LabeledPolynomial::new(
            "g_".to_string() + label,
            g,
            Some(non_zero_domain.size() - 2),
            None,
        );

        assert!(h.degree() <= non_zero_domain.size() - 2);
        assert!(g.degree() <= non_zero_domain.size() - 2);
        (f.coeffs[0], h, g)
    }
}
