//! Transactions memory pool
//!
//! `MemoryPool` keeps a vector of transactions seen by the miner.

use indexmap::IndexMap;
use snarkd_common::{objects::{Transaction, BlockHeader}, Digest};

/// Stores a transaction and it's size in the memory pool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MempoolEntry {
    pub(crate) size_in_bytes: usize,
    pub(crate) transaction: Transaction,
}

/// Stores transactions received by the server.
/// Transaction entries will eventually be fetched by the miner and assembled into blocks.
#[derive(Debug, Default)]
pub struct MemoryPool {
    /// The mapping of all unconfirmed transaction IDs to their corresponding transaction data.
    pub(crate) transactions: IndexMap<Digest, MempoolEntry>,
}

const BLOCK_HEADER_SIZE: usize = BlockHeader::size();
const COINBASE_TRANSACTION_SIZE: usize = 1490; // TODO Find the value for actual coinbase transaction size

impl MemoryPool {
    /// Initialize a new memory pool with no transactions
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Removes transaction from memory pool based on the transaction id.
    pub fn remove(&mut self, transaction_id: &Digest) -> Option<Transaction> {
        match self.transactions.remove(transaction_id) {
            Some(entry) => {
                Some(entry.transaction)
            }
            None => None,
        }
    }

    /// Get candidate transactions for a new block.
    pub fn get_candidates(&self, max_size: usize) -> Vec<&Transaction> {
        let max_size = max_size - (BLOCK_HEADER_SIZE + COINBASE_TRANSACTION_SIZE);

        let mut block_size = 0;
        let mut transactions = vec![];

        // TODO Change naive transaction selection
        for (_, entry) in self.transactions.iter() {
            if block_size + entry.size_in_bytes <= max_size {
                block_size += entry.size_in_bytes;
                transactions.push(&entry.transaction);
            }
        }

        transactions
    }
}
