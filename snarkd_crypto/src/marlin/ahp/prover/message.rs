use crate::bls12_377::Scalar;

/// The prover message in the third round.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ThirdMessage {
    pub sum_a: Scalar,
    pub sum_b: Scalar,
    pub sum_c: Scalar,
}
