use super::ComputeKey;
use super::Scalar;

#[derive(Clone, PartialEq, Eq)]
pub struct Signature {
    pub challenge: Scalar,
    pub response: Scalar,
    pub compute_key: ComputeKey,
}
