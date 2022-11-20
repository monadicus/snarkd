use crate::marlin::{ahp::AHPForR1CS, MarlinMode};

use core::marker::PhantomData;

/// Information about the circuit, including the field of definition, the number of
/// variables, the number of constraints, and the maximum number of non-zero
/// entries in any of the constraint matrices.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CircuitInfo {
    /// The number of public inputs after padding.
    pub num_public_inputs: usize,
    /// The total number of variables in the constraint system.
    pub num_variables: usize,
    /// The number of constraints.
    pub num_constraints: usize,
    /// The number of non-zero entries in the A matrix.
    pub num_non_zero_a: usize,
    /// The number of non-zero entries in the B matrix.
    pub num_non_zero_b: usize,
    /// The number of non-zero entries in the C matrix.
    pub num_non_zero_c: usize,
}

impl CircuitInfo {
    /// The maximum degree of polynomial required to represent this index in the AHP.
    pub fn max_degree(&self, zk: bool) -> usize {
        let max_non_zero = self
            .num_non_zero_a
            .max(self.num_non_zero_b)
            .max(self.num_non_zero_c);
        AHPForR1CS { mode: zk }
            .max_degree(self.num_constraints, self.num_variables, max_non_zero)
            .unwrap()
    }
}
