use crate::{
    fft::EvaluationDomain,
    marlin::{ahp::indexer::*, CircuitProvingKey, PreparedCircuitVerifyingKey},
    polycommit::sonic_pc,
    Prepare,
};

/// Verification key for a specific index (i.e., R1CS matrices).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircuitVerifyingKey {
    /// Stores information about the size of the circuit, as well as its defined field.
    pub circuit_info: CircuitInfo,
    /// Commitments to the indexed polynomials.
    pub circuit_commitments: Vec<sonic_pc::Commitment>,
    /// The verifier key for this index, trimmed from the universal SRS.
    pub verifier_key: sonic_pc::VerifierKey,
}

impl Prepare for CircuitVerifyingKey {
    type Prepared = PreparedCircuitVerifyingKey;

    /// Prepare the circuit verifying key.
    fn prepare(&self) -> Self::Prepared {
        let constraint_domain_size =
            EvaluationDomain::compute_size_of_domain(self.circuit_info.num_constraints).unwrap()
                as u64;
        let non_zero_a_domain_size =
            EvaluationDomain::compute_size_of_domain(self.circuit_info.num_non_zero_a).unwrap()
                as u64;
        let non_zero_b_domain_size =
            EvaluationDomain::compute_size_of_domain(self.circuit_info.num_non_zero_b).unwrap()
                as u64;
        let non_zero_c_domain_size =
            EvaluationDomain::compute_size_of_domain(self.circuit_info.num_non_zero_b).unwrap()
                as u64;

        PreparedCircuitVerifyingKey {
            constraint_domain_size,
            non_zero_a_domain_size,
            non_zero_b_domain_size,
            non_zero_c_domain_size,
            orig_vk: (*self).clone(),
        }
    }
}

impl From<CircuitProvingKey> for CircuitVerifyingKey {
    fn from(other: CircuitProvingKey) -> Self {
        other.circuit_verifying_key
    }
}

impl<'a> From<&'a CircuitProvingKey> for CircuitVerifyingKey {
    fn from(other: &'a CircuitProvingKey) -> Self {
        other.circuit_verifying_key.clone()
    }
}

impl From<PreparedCircuitVerifyingKey> for CircuitVerifyingKey {
    fn from(other: PreparedCircuitVerifyingKey) -> Self {
        other.orig_vk
    }
}

impl CircuitVerifyingKey {
    /// Iterate over the commitments to indexed polynomials in `self`.
    pub fn iter(&self) -> impl Iterator<Item = &sonic_pc::Commitment> {
        self.circuit_commitments.iter()
    }
}
