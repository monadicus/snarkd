use super::{Group, Scalar};

static _COMPUTE_KEY_PREFIX: [u8; 10] = [109, 249, 98, 224, 36, 15, 213, 187, 79, 190]; // AComputeKey1

#[derive(Clone, PartialEq, Eq)]
pub struct ComputeKey {
    pub pk_sig: Group,
    pub pr_sig: Group,
    pub sk_prg: Scalar,
}
