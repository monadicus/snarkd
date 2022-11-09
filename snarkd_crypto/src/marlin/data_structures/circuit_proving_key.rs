use crate::{
    polycommit::sonic_pc,
    snark::marlin::{ahp::indexer::*, CircuitVerifyingKey, MarlinMode},
};
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

impl<E: PairingEngine, MM: MarlinMode> ToBytes for CircuitProvingKey<E, MM> {
    fn write_le<W: Write>(&self, mut writer: W) -> io::Result<()> {
        CanonicalSerialize::serialize_compressed(&self.circuit_verifying_key, &mut writer)?;
        CanonicalSerialize::serialize_compressed(&self.circuit_commitment_randomness, &mut writer)?;
        CanonicalSerialize::serialize_compressed(&self.circuit, &mut writer)?;

        self.committer_key.write_le(&mut writer)
    }
}

impl<E: PairingEngine, MM: MarlinMode> FromBytes for CircuitProvingKey<E, MM> {
    #[inline]
    fn read_le<R: Read>(mut reader: R) -> io::Result<Self> {
        let circuit_verifying_key = CanonicalDeserialize::deserialize_compressed(&mut reader)?;
        let circuit_commitment_randomness =
            CanonicalDeserialize::deserialize_compressed(&mut reader)?;
        let circuit = CanonicalDeserialize::deserialize_compressed(&mut reader)?;
        let committer_key = Arc::new(FromBytes::read_le(&mut reader)?);

        Ok(Self {
            circuit_verifying_key,
            circuit_commitment_randomness,
            circuit,
            committer_key,
        })
    }
}
