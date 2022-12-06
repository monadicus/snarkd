use super::{ComputeKey, Signature};
use crate::{
    bls12_377::{Affine, Field, Fp, G1Affine, Projective, Scalar},
    utils::*,
};

#[derive(Clone, PartialEq, Eq)]
pub struct PrivateKey {
    /// The derived signature secret key.
    pub sk_sig: Scalar,
    /// The derived signature randomizer.
    pub r_sig: Scalar,
}

impl PrivateKey {
    /// Creates a new private key from the given data.
    pub fn new(sk_sig: Scalar, r_sig: Scalar) -> Self {
        Self { sk_sig, r_sig }
    }

    /// Samples a new random private key.
    pub fn rand() -> Self {
        Self {
            sk_sig: Scalar::rand(),
            r_sig: Scalar::rand(),
        }
    }

    /// Returns a signature for the given message (as field elements) using the private key.
    pub fn sign(&self, message: &[Fp]) -> Signature {
        // Sample a random nonce from the scalar field.
        let nonce = Scalar::rand();
        // Compute `g_r` as `nonce * G`.
        let g_r = (G1Affine::prime_subgroup_generator() * nonce).to_affine();

        // Derive the compute key from the private key.
        let compute_key = ComputeKey::from(self);
        // Retrieve pk_sig.
        let pk_sig = compute_key.public_key_signature;
        // Retrieve pr_sig.
        let pr_sig = compute_key.public_randomness_signature;

        // Derive the address from the compute key.
        let address = compute_key.to_address();

        // Construct the hash input as (r * G, pk_sig, pr_sig, address, message).
        let mut preimage = Vec::with_capacity(4 + message.len());
        preimage.extend([g_r, pk_sig, pr_sig, address.0].map(|point| point.x));
        preimage.extend(message);

        // Compute the verifier challenge.
        let mut sponge = PoseidonSponge::default();
        sponge.absorb_native_field_elements(&preimage);
        let challenge = sponge.squeeze_short_nonnative_field_element();
        // Compute the prover response.
        let response = nonce - (challenge * self.sk_sig);

        // Output the signature.
        Signature::new(challenge, response, compute_key)
    }
}
