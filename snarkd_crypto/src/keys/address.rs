use crate::bls12_377::G1Affine;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Address(pub G1Affine);

impl Address {
    /// Returns a new address from the given group element.
    pub fn new(group: G1Affine) -> Self {
        Self(group)
    }
}
