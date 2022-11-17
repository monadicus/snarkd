use crate::{
    polycommit::sonic_pc,
    snark::marlin::{ahp::indexer::*, CircuitVerifyingKey, MarlinMode},
};
use snarkvm_curves::PairingEngine;

use std::sync::Arc;

/// Proving key for a specific circuit (i.e., R1CS matrices).
#[derive(Clone, Debug)]
pub struct CircuitProvingKey<E: PairingEngine, MM: MarlinMode> {
    /// The circuit verifying key.
    pub circuit_verifying_key: CircuitVerifyingKey<E, MM>,
    /// The randomness for the circuit polynomial commitments.
    pub circuit_commitment_randomness: Vec<sonic_pc::Randomness<E>>,
    /// The circuit itself.
    pub circuit: Arc<Circuit<E::Fr, MM>>,
    /// The committer key for this index, trimmed from the universal SRS.
    pub committer_key: Arc<sonic_pc::CommitterKey<E>>,
}
