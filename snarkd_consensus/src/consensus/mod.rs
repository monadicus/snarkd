use std::sync::Arc;

use anyhow::Result;
use snarkd_common::objects::{Transaction, TransactionID};
use snarkd_storage::{
    BlockStatus,
    ForkDescription, Database,
};

use self::memory_pool::{MemoryPool, MempoolEntry};

mod commit;
mod transaction;
mod memory_pool;

pub struct Consensus {
    pub(crate) memory_pool: MemoryPool,
    pub(crate) storage: Arc<Database>,
}

impl Consensus {
    pub fn new(storage: Arc<Database>) -> Self {
        Self {
            memory_pool: MemoryPool::new(),
            storage,
        }
    }
    /// Adds entry to memory pool if valid in the current ledger.
    pub(crate) fn insert_into_mempool(
        &mut self,
        transaction: Transaction,
    ) -> Result<Option<TransactionID>> {
        if self.memory_pool.transactions.contains_key(transaction.id()) {
            return Ok(None);
        }

        let id = transaction.id().clone();

        self.memory_pool
            .transactions
            .insert(transaction.id().clone(), MempoolEntry {
                size_in_bytes: transaction.size(),
                transaction,
            });

        Ok(Some(id))
    }

    /// Cleanse the memory pool of outdated transactions.
    pub(crate) fn cleanse_memory_pool(&mut self) -> Result<()> {
        let old_mempool = std::mem::take(&mut self.memory_pool);

        for (_, entry) in &old_mempool.transactions {
            if let Err(e) = self.insert_into_mempool(entry.transaction.clone()) {
                self.memory_pool = old_mempool;
                return Err(e);
            }
        }

        Ok(())
    }
}

pub const OLDEST_FORK_THRESHOLD: usize = u32::MAX as usize;
pub const ALEO_DENOMINATION: i64 = 100000;

/// Calculate a block reward that halves every 4 years * 365 days * 24 hours * 100 blocks/hr = 3,504,000 blocks.
pub fn get_block_reward(block_num: u32) -> i64 {
    let expected_blocks_per_hour: u32 = 100;
    let num_years = 4;
    let block_segments = num_years * 365 * 24 * expected_blocks_per_hour;

    let initial_reward = 150i64 * ALEO_DENOMINATION;

    // The block reward halves at most 2 times - minimum is 37.5 ALEO after 8 years.
    let num_halves = u32::min(block_num / block_segments, 2);
    let reward = initial_reward / (2_u64.pow(num_halves)) as i64;

    reward
}
