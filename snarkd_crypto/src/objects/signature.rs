use super::ComputeKey;
use super::Scalar;

#[derive(Clone, PartialEq, Eq)]
pub struct Signature {
    challenge: Scalar,
    response: Scalar,
    compute_key: ComputeKey,
}
