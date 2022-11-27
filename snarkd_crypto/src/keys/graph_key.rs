<<<<<<< HEAD:snarkd_crypto/src/keys/graph_key.rs
use crate::bls12_377::Fp;
=======
use snarkd_crypto::bls12_377::Fp;
>>>>>>> main:snarkd_common/src/objects/keys/graph_key.rs

#[derive(Clone, PartialEq, Eq)]
pub struct GraphKey {
    pub sk_tag: Fp,
}
