#![allow(non_snake_case)]

mod circuit;
pub(crate) use circuit::*;

mod circuit_info;
pub(crate) use circuit_info::*;

mod constraint_system;
pub(crate) use constraint_system::*;

mod indexer;

/// Represents a matrix.
pub(crate) type Matrix<F> = Vec<Vec<(F, usize)>>;

pub(crate) fn num_non_zero<F>(joint_matrix: &Matrix<F>) -> usize {
    joint_matrix.iter().map(|row| row.len()).sum()
}
