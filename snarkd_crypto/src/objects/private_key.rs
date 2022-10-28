use super::{Field, Scalar};

#[derive(Clone, PartialEq, Eq)]
pub struct PrivateKey {
    pub seed: Field,
    pub sk_sig: Scalar,
    pub r_sig: Scalar,
}
