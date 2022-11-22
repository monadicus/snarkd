use super::{Address, PrivateKey};
use snarkd_crypto::{
    bls12_377::{Affine, G1Affine, Projective, Scalar},
    utils::*,
};

//@jds: what is this?
// It's a constant prefix used for key identification
static _COMPUTE_KEY_PREFIX: [u8; 10] = [109, 249, 98, 224, 36, 15, 213, 187, 79, 190]; // AComputeKey1

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ComputeKey {
    pub public_key_signature: G1Affine,
    pub public_randomness_signature: G1Affine,
    pub prf_secret_key: Scalar,
}

impl ComputeKey {
    /// Returns the address corresponding to the compute key.
    pub fn to_address(&self) -> Address {
        // Compute pk_prf := G^sk_prf.
        let pk_prf = G1Affine::prime_subgroup_generator() * self.prf_secret_key;
        // Compute the address := pk_sig + pr_sig + pk_prf.
        Address::new(
            (self.public_key_signature.to_projective()
                + self.public_randomness_signature.to_projective()
                + pk_prf)
                .to_affine(),
        )
    }
}

impl From<&PrivateKey> for ComputeKey {
    fn from(value: &PrivateKey) -> Self {
        let public_key_signature =
            (G1Affine::prime_subgroup_generator() * value.sk_sig).to_affine();
        let public_randomness_signature =
            (G1Affine::prime_subgroup_generator() * value.r_sig).to_affine();
        let mut sponge = PoseidonSponge::default();
        sponge
            .absorb_native_field_elements(&[public_key_signature.x, public_randomness_signature.x]);
        let prf_secret_key = sponge.squeeze_short_nonnative_field_element();
        Self {
            public_key_signature,
            public_randomness_signature,
            prf_secret_key,
        }
    }
}
