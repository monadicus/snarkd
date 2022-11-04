use crate::polycommit::sonic_pc;

/// A certificate for the verifying key.
#[derive(Clone, Debug, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
pub struct Certificate<E: PairingEngine> {
    /// An evaluation proof from the polynomial commitment.
    pub pc_proof: sonic_pc::BatchLCProof<E>,
}

impl<E: PairingEngine> Certificate<E> {
    /// Construct a new certificate.
    pub fn new(pc_proof: sonic_pc::BatchLCProof<E>) -> Self {
        Self { pc_proof }
    }
}

impl<E: PairingEngine> ToBytes for Certificate<E> {
    fn write_le<W: Write>(&self, mut w: W) -> io::Result<()> {
        Self::serialize_compressed(self, &mut w)
            .map_err(|_| error("Failed to serialize certificate"))
    }
}

impl<E: PairingEngine> FromBytes for Certificate<E> {
    fn read_le<R: Read>(mut r: R) -> io::Result<Self> {
        Self::deserialize_compressed(&mut r).map_err(|_| error("Failed to deserialize certificate"))
    }
}
