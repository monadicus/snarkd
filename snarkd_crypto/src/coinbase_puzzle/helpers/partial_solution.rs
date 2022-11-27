use super::*;
use sha2::{Digest, Sha256};

pub fn double_sha256(data: &[u8]) -> [u8; 32] {
    let digest = Sha256::digest(Sha256::digest(data));
    let mut ret = [0u8; 32];
    ret.copy_from_slice(&digest);
    ret
}

pub fn sha256d_to_u64(data: &[u8]) -> u64 {
    let hash_slice = double_sha256(data);
    let mut hash = [0u8; 8];
    hash[..].copy_from_slice(&hash_slice[..8]);
    u64::from_le_bytes(hash)
}

/// The partial solution for the coinbase puzzle from a prover.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PartialSolution {
    /// The address of the prover.
    address: Address,
    /// The nonce for the solution.
    nonce: u64,
    /// The commitment for the solution.
    commitment: PuzzleCommitment,
}

impl PartialSolution {
    /// Initializes a new instance of the partial solution.
    pub fn new<C: Into<PuzzleCommitment>>(address: Address, nonce: u64, commitment: C) -> Self {
        Self {
            address,
            nonce,
            commitment: commitment.into(),
        }
    }

    /// Returns the address of the prover.
    pub const fn address(&self) -> Address {
        self.address
    }

    /// Returns the nonce for the solution.
    pub const fn nonce(&self) -> u64 {
        self.nonce
    }

    /// Returns the commitment for the solution.
    pub const fn commitment(&self) -> PuzzleCommitment {
        self.commitment
    }

    /// Returns the prover polynomial.
    pub fn to_prover_polynomial(
        &self,
        epoch_challenge: &EpochChallenge,
    ) -> Result<DensePolynomial> {
        CoinbasePuzzle::prover_polynomial(epoch_challenge, self.address(), self.nonce())
    }

    /// Returns the target of the solution.
    pub fn to_target(&self) -> Result<u64> {
        let mut bytes = Vec::with_capacity(96);
        bytes.extend(
            self.commitment
                .commitment
                .0
                .x
                .0
                .to_le_bytes::<48>()
                .into_iter(),
        );
        bytes.extend(
            self.commitment
                .commitment
                .0
                .y
                .0
                .to_le_bytes::<48>()
                .into_iter(),
        );
        let hash_to_u64 = sha256d_to_u64(&bytes);
        if hash_to_u64 == 0 {
            Ok(u64::MAX)
        } else {
            Ok(u64::MAX / hash_to_u64)
        }
    }
}
