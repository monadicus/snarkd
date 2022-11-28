use crate::bls12_377::Fp;

#[derive(Clone, PartialEq, Eq)]
pub struct GraphKey {
    pub sk_tag: Fp,
}
