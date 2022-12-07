use crate::{
    bls12_377::Scalar,
    fft::EvaluationDomain,
    marlin::{
        ahp::{
            indexer::{Circuit, CircuitInfo, ConstraintSystem as IndexerConstraintSystem},
            matrices::arithmetize_matrix,
            AHPError, AHPForR1CS,
        },
        matrices::{matrix_evals, precomputation_for_matrix_evals, MatrixEvals},
        num_non_zero,
    },
    polycommit::sonic_pc::{PolynomialInfo, PolynomialLabel},
    r1cs::{ConstraintSynthesizer, ConstraintSystem},
};
use anyhow::anyhow;

use std::collections::BTreeMap;

use rayon::prelude::*;

use super::Matrix;

impl AHPForR1CS {
    /// Generate the index for this constraint system.
    pub fn index<C: ConstraintSynthesizer<Scalar>>(c: &C, mode: bool) -> Result<Circuit, AHPError> {
        let IndexerState {
            constraint_domain,

            a,
            non_zero_a_domain,
            a_evals,

            b,
            non_zero_b_domain,
            b_evals,

            c,
            non_zero_c_domain,
            c_evals,

            index_info,
        } = Self::index_helper(c)?;

        let [a_arith, b_arith, c_arith]: [_; 3] = [("a", a_evals), ("b", b_evals), ("c", c_evals)]
            .into_iter()
            .map(|(label, evals)| arithmetize_matrix(label, evals))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let (fft_precomputation, ifft_precomputation) = Self::fft_precomputation(
            constraint_domain.size(),
            non_zero_a_domain.size(),
            non_zero_b_domain.size(),
            non_zero_c_domain.size(),
        )
        .ok_or_else(|| anyhow!("polynomial degree too large"))?;

        Ok(Circuit {
            index_info,
            a,
            b,
            c,
            a_arith,
            b_arith,
            c_arith,
            fft_precomputation,
            ifft_precomputation,
            zk: mode,
        })
    }

    pub fn index_polynomial_info() -> BTreeMap<PolynomialLabel, PolynomialInfo> {
        let mut map = BTreeMap::new();
        for matrix in ["a", "b", "c"] {
            map.insert(
                format!("row_{matrix}"),
                PolynomialInfo::new(format!("row_{matrix}"), None, None),
            );
            map.insert(
                format!("col_{matrix}"),
                PolynomialInfo::new(format!("col_{matrix}"), None, None),
            );
            map.insert(
                format!("val_{matrix}"),
                PolynomialInfo::new(format!("val_{matrix}"), None, None),
            );
            map.insert(
                format!("row_col_{matrix}"),
                PolynomialInfo::new(format!("row_col_{matrix}"), None, None),
            );
        }
        map
    }

    pub fn index_polynomial_labels() -> impl Iterator<Item = PolynomialLabel> {
        ["a", "b", "c"].into_iter().flat_map(|matrix| {
            [
                format!("row_{matrix}"),
                format!("col_{matrix}"),
                format!("val_{matrix}"),
                format!("row_col_{matrix}"),
            ]
        })
    }

    fn index_helper<C: ConstraintSynthesizer<Scalar>>(c: &C) -> Result<IndexerState, AHPError> {
        let mut ics = IndexerConstraintSystem::new();
        c.generate_constraints(&mut ics)?;

        crate::marlin::ahp::matrices::pad_input_for_indexer_and_prover(&mut ics);
        ics.make_matrices_square();

        let a = ics.a_matrix();
        let b = ics.b_matrix();
        let c = ics.c_matrix();

        // balance_matrices(&mut a, &mut b);

        let num_padded_public_variables = ics.num_public_variables();
        let num_private_variables = ics.num_private_variables();
        let num_constraints = ics.num_constraints();
        let num_non_zero_a = num_non_zero(&a);
        let num_non_zero_b = num_non_zero(&b);
        let num_non_zero_c = num_non_zero(&c);
        let num_variables = num_padded_public_variables + num_private_variables;

        if cfg!(debug_assertions) {
            println!(
                "Number of padded public variables: {}",
                num_padded_public_variables
            );
            println!("Number of private variables: {}", num_private_variables);
            println!("Number of num_constraints: {}", num_constraints);
            println!("Number of non-zero entries in A: {}", num_non_zero_a);
            println!("Number of non-zero entries in B: {}", num_non_zero_b);
            println!("Number of non-zero entries in C: {}", num_non_zero_c);
        }

        if num_constraints != num_variables {
            eprintln!(
                "Number of padded public variables: {}",
                num_padded_public_variables
            );
            eprintln!("Number of private variables: {}", num_private_variables);
            eprintln!("Number of num_constraints: {}", num_constraints);
            eprintln!("Number of non-zero entries in A: {}", num_non_zero_a);
            eprintln!("Number of non-zero entries in B: {}", num_non_zero_b);
            eprintln!("Number of non-zero entries in C: {}", num_non_zero_c);
            return Err(AHPError::NonSquareMatrix);
        }

        Self::num_formatted_public_inputs_is_admissible(num_padded_public_variables)?;

        let index_info = CircuitInfo {
            num_public_inputs: num_padded_public_variables,
            num_variables,
            num_constraints,
            num_non_zero_a,
            num_non_zero_b,
            num_non_zero_c,
        };

        let constraint_domain = EvaluationDomain::new(num_constraints)
            .ok_or_else(|| anyhow!("polynomial degree too large"))?;
        let input_domain = EvaluationDomain::new(num_padded_public_variables)
            .ok_or_else(|| anyhow!("polynomial degree too large"))?;

        let non_zero_a_domain = EvaluationDomain::new(num_non_zero_a)
            .ok_or_else(|| anyhow!("polynomial degree too large"))?;
        let non_zero_b_domain = EvaluationDomain::new(num_non_zero_b)
            .ok_or_else(|| anyhow!("polynomial degree too large"))?;
        let non_zero_c_domain = EvaluationDomain::new(num_non_zero_c)
            .ok_or_else(|| anyhow!("polynomial degree too large"))?;

        let (constraint_domain_elements, constraint_domain_eq_poly_vals) =
            precomputation_for_matrix_evals(&constraint_domain);

        let [a_evals, b_evals, c_evals]: [_; 3] = cfg_into_iter!([
            (&a, &non_zero_a_domain),
            (&b, &non_zero_b_domain),
            (&c, &non_zero_c_domain),
        ])
        .map(|(matrix, non_zero_domain)| {
            matrix_evals(
                matrix,
                non_zero_domain,
                &constraint_domain,
                &input_domain,
                &constraint_domain_elements,
                &constraint_domain_eq_poly_vals,
            )
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

        Ok(IndexerState {
            constraint_domain,

            a,
            non_zero_a_domain,
            a_evals,

            b,
            non_zero_b_domain,
            b_evals,

            c,
            non_zero_c_domain,
            c_evals,

            index_info,
        })
    }

    pub fn evaluate_index_polynomials<C: ConstraintSynthesizer<Scalar>>(
        c: &C,
        point: Scalar,
    ) -> Result<impl Iterator<Item = Scalar>, AHPError> {
        let state = Self::index_helper(c)?;
        let mut evals = [
            ("a", state.a_evals, state.non_zero_a_domain),
            ("b", state.b_evals, state.non_zero_b_domain),
            ("c", state.c_evals, state.non_zero_c_domain),
        ]
        .into_iter()
        .flat_map(move |(matrix, evals, domain)| {
            let labels = [
                format!("row_{matrix}"),
                format!("col_{matrix}"),
                format!("val_{matrix}"),
                format!("row_col_{matrix}"),
            ];
            let lagrange_coefficients_at_point = domain.evaluate_all_lagrange_coefficients(point);
            labels
                .into_iter()
                .zip(evals.evaluate(&lagrange_coefficients_at_point))
        })
        .collect::<Vec<_>>();
        evals.sort_by(|(l1, _), (l2, _)| l1.cmp(l2));
        Ok(evals.into_iter().map(|(_, eval)| eval))
    }
}

struct IndexerState {
    constraint_domain: EvaluationDomain,

    a: Matrix,
    non_zero_a_domain: EvaluationDomain,
    a_evals: MatrixEvals,

    b: Matrix,
    non_zero_b_domain: EvaluationDomain,
    b_evals: MatrixEvals,

    c: Matrix,
    non_zero_c_domain: EvaluationDomain,
    c_evals: MatrixEvals,

    index_info: CircuitInfo,
}
