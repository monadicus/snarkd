use super::*;
use anyhow::{anyhow, Result};

/// The coinbase puzzle solution constructed by accumulating the individual prover solutions.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CoinbaseSolution {
    /// The partial solutions of the coinbase puzzle, which are aggregated into a single solution.
    partial_solutions: Vec<PartialSolution>,
    /// The KZG proof of the coinbase solution.
    proof: PuzzleProof,
}

impl CoinbaseSolution {
    /// Initializes a new instance of a coinbase solution.
    pub const fn new(partial_solutions: Vec<PartialSolution>, proof: PuzzleProof) -> Self {
        Self {
            partial_solutions,
            proof,
        }
    }

    /// Returns the partial solutions.
    pub fn partial_solutions(&self) -> &[PartialSolution] {
        &self.partial_solutions
    }

    /// Returns the puzzle commitments.
    pub fn puzzle_commitments(&self) -> impl '_ + Iterator<Item = PuzzleCommitment> {
        self.partial_solutions.iter().map(|s| s.commitment())
    }

    /// Returns the KZG proof.
    pub const fn proof(&self) -> &PuzzleProof {
        &self.proof
    }

    /// Returns the number of partial solutions.
    pub fn len(&self) -> usize {
        self.partial_solutions.len()
    }

    /// Returns `true` if there are no partial solutions.
    pub fn is_empty(&self) -> bool {
        self.partial_solutions.is_empty()
    }

    /// Returns the cumulative sum of the prover solutions.
    pub fn to_cumulative_proof_target(&self) -> Result<u128> {
        // Compute the cumulative target as a u128.
        self.partial_solutions
            .iter()
            .try_fold(0u128, |cumulative, solution| {
                cumulative
                    .checked_add(solution.to_target()? as u128)
                    .ok_or_else(|| anyhow!("Cumulative target overflowed"))
            })
    }

    /// Returns the accumulator challenge point.
    pub fn to_accumulator_point(&self) -> Result<Scalar> {
        let mut challenge_points = hash_commitments(
            self.partial_solutions
                .iter()
                .map(|solution| *solution.commitment()),
        )?;
        if challenge_points.len() != self.partial_solutions.len() + 1 {
            return Err(anyhow!("Invalid number of challenge points"));
        }

        // Pop the last challenge point as the accumulator challenge point.
        match challenge_points.pop() {
            Some(point) => Ok(point),
            None => return Err(anyhow!("Missing the accumulator challenge point")),
        }
    }
}
