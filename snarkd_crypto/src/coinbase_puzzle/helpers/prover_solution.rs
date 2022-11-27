use super::*;
use anyhow::{anyhow, Result};

/// The prover solution for the coinbase puzzle from a prover.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ProverSolution {
    /// The core data of the prover solution.
    partial_solution: PartialSolution,
    /// The proof for the solution.
    proof: PuzzleProof,
}

impl ProverSolution {
    /// Initializes a new instance of the prover solution.
    pub const fn new(partial_solution: PartialSolution, proof: PuzzleProof) -> Self {
        Self {
            partial_solution,
            proof,
        }
    }

    /// Returns `true` if the prover solution is valid.
    pub fn verify(
        &self,
        verifying_key: &CoinbaseVerifyingKey,
        epoch_challenge: &EpochChallenge,
        proof_target: u64,
    ) -> Result<bool> {
        // Ensure the proof is non-hiding.
        if self.proof.is_hiding() {
            return Ok(false);
        }

        // Ensure that the prover solution is greater than the proof target.
        if self.to_target()? < proof_target {
            return Err(anyhow!(
                "Prover puzzle does not meet the proof target requirements."
            ));
        }

        // Compute the prover polynomial.
        let prover_polynomial = self
            .partial_solution
            .to_prover_polynomial(epoch_challenge)?;

        // Compute the challenge point.
        let challenge_point = hash_commitment(&self.commitment().commitment)?;

        // Evaluate the epoch and prover polynomials at the challenge point.
        let epoch_evaluation = epoch_challenge.epoch_polynomial().evaluate(challenge_point);
        let prover_evaluation = prover_polynomial.evaluate(challenge_point);

        // Compute the claimed value by multiplying the evaluations.
        let claimed_value = epoch_evaluation * prover_evaluation;

        // Check the KZG proof.
        Ok(KZG10::check(
            verifying_key,
            &self.commitment().commitment,
            challenge_point,
            claimed_value,
            self.proof(),
        )?)
    }

    /// Returns the address of the prover.
    pub const fn address(&self) -> Address {
        self.partial_solution.address()
    }

    /// Returns the nonce for the solution.
    pub const fn nonce(&self) -> u64 {
        self.partial_solution.nonce()
    }

    /// Returns the commitment for the solution.
    pub const fn commitment(&self) -> PuzzleCommitment {
        self.partial_solution.commitment()
    }

    /// Returns the proof for the solution.
    pub const fn proof(&self) -> &PuzzleProof {
        &self.proof
    }

    /// Returns the prover polynomial.
    pub fn to_prover_polynomial(
        &self,
        epoch_challenge: &EpochChallenge,
    ) -> Result<DensePolynomial> {
        self.partial_solution.to_prover_polynomial(epoch_challenge)
    }

    /// Returns the target of the solution.
    pub fn to_target(&self) -> Result<u64> {
        self.partial_solution.to_target()
    }
}
