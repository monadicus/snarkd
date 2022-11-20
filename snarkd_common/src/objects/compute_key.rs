use super::{Group, Scalar};

//@jds: what is this?
static _COMPUTE_KEY_PREFIX: [u8; 10] = [109, 249, 98, 224, 36, 15, 213, 187, 79, 190]; // AComputeKey1

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComputeKey {
    pub public_key_signature: Group,
    pub public_randomness_signature: Group,
    pub secret_key_program: Scalar,
}
