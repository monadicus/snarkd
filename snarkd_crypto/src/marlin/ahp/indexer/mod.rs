#![allow(non_snake_case)]

use crate::bls12_377::Scalar;

mod circuit;
pub(crate) use circuit::*;

mod circuit_info;
pub(crate) use circuit_info::*;

mod constraint_system;
pub(crate) use constraint_system::*;

mod indexer;

/// Represents a matrix.
pub(crate) type Matrix = Vec<Vec<(Scalar, usize)>>;

pub(crate) fn num_non_zero(joint_matrix: &Matrix) -> usize {
    joint_matrix.iter().map(|row| row.len()).sum()
}
