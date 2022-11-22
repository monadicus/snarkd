use snarkd_crypto::bls12_377::G1Affine;

#[derive(Clone, PartialEq, Eq)]
pub struct Address(pub G1Affine);

impl Address {
    /// Returns a new address from the given group element.
    pub fn new(group: G1Affine) -> Self {
        Self(group)
    }
}