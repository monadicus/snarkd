use crate::{
    marlin::{ahp::indexer::*, CircuitVerifyingKey},
    polycommit::sonic_pc,
};

use std::sync::Arc;

/// Proving key for a specific circuit (i.e., R1CS matrices).
#[derive(Clone, Debug)]
pub struct CircuitProvingKey {
    /// The circuit verifying key.
    pub circuit_verifying_key: CircuitVerifyingKey,
    /// The randomness for the circuit polynomial commitments.
    pub circuit_commitment_randomness: Vec<sonic_pc::Randomness>,
    /// The circuit itself.
    pub circuit: Arc<Circuit>,
    /// The committer key for this index, trimmed from the universal SRS.
    pub committer_key: Arc<sonic_pc::CommitterKey>,
}
