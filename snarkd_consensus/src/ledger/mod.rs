use std::{
    ops::{Deref, DerefMut},
};

use snarkd_common::Digest;

pub(crate) mod dummy;
mod merkle;
pub use merkle::MerkleLedger;
mod indexed_merkle_tree;
pub use indexed_merkle_tree::IndexedMerkleTree;
mod indexed_digests;
pub use indexed_digests::IndexedDigests;

mod merkle_tree;

pub trait Ledger: Send + Sync {
    fn extend(
        &mut self,
        new_commitments: &[Digest],
        new_serial_numbers: &[Digest],
        new_memos: &[Digest],
    ) -> Result<Digest>;

    /// Pushes raw ledger digests into the ledger -- used when committing multiple blocks at a time
    fn push_interim_digests(&mut self, new_ledger_digests: &[Digest]) -> Result<()>;

    fn rollback(&mut self, commitments: &[Digest], serial_numbers: &[Digest], memos: &[Digest]) -> Result<()>;

    fn clear(&mut self);

    fn commitment_len(&self) -> usize;

    fn contains_commitment(&self, commitment: &Digest) -> bool;

    fn commitment_index(&self, commitment: &Digest) -> Option<usize>;

    fn contains_serial(&self, serial: &Digest) -> bool;

    fn contains_memo(&self, memo: &Digest) -> bool;

    fn validate_digest(&self, digest: &Digest) -> bool;

    fn digest(&self) -> Digest;

    fn generate_proof(&self, commitment: &Digest, index: usize) -> Result<Vec<(Digest, Digest)>>;

    /// checks if a ledgers state is consistent
    fn validate_ledger(&self) -> bool;

    fn requires_async_task(&self, new_commitments_len: usize, new_serial_numbers_len: usize) -> bool;
}

pub struct DynLedger(pub Box<dyn Ledger>);

impl DynLedger {
    pub fn dummy() -> Self {
        Self(Box::new(dummy::DummyLedger))
    }
}

impl Deref for DynLedger {
    type Target = dyn Ledger;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl DerefMut for DynLedger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}
