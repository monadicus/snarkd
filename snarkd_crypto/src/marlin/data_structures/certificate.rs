use crate::polycommit::sonic_pc;

/// A certificate for the verifying key.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Certificate {
    /// An evaluation proof from the polynomial commitment.
    pub pc_proof: sonic_pc::BatchLCProof,
}

impl Certificate {
    /// Construct a new certificate.
    pub fn new(pc_proof: sonic_pc::BatchLCProof) -> Self {
        Self { pc_proof }
    }
}
