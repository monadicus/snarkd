use snarkd_crypto::bls12_377::Scalar;

#[derive(Clone, PartialEq, Eq)]
pub struct ViewKey(pub Scalar);
