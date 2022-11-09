/// Algebraic holographic proofs.
pub mod ahp;
pub use ahp::*;

/// Errors.
pub mod errors;
pub use errors::*;

/// Describes data structures and the algorithms used by the AHP indexer.
pub mod indexer;
pub(crate) use indexer::*;

pub(crate) mod matrices;

/// Describes data structures and the algorithms used by the AHP prover.
pub mod prover;

/// Describes data structures and the algorithms used by the AHP verifier.
pub mod verifier;
