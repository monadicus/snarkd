#![allow(non_snake_case)]

use crate::{
    bls12_377::Scalar,
    fft::{EvaluationDomain, Evaluations as EvaluationsOnDomain},
    marlin::ahp::{indexer::Matrix, UnnormalizedBivariateLagrangePoly},
    polycommit::sonic_pc::LabeledPolynomial,
    r1cs::{ConstraintSystem, Index as VarIndex},
    utils::*,
};
use hashbrown::HashMap;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::BTreeMap;

// This function converts a matrix output by Zexe's constraint infrastructure
// to the one used in this crate.
pub(crate) fn to_matrix_helper(
    matrix: &[Vec<(Scalar, VarIndex)>],
    num_input_variables: usize,
) -> Matrix {
    cfg_iter!(matrix)
        .map(|row| {
            let mut row_map = BTreeMap::new();
            row.iter().for_each(|(fe, column)| {
                let column = match column {
                    VarIndex::Public(i) => *i,
                    VarIndex::Private(i) => num_input_variables + i,
                };
                *row_map.entry(column).or_insert_with(Scalar::ZERO) += *fe;
            });
            row_map
                .into_iter()
                .map(|(column, coeff)| (coeff, column))
                .collect()
        })
        .collect()
}

/// This must *always* be in sync with `make_matrices_square`.
pub(crate) fn padded_matrix_dim(num_formatted_variables: usize, num_constraints: usize) -> usize {
    core::cmp::max(num_formatted_variables, num_constraints)
}

/// Pads the public variables up to the closest power of two.
pub(crate) fn pad_input_for_indexer_and_prover<CS: ConstraintSystem>(cs: &mut CS) {
    let num_public_variables = cs.num_public_variables();

    let power_of_two = EvaluationDomain::new(num_public_variables);
    assert!(power_of_two.is_some());

    // Allocated `zero` variables to pad the public input up to the next power of two.
    let padded_size = power_of_two.unwrap().size();
    if padded_size > num_public_variables {
        for i in 0..(padded_size - num_public_variables) {
            cs.alloc_input(|| format!("pad_input_{}", i), || Ok(Scalar::ZERO))
                .unwrap();
        }
    }
}

pub(crate) fn make_matrices_square<CS: ConstraintSystem>(
    cs: &mut CS,
    num_formatted_variables: usize,
) {
    let num_constraints = cs.num_constraints();
    let matrix_padding = ((num_formatted_variables as isize) - (num_constraints as isize)).abs();

    if num_formatted_variables > num_constraints {
        use core::convert::identity as iden;
        // Add dummy constraints of the form 0 * 0 == 0
        for i in 0..matrix_padding {
            cs.enforce(|| format!("pad_constraint_{}", i), iden, iden, iden);
        }
    } else {
        // Add dummy unconstrained variables
        for i in 0..matrix_padding {
            let _ = cs
                .alloc(|| format!("pad_variable_{}", i), || Ok(Scalar::ONE))
                .expect("alloc failed");
        }
    }
}

#[derive(Clone, Debug)]
pub struct MatrixEvals {
    /// Evaluations of the `row` polynomial.
    pub row: EvaluationsOnDomain,
    /// Evaluations of the `col` polynomial.
    pub col: EvaluationsOnDomain,
    /// Evaluations of the `val` polynomial.
    pub val: EvaluationsOnDomain,
    /// Evaluations of the `row_col` polynomial.
    pub row_col: EvaluationsOnDomain,
}

impl MatrixEvals {
    pub(crate) fn evaluate(&self, lagrange_coefficients_at_point: &[Scalar]) -> [Scalar; 4] {
        [
            self.row
                .evaluate_with_coeffs(lagrange_coefficients_at_point),
            self.col
                .evaluate_with_coeffs(lagrange_coefficients_at_point),
            self.val
                .evaluate_with_coeffs(lagrange_coefficients_at_point),
            self.row_col
                .evaluate_with_coeffs(lagrange_coefficients_at_point),
        ]
    }
}

/// Contains information about the arithmetization of the matrix M^*.
/// Here `M^*(i, j) := M(j, i) * u_H(j, j)`. For more details, see [\[COS20\]](https://eprint.iacr.org/2019/1076).
#[derive(Clone, Debug)]
pub struct MatrixArithmetization {
    /// LDE of the row indices of M^*.
    pub row: LabeledPolynomial,
    /// LDE of the column indices of M^*.
    pub col: LabeledPolynomial,
    /// LDE of the vector containing entry-wise products of `row` and `col`.
    pub row_col: LabeledPolynomial,
    /// LDE of the non-zero entries of M^*.
    pub val: LabeledPolynomial,

    /// Evaluation of `self.row_a`, `self.col_a`, and `self.val_a` on the domain `K`.
    pub evals_on_K: MatrixEvals,
}

pub(crate) fn precomputation_for_matrix_evals(
    constraint_domain: &EvaluationDomain,
) -> (Vec<Scalar>, HashMap<Scalar, Scalar>) {
    let elements = constraint_domain.elements().collect::<Vec<_>>();
    let eq_poly_vals: HashMap<Scalar, Scalar> = elements
        .iter()
        .cloned()
        .zip_eq(
            constraint_domain.batch_eval_unnormalized_bivariate_lagrange_poly_with_same_inputs(),
        )
        .collect();
    (elements, eq_poly_vals)
}

pub(crate) fn matrix_evals(
    matrix: &Matrix,
    non_zero_domain: &EvaluationDomain,
    constraint_domain: &EvaluationDomain,
    input_domain: &EvaluationDomain,
    elems: &[Scalar],
    eq_poly_vals: &HashMap<Scalar, Scalar>,
) -> MatrixEvals {
    // Recall that we are computing the arithmetization of M^*,
    // where `M^*(i, j) := M(j, i) * u_H(j, j)`.

    let mut row_vec = Vec::with_capacity(non_zero_domain.size());
    let mut col_vec = Vec::with_capacity(non_zero_domain.size());
    let mut val_vec = Vec::with_capacity(non_zero_domain.size());
    let mut inverses = Vec::with_capacity(non_zero_domain.size());
    let mut count = 0;

    // Recall that we are computing the arithmetization of M^*,
    // where `M^*(i, j) := M(j, i) * u_H(j, j)`.
    for (r, row) in matrix.iter().enumerate() {
        for (val, i) in row {
            let row_val = elems[r];
            let col_val = elems[constraint_domain.reindex_by_subdomain(input_domain, *i)];

            // We are dealing with the transpose of M
            row_vec.push(col_val);
            col_vec.push(row_val);
            val_vec.push(*val);
            inverses.push(eq_poly_vals[&col_val]);

            count += 1;
        }
    }

    batch_inversion::<Scalar>(&mut inverses);

    cfg_iter_mut!(val_vec)
        .zip_eq(inverses)
        .for_each(|(v, inv)| *v *= inv);

    for _ in count..non_zero_domain.size() {
        col_vec.push(elems[0]);
        row_vec.push(elems[0]);
        val_vec.push(Scalar::ZERO);
    }

    let row_col_vec: Vec<_> = row_vec
        .iter()
        .zip_eq(&col_vec)
        .map(|(row, col)| *row * col)
        .collect();

    let row_evals_on_K = EvaluationsOnDomain::from_vec_and_domain(row_vec, *non_zero_domain);
    let col_evals_on_K = EvaluationsOnDomain::from_vec_and_domain(col_vec, *non_zero_domain);
    let val_evals_on_K = EvaluationsOnDomain::from_vec_and_domain(val_vec, *non_zero_domain);
    let row_col_evals_on_K =
        EvaluationsOnDomain::from_vec_and_domain(row_col_vec, *non_zero_domain);
    MatrixEvals {
        row: row_evals_on_K,
        col: col_evals_on_K,
        row_col: row_col_evals_on_K,
        val: val_evals_on_K,
    }
}

// TODO for debugging: add test that checks result of arithmetize_matrix(M).
pub(crate) fn arithmetize_matrix(label: &str, matrix_evals: MatrixEvals) -> MatrixArithmetization {
    let row = matrix_evals.row.clone().interpolate();
    let col = matrix_evals.col.clone().interpolate();
    let val = matrix_evals.val.clone().interpolate();
    let row_col = matrix_evals.row_col.clone().interpolate();

    MatrixArithmetization {
        row: LabeledPolynomial::new("row_".to_string() + label, row, None, None),
        col: LabeledPolynomial::new("col_".to_string() + label, col, None, None),
        val: LabeledPolynomial::new("val_".to_string() + label, val, None, None),
        row_col: LabeledPolynomial::new("row_col_".to_string() + label, row_col, None, None),
        evals_on_K: matrix_evals,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::marlin::num_non_zero;
    use snarkvm_curves::bls12_377::Fr as F;
    use snarkvm_fields::{One, Zero};

    fn entry(matrix: &Matrix<F>, row: usize, col: usize) -> F {
        matrix[row]
            .iter()
            .find_map(|(f, i)| (i == &col).then_some(*f))
            .unwrap_or_else(F::zero)
    }

    #[test]
    fn check_arithmetization() {
        let a = vec![
            vec![(F::one(), 1), (F::one(), 2)],
            vec![(F::one(), 3)],
            vec![(F::one(), 3)],
            vec![(F::one(), 0), (F::one(), 1), (F::one(), 5)],
            vec![(F::one(), 1), (F::one(), 2), (F::one(), 6)],
            vec![(F::one(), 2), (F::one(), 5), (F::one(), 7)],
            vec![(F::one(), 3), (F::one(), 4), (F::one(), 6)],
            vec![(F::one(), 0), (F::one(), 6), (F::one(), 7)],
        ];

        let b = vec![
            vec![],
            vec![(F::one(), 1)],
            vec![(F::one(), 0)],
            vec![(F::one(), 2)],
            vec![(F::one(), 3)],
            vec![(F::one(), 4)],
            vec![(F::one(), 5)],
            vec![(F::one(), 6)],
        ];

        let c = vec![
            vec![],
            vec![(F::one(), 7)],
            vec![],
            vec![],
            vec![],
            vec![(F::one(), 3)],
            vec![],
            vec![],
        ];

        let constraint_domain = EvaluationDomain::new(2 + 6).unwrap();
        let input_domain = EvaluationDomain::new(2).unwrap();
        let inverse_map = constraint_domain
            .elements()
            .enumerate()
            .map(|(i, e)| (e, i))
            .collect::<HashMap<_, _>>();
        let elements = constraint_domain.elements().collect::<Vec<_>>();
        let reindexed_inverse_map = (0..constraint_domain.size())
            .map(|i| {
                let reindexed_i = constraint_domain.reindex_by_subdomain(&input_domain, i);
                (elements[reindexed_i], i)
            })
            .collect::<HashMap<_, _>>();
        let eq_poly_vals: HashMap<F, F> = constraint_domain
            .elements()
            .zip_eq(
                constraint_domain
                    .batch_eval_unnormalized_bivariate_lagrange_poly_with_same_inputs(),
            )
            .collect();

        let (constraint_domain_elements, constraint_domain_eq_poly_vals) =
            precomputation_for_matrix_evals(&constraint_domain);
        for (matrix, label) in [(a, "a"), (b, "b"), (c, "c")] {
            let num_non_zero = num_non_zero(&matrix);
            let interpolation_domain = EvaluationDomain::new(num_non_zero).unwrap();

            let evals = matrix_evals(
                &matrix,
                &interpolation_domain,
                &constraint_domain,
                &input_domain,
                &constraint_domain_elements,
                &constraint_domain_eq_poly_vals,
            );
            let arith = arithmetize_matrix(label, evals);

            for (k_index, k) in interpolation_domain.elements().enumerate() {
                let row_val = arith.row.evaluate(k);
                let col_val = arith.col.evaluate(k);

                let inverse = (eq_poly_vals[&row_val]).inverse().unwrap();
                // we're in transpose land.

                let val = arith.val.evaluate(k);
                assert_eq!(arith.evals_on_K.row[k_index], row_val);
                assert_eq!(arith.evals_on_K.col[k_index], col_val);
                assert_eq!(arith.evals_on_K.val[k_index], val);
                if k_index < num_non_zero {
                    let col = *dbg!(reindexed_inverse_map.get(&row_val).unwrap());
                    let row = *dbg!(inverse_map.get(&col_val).unwrap());
                    assert!(matrix[row].iter().any(|(_, c)| *c == col));
                    assert_eq!(val, inverse * entry(&matrix, row, col),);
                }
            }
        }
    }
}
