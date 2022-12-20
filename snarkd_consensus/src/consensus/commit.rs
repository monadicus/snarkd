use anyhow::{bail, Context};
use log::{debug, warn};
use snarkd_common::{Digest, objects::{Block, BlockHeader}, DigestTree};

use super::*;

lazy_static::lazy_static! {
    static ref GENESIS_BLOCK: Block = {
        let mut header = BlockHeader {
            block_hash: Digest::default(),
            previous_hash: Digest::default(),
            network: 0,
            height: 0,
            coinbase_target: 100,
            timestamp: 0,
            nonce: 0,
        };
        header.block_hash = header.hash();
        Block {
            header,
            transactions: vec![],
        }
    };
}

impl Consensus {
    /// initialize genesis block if neccesary and catches up the chain
    pub async fn init(&mut self) -> Result<()> {
        let canon = self.storage.canon().await?;
        // no blocks present/genesis situation
        if canon.is_empty() {
            // no blocks
            let hash = GENESIS_BLOCK.header.hash();
            self.storage.insert_block(GENESIS_BLOCK.clone()).await?;

            self.commit_block(&hash, &GENESIS_BLOCK).await?;
        }

        Ok(())
    }

    /// Receive a block from an external source and process it based on ledger state.
    pub async fn receive_block(&mut self, block: Block) -> Result<()> {
        self.storage.insert_block(block.clone()).await?;

        let hash = block.header.hash();
        self.try_commit_block(&hash, block).await?;

        self.try_to_fast_forward().await?;

        Ok(())
    }

    pub async fn try_commit_block(&mut self, hash: &Digest, block: Block) -> Result<()> {
        let canon = self.storage.canon().await?;

        match self.storage.get_block_state(block.header.previous_hash.clone()).await? {
            BlockStatus::Committed(n) if n == canon.block_height => {
                debug!("Processing a block that is on canon chain. Height {} -> {}", n, n + 1);
                // metrics::gauge!(metrics::blocks::HEIGHT, n as f64 + 1.0);
                // Process the block now.
            }
            BlockStatus::Unknown => {
                debug!("Processing a block that is an unknown orphan");
                // metrics::increment_counter!(metrics::blocks::ORPHANS);
                // Don't process the block.
                return Ok(());
            }
            _ => {
                let fork_path = self.storage.get_fork_path(hash.clone(), super::OLDEST_FORK_THRESHOLD).await?;
                match fork_path {
                    ForkDescription::Path(fork_path) => {
                        let new_block_number = fork_path.base_index + fork_path.path.len() as u32;
                        debug!("Processing a block that is on side chain. Height {}", new_block_number);
                        // If the side chain is now longer than the canon chain,
                        // perform a fork to the side chain.
                        if new_block_number as usize > canon.block_height {
                            debug!(
                                "Determined side chain is longer than canon chain by {} blocks",
                                new_block_number as usize - canon.block_height
                            );
                            warn!("A valid fork has been detected. Performing a fork to the side chain.");

                            let head_header = self.storage.get_block_header(fork_path.path[0].clone()).await?.context("missing block")?;
                            let canon_branch_number =
                                match self.storage.get_block_state(head_header.previous_hash.clone()).await? {
                                    BlockStatus::Unknown => {
                                        bail!("failed to find parent block of fork");
                                    }
                                    BlockStatus::Committed(n) => n as u32,
                                    BlockStatus::Uncommitted => {
                                        bail!("proposed parent block of fork is non-canon");
                                    }
                                };

                            // Remove existing canon chain descendents, if any.
                            match self.storage.get_block_hash(canon_branch_number + 1).await? {
                                None => (),
                                Some(hash) => {
                                    self.decommit_ledger_block(&hash).await?;
                                }
                            };

                            {
                                let canon = self.storage.canon().await?;
                                // metrics::gauge!(metrics::blocks::HEIGHT, canon.block_height as f64);
                            }

                            self.storage.recommit_blockchain(fork_path.path[0].clone()).await?;
                            let committed_blocks =
                                self.storage.canon().await?.block_height - canon_branch_number as usize;

                            for block_hash in &fork_path.path[committed_blocks.min(fork_path.path.len())..] {
                                if block_hash == hash {
                                    self.verify_and_commit_block(hash, &block).await?;
                                } else {
                                    let new_block = self.storage.get_block(block_hash.clone()).await?.context("missing block")?;
                                    self.verify_and_commit_block(&new_block.header.hash(), &new_block)
                                        .await?;
                                }
                            }
                        } else {
                            // Don't process the block.
                            return Ok(());
                        }
                    }
                    ForkDescription::Orphan => {
                        debug!("Processing a block that is on unknown orphan chain");
                        // metrics::increment_counter!(metrics::blocks::ORPHANS);
                        // Don't process the block.
                        return Ok(());
                    }
                    ForkDescription::TooLong => {
                        debug!("Processing a block that is on an over-length fork");
                        // Don't process the block.
                        return Ok(());
                    }
                }
            }
        }

        // Process the block.
        self.verify_and_commit_block(hash, &block).await?;

        Ok(())
    }

    /// Return whether or not the given block is valid and insert it.
    /// 1. Verify that the block header is valid.
    /// 2. Verify that the transactions are valid.
    /// 3. Insert/canonize block.
    pub async fn verify_and_commit_block(
        &mut self,
        hash: &Digest,
        block: &Block,
    ) -> Result<()> {
        let now = std::time::Instant::now();

        match self.recommit_block(hash).await? {
            BlockStatus::Committed(_) => return Ok(()),
            BlockStatus::Unknown => bail!("unknown block for commit: {hash}"),
            BlockStatus::Uncommitted => (),
        }

        let canon = self.storage.canon().await?;
        let canon_header = self.storage.get_block_header(canon.hash.clone()).await?.context("missing commited block")?;

        // 1. Verify that the block valid
        if !self
            .verify_block(block, canon_header, canon.block_height as u32)
            .await?
        {
            debug!("failed to validate block '{}', deleting from storage...", hash);
            self.storage.delete_block(hash.clone()).await?;
            bail!("failed to verify block: {hash}");
        }

        // 2. Insert/canonize block
        self.commit_block(hash, block).await?;

        // 3. Remove transactions from the mempool
        for transaction in block.transactions.iter() {
            self.memory_pool.remove(transaction.id());
        }

        // metrics::histogram!(metrics::blocks::COMMIT_TIME, now.elapsed());

        Ok(())
    }

    /// Check if the block is valid.
    /// Verify transactions and transaction fees.
    pub async fn verify_block(
        &mut self,
        block: &Block,
        parent_header: BlockHeader,
        parent_height: u32,
    ) -> Result<bool> {
        if block.header.previous_hash != parent_header.hash() {
            bail!("attempted to commit a block that wasn't a direct child of tip of canon");
        }

        self.verify_transactions(&block.transactions[..]).await
    }

    async fn inner_commit_block(&mut self, block: &Block) -> Result<Digest> {
        Ok(block.header.block_hash.clone())
    }

    pub async fn commit_block(&mut self, hash: &Digest, block: &Block) -> Result<()> {
        let digest = self.inner_commit_block(block).await?;

        self.storage.commit_block(hash.clone(), digest).await?;
        self.cleanse_memory_pool()
    }

    pub async fn recommit_block(&mut self, hash: &Digest) -> Result<BlockStatus> {
        let initial_state = self.storage.get_block_state(hash.clone()).await?;
        if initial_state != BlockStatus::Uncommitted {
            return Ok(initial_state);
        }
        let out = self.storage.recommit_block(hash.clone()).await?;
        self.cleanse_memory_pool()?;
        Ok(out)
    }

    pub async fn try_to_fast_forward(&mut self) -> Result<()> {
        let canon = self.storage.canon().await?;
        let mut children = self.storage.get_block_digest_tree(canon.hash.clone()).await?;
        if matches!(&children, DigestTree::Leaf(x) if x == &canon.hash) {
            return Ok(());
        }
        debug!(
            "Attempting to canonize the descendants of block at height {}.",
            canon.block_height
        );
        loop {
            let mut sub_children = children.take_children();
            // rust doesn't believe we will always set children before the next loop
            children = DigestTree::Leaf(Digest::from([0u8; 32]));
            if sub_children.is_empty() {
                break;
            }
            sub_children.sort_by_key(|child| std::cmp::Reverse(child.longest_length()));
            debug!("Processing the next known descendant.");
            let mut last_error = None;
            for child in sub_children {
                let new_block = self.storage.get_block(child.root().clone()).await?.context("missing block")?;
                match self.try_commit_block(child.root(), new_block).await {
                    Ok(()) => {
                        children = child;
                        last_error = None;
                        break;
                    }
                    Err(e) => {
                        warn!("failed to commit descendent block, trying sibling... error: {:?}", e);
                        last_error = Some(e);
                    }
                }
            }
            if let Some(last_error) = last_error {
                return Err(last_error);
            }
        }
        Ok(())
    }

    /// removes a block and all of it's descendents from storage and ledger
    pub async fn decommit_ledger_block(&mut self, hash: &Digest) -> Result<()> {
        let decommited_blocks = self.storage.decommit_blocks(hash.clone()).await?;
        debug!("decommited {} blocks", decommited_blocks.len());
        for block in decommited_blocks.into_iter().rev() {
            debug!("ledger: rolling back block {}", block.header.hash());
        }

        self.cleanse_memory_pool()
    }
}
