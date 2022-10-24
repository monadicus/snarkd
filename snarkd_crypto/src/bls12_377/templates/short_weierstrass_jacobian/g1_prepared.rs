#[derive(Clone, Debug, PartialEq, Eq, Hash, CanonicalSerialize, CanonicalDeserialize)]
pub struct G1Prepared<P: Bls12Parameters>(pub G1Affine<P>);

impl<P: Bls12Parameters> G1Prepared<P> {
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn from_affine(p: G1Affine<P>) -> Self {
        G1Prepared(p)
    }
}

impl<P: Bls12Parameters> Default for G1Prepared<P> {
    fn default() -> Self {
        G1Prepared(G1Affine::<P>::prime_subgroup_generator())
    }
}
