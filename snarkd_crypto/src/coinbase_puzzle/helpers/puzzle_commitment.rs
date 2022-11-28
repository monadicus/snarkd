use super::*;
use std::ops::Deref;

/// A coinbase puzzle commitment to a polynomial.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PuzzleCommitment {
    /// The commitment for the solution.
    commitment: Commitment,
}

impl PuzzleCommitment {
    /// Initializes a new instance of the puzzle commitment.
    pub const fn new(commitment: Commitment) -> Self {
        Self { commitment }
    }
}

impl From<Commitment> for PuzzleCommitment {
    /// Initializes a new instance of the puzzle commitment.
    fn from(commitment: Commitment) -> Self {
        Self::new(commitment)
    }
}

impl Deref for PuzzleCommitment {
    type Target = Commitment;

    fn deref(&self) -> &Self::Target {
        &self.commitment
    }
}
