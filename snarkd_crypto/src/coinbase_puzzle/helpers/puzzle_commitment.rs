use super::*;
use std::ops::Deref;

/// A coinbase puzzle commitment to a polynomial.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PuzzleCommitment {
    /// The commitment for the solution.
    pub commitment: KZGCommitment,
}

impl PuzzleCommitment {
    /// Initializes a new instance of the puzzle commitment.
    pub const fn new(commitment: KZGCommitment) -> Self {
        Self { commitment }
    }
}

impl From<KZGCommitment> for PuzzleCommitment {
    /// Initializes a new instance of the puzzle commitment.
    fn from(commitment: KZGCommitment) -> Self {
        Self::new(commitment)
    }
}

impl Deref for PuzzleCommitment {
    type Target = KZGCommitment;

    fn deref(&self) -> &Self::Target {
        &self.commitment
    }
}
