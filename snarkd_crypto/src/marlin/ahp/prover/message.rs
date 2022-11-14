use snarkvm_fields::Field;
use snarkvm_utilities::{error, serialize::*, ToBytes, Write};

/// The prover message in the third round.
#[derive(Clone, Debug, Default, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
pub struct ThirdMessage<F: Field> {
    pub sum_a: F,
    pub sum_b: F,
    pub sum_c: F,
}

impl<F: Field> ToBytes for ThirdMessage<F> {
    fn write_le<W: Write>(&self, mut w: W) -> io::Result<()> {
        CanonicalSerialize::serialize_compressed(self, &mut w)
            .map_err(|_| error("Could not serialize ProverMsg"))
    }
}
