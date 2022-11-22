use super::{Address, ComputeKey};
use snarkd_crypto::bls12_377::Scalar;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Signature {
    /// The verifier challenge to check against.
    pub challenge: Scalar,
    /// The prover response to the challenge.
    pub response: Scalar,
    /// The compute key of the prover.
    pub compute_key: ComputeKey,
}

impl Signature {
    /// Returns a new [`Signature`] from the given data.
    pub fn new(challenge: Scalar, response: Scalar, compute_key: ComputeKey) -> Self {
        Self {
            challenge,
            response,
            compute_key,
        }
    }

    /// Returns the signer address.
    pub fn signer_address(&self) -> Address {
        self.compute_key.to_address()
    }
}
