<<<<<<< HEAD:snarkd_crypto/src/keys/view_key.rs
use crate::bls12_377::Scalar;
=======
use snarkd_crypto::bls12_377::Scalar;
>>>>>>> main:snarkd_common/src/objects/keys/view_key.rs

#[derive(Clone, PartialEq, Eq)]
pub struct ViewKey(pub Scalar);
