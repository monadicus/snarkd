use crate::snark::marlin::{CircuitVerifyingKey, MarlinMode};
use snarkvm_curves::PairingEngine;

/// Verification key, prepared (preprocessed) for use in pairings.

#[derive(Clone)]
pub struct PreparedCircuitVerifyingKey<E: PairingEngine, MM: MarlinMode> {
    /// Size of the variable domain.
    pub constraint_domain_size: u64,
    /// Size of the domain that represents A.
    pub non_zero_a_domain_size: u64,
    /// Size of the domain that represents B.
    pub non_zero_b_domain_size: u64,
    /// Size of the domain that represents C.
    pub non_zero_c_domain_size: u64,
    /// Non-prepared verification key, for use in native "prepared verify" (which
    /// is actually standard verify), as well as in absorbing the original vk into
    /// the Fiat-Shamir sponge.
    pub orig_vk: CircuitVerifyingKey<E, MM>,
}
