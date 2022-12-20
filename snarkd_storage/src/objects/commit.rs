use std::collections::{HashMap, VecDeque};

use anyhow::{anyhow, bail, Result, Context};
use log::debug;
use rusqlite::params;
use snarkd_common::{objects::Block, Digest, DigestTree};

use crate::{db::InnerDatabase, BlockStatus, Database};

#[derive(Debug)]
pub struct CanonData {
    /// Current block height of canon
    pub block_height: usize,
    /// Current hash of canon block
    pub hash: Digest,
}

pub struct ForkPath {
    /// Index of the canon block this fork is based on.
    pub base_index: u32,
    /// Set of digests from `base_index`'s corresponding block to the target block
    pub path: Vec<Digest>,
}

pub enum ForkDescription {
    /// A valid fork path was found from a canon block
    Path(ForkPath),
    /// There might be a valid fork path, but it was too long to tell
    TooLong,
    /// The block never found a canon ancestor
    Orphan,
}

impl CanonData {
    pub fn is_empty(&self) -> bool {
        self.block_height == 0 && self.hash.is_empty()
    }
}

impl Database {
    pub async fn commit_block(&self, hash: Digest, previous_state_root: Digest) -> Result<BlockStatus> {
        self.call(move |x| x.commit_block(&hash, &previous_state_root)).await
    }

    pub async fn canon(&self) -> Result<CanonData> {
        self.call(|x| x.canon()).await
    }

    pub async fn get_fork_path(
        &self,
        hash: Digest,
        oldest_fork_threshold: usize,
    ) -> Result<ForkDescription> {
        self.call(move |x| x.get_fork_path(&hash, oldest_fork_threshold)).await
    }

    pub async fn recommit_blockchain(&self, root_hash: Digest) -> Result<()> {
        self.call(move |x| x.recommit_blockchain(&root_hash)).await
    }

    pub async fn recommit_block(&self, hash: Digest) -> Result<BlockStatus> {
        self.call(move |x| x.recommit_block(&hash)).await
    }

    pub async fn get_block_digest_tree(&self, hash: Digest) -> Result<DigestTree> {
        self.call(move |x| x.get_block_digest_tree(&hash)).await
    }

    pub async fn decommit_blocks(&self, hash: Digest) -> Result<Vec<Block>> {
        self.call(move |x| x.decommit_blocks(&hash)).await
    }
}

impl InnerDatabase {
    /// Commits a block into canon.
    pub fn commit_block(
        &mut self,
        hash: &Digest,
        previous_state_root: &Digest,
    ) -> Result<BlockStatus> {
        self.optimize()?;

        let canon = self.canon()?;
        match self.get_block_state(hash)? {
            BlockStatus::Committed(_) => {
                bail!("attempted to recommit block {hash}");
            }
            BlockStatus::Unknown => bail!("attempted to commit unknown block"),
            _ => (),
        }
        let next_canon_height = if canon.is_empty() {
            0
        } else {
            canon.block_height + 1
        };
        let inner_hash = hash.clone();
        self.connection.execute(
            r"UPDATE blocks SET canon_height = ?, previous_state_root = ? WHERE hash = ?",
            params![next_canon_height, previous_state_root, inner_hash],
        )?;
        self.get_block_state(hash)
    }

    pub fn recommit_blockchain(&mut self, root_hash: &Digest) -> Result<()> {
        let canon = self.canon()?;
        match self.get_block_state(root_hash)? {
            BlockStatus::Committed(_) => {
                return Err(anyhow!("attempted to recommit block {}", root_hash));
            }
            BlockStatus::Unknown => return Err(anyhow!("attempted to commit unknown block")),
            _ => (),
        }
        let next_canon_height = if canon.is_empty() {
            0
        } else {
            canon.block_height + 1
        };
        self.connection.execute(
            r"
            WITH RECURSIVE
                children(parent, sub, length) AS (
                    SELECT ?, NULL, 0 as length
                    UNION ALL
                    SELECT blocks.hash, blocks.previous_block_hash, children.length + 1 FROM blocks
                    INNER JOIN children
                    WHERE blocks.previous_block_hash = children.parent
                ),
                preferred_tip AS (
                    SELECT parent, sub, length FROM children
                    WHERE length = (SELECT max(length) FROM children)
                    ORDER BY parent
                    LIMIT 1
                ),
                total_tip(parent, remaining, digest) AS (
                    SELECT preferred_tip.parent, preferred_tip.length, NULL FROM preferred_tip
                    UNION ALL
                    SELECT blocks.previous_block_hash, total_tip.remaining - 1, blocks.canon_ledger_digest
                    FROM total_tip
                    INNER JOIN blocks ON blocks.hash = total_tip.parent
                )
                UPDATE blocks SET
                    canon_height = total_tip.remaining + ?
                FROM total_tip
                WHERE
                    total_tip.parent = blocks.hash
                    AND total_tip.digest IS NOT NULL;
            ",
            params![root_hash, next_canon_height],
        )?;
        Ok(())
    }

    /// Attempts to recommit a block into canon if it has a ledger digest.
    pub fn recommit_block(&mut self, hash: &Digest) -> Result<BlockStatus> {
        let canon = self.canon()?;
        match self.get_block_state(hash)? {
            BlockStatus::Committed(_) => {
                return Err(anyhow!("attempted to recommit block {}", hash));
            }
            BlockStatus::Unknown => return Err(anyhow!("attempted to commit unknown block")),
            _ => (),
        }
        let next_canon_height = if canon.is_empty() {
            0
        } else {
            canon.block_height + 1
        };
        self.connection.execute(
            r"UPDATE blocks SET canon_height = ? WHERE hash = ? AND canon_ledger_digest IS NOT NULL",
            params![next_canon_height, hash],
        )?;
        self.get_block_state(hash)
    }

    /// Decommits a block and all descendent blocks, returning them in ascending order
    pub fn decommit_blocks(&mut self, hash: &Digest) -> Result<Vec<Block>> {
        self.optimize()?;

        match self.get_block_state(hash)? {
            BlockStatus::Committed(_) => (),
            _ => return Err(anyhow!("attempted to decommit uncommitted block")),
        }
        let canon = self.canon()?;
        if canon.block_height == 0 {
            return Err(anyhow!("cannot decommit genesis block"));
        }
        let mut decommitted = vec![];

        let mut last_hash = canon.hash;
        loop {
            let Some(block) = self.get_block(&last_hash)? else {
                bail!("missing block in chain, database corrupt? {last_hash}");
            };
            let block_number = match self.get_block_state(&last_hash)? {
                BlockStatus::Unknown => return Err(anyhow!("unknown block state")),
                BlockStatus::Committed(n) => n as u32,
                BlockStatus::Uncommitted => return Err(anyhow!("uncommitted block in decommit")),
            };

            debug!("Decommitting block {} ({})", last_hash, block_number);

            self.connection.execute(
                r"UPDATE blocks SET canon_height = NULL WHERE hash = ?",
                [&last_hash],
            )?;

            let new_last_hash = block.header.previous_hash.clone();
            decommitted.push(block);
            if &last_hash == hash {
                break;
            }
            last_hash = new_last_hash;
        }

        Ok(decommitted)
    }

    /// Gets the current canon height of storage
    pub fn canon_height(&mut self) -> Result<u32> {
        self.optimize()?;

        self.connection
            .query_row(
                r"SELECT coalesce(max(canon_height), 0) FROM blocks",
                [],
                |row| row.get::<_, u32>(0),
            )
            .map_err(Into::into)
    }

    /// Gets the current canon state of storage
    pub fn canon(&mut self) -> Result<CanonData> {
        self.optimize()?;

        let canon_height = self.canon_height()?;

        let hash = self.get_block_hash(canon_height)?;
        // handle genesis
        if hash.is_none() && canon_height == 0 {
            return Ok(CanonData {
                block_height: 0,
                hash: Digest::default(), // empty
            });
        }
        Ok(CanonData {
            block_height: canon_height as usize,
            hash: hash.ok_or_else(|| anyhow!("missing canon block"))?,
        })
    }

    /// Gets the longest, committed or uncommitted, chain of blocks originating from `block_hash`, including `block_hash`.
    pub fn longest_child_path(&mut self, block_hash: &Digest) -> Result<Vec<Digest>> {
        self.optimize()?;

        let mut stmt = self.connection.prepare_cached(
            r"
            WITH RECURSIVE
                children(parent, sub, length) AS (
                    SELECT ?, NULL, 0 as length
                    UNION ALL
                    SELECT blocks.hash, blocks.previous_block_hash, children.length + 1 FROM blocks
                    INNER JOIN children
                    WHERE blocks.previous_block_hash = children.parent
                ),
                preferred_tip AS (
                    SELECT parent, sub, length FROM children
                    WHERE length = (SELECT max(length) FROM children)
                    ORDER BY parent
                    LIMIT 1
                ),
                total_tip(parent, remaining) AS (
                    SELECT preferred_tip.parent, preferred_tip.length FROM preferred_tip
                    UNION ALL
                    SELECT blocks.previous_block_hash, total_tip.remaining - 1
                    FROM total_tip
                    INNER JOIN blocks ON blocks.hash = total_tip.parent
                    WHERE total_tip.remaining > 0
                )
                SELECT total_tip.parent, total_tip.remaining FROM total_tip
                order by remaining;
        ",
        )?;
        let out = stmt
            .query_map([block_hash], |row| row.get(0))?
            .collect::<rusqlite::Result<Vec<Digest>>>()?;
        Ok(out)
    }

    /// Gets a tree structure representing all the descendents of [`block_hash`]
    pub fn get_block_digest_tree(&mut self, block_hash: &Digest) -> Result<DigestTree> {
        self.optimize()?;

        let mut stmt = self.connection.prepare_cached(
            r"
            WITH RECURSIVE
                children(parent, sub, length) AS (
                    SELECT ?, NULL, 0 as length
                    UNION ALL
                    SELECT blocks.hash, blocks.previous_block_hash, children.length + 1 FROM blocks
                    INNER JOIN children
                    WHERE blocks.previous_block_hash = children.parent
                )
                SELECT * FROM children
                WHERE length > 0
                ORDER BY length;
        ",
        )?;
        let out = stmt
            .query_map([block_hash], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })?
            .collect::<rusqlite::Result<Vec<(Digest, Digest, u32)>>>()?;

        let mut past_leaves = HashMap::<Digest, Vec<DigestTree>>::default();
        let mut pending_leaves = HashMap::<Digest, Vec<DigestTree>>::default();
        let mut current_tree_depth = None::<u32>;
        for (hash, parent_hash, tree_depth) in out.into_iter().rev() {
            if current_tree_depth.is_none() {
                current_tree_depth = Some(tree_depth);
            } else if Some(tree_depth) != current_tree_depth {
                current_tree_depth = Some(tree_depth);

                past_leaves.clear();
                std::mem::swap(&mut past_leaves, &mut pending_leaves);
            }
            let waiting_children = past_leaves.remove(&hash).unwrap_or_default();
            let node = if !waiting_children.is_empty() {
                let max_dist = waiting_children
                    .iter()
                    .map(|x| x.longest_length())
                    .max()
                    .unwrap_or(0);
                DigestTree::Node(hash, waiting_children, max_dist)
            } else {
                DigestTree::Leaf(hash)
            };
            pending_leaves
                .entry(parent_hash)
                .or_insert_with(Vec::new)
                .push(node);
        }

        if let Some(children) = pending_leaves.remove(block_hash) {
            let max_dist = children
                .iter()
                .map(|x| x.longest_length())
                .max()
                .unwrap_or(0);

            Ok(DigestTree::Node(block_hash.clone(), children, max_dist))
        } else {
            Ok(DigestTree::Leaf(block_hash.clone()))
        }
    }

    /// Gets the immediate children of `block_hash`.
    pub fn get_block_children(&mut self, hash: &Digest) -> Result<Vec<Digest>> {
        self.optimize()?;

        let mut stmt = self.connection.prepare_cached(
            r"
            SELECT blocks.hash FROM blocks
            WHERE blocks.previous_block_hash = ?
            ORDER BY blocks.hash
        ",
        )?;
        let out = stmt
            .query_map([hash], |row| row.get(0))?
            .collect::<rusqlite::Result<Vec<Digest>>>()?;
        Ok(out)
    }

    /// scans uncommitted blocks with a known path to the canon chain for forks
    pub fn scan_forks(&mut self, scan_depth: u32) -> Result<Vec<(Digest, Digest)>> {
        self.optimize()?;

        let mut stmt = self.connection.prepare_cached(
            r"
            WITH RECURSIVE
                children(parent, sub, length) AS (
                    SELECT NULL, (select hash from blocks where canon_height = (select max(canon_height) from blocks)), 0
                    UNION ALL
                    SELECT blocks.hash, blocks.previous_block_hash, children.length + 1 FROM blocks
                    INNER JOIN children
                    WHERE blocks.hash = children.sub AND blocks.canon_height IS NOT NULL AND length <= ?
                )
                SELECT b.previous_block_hash, b.hash FROM children
                INNER JOIN blocks b ON b.previous_block_hash = children.sub AND b.hash != children.parent
                WHERE children.length > 0
                GROUP BY children.parent
                HAVING count(b.id) >= 1
                ORDER BY length;
        ",
        )?;

        let out = stmt
            .query_map([scan_depth], |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<rusqlite::Result<Vec<(Digest, Digest)>>>()?;
        Ok(out)
    }

    /// Finds a fork path from any applicable canon node within `oldest_fork_threshold` to `hash`.
    pub fn get_fork_path(
        &mut self,
        hash: &Digest,
        oldest_fork_threshold: usize,
    ) -> Result<ForkDescription> {
        let mut side_chain_path = VecDeque::new();
        let header = self.get_block_header(hash)?.context("hash doesn't exist")?;
        let canon_height = self.canon_height()?;
        let mut parent_hash = header.previous_hash;
        for _ in 0..=oldest_fork_threshold {
            // check if the part is part of the canon chain
            match self.get_block_state(&parent_hash)? {
                // This is a canon parent
                BlockStatus::Committed(block_num) => {
                    // Add the children from the latest block
                    if canon_height as usize - block_num > oldest_fork_threshold {
                        debug!("exceeded maximum fork length in extended path");
                        return Ok(ForkDescription::TooLong);
                    }
                    let longest_path = self.longest_child_path(hash)?;
                    // let descendents = self.get_block_digest_tree(hash)?;
                    debug!("longest child path terminating in {:?}", longest_path.len());
                    side_chain_path.extend(longest_path);
                    return Ok(ForkDescription::Path(ForkPath {
                        base_index: block_num as u32,
                        path: side_chain_path.into(),
                    }));
                }
                // Add to the side_chain_path
                BlockStatus::Uncommitted => {
                    side_chain_path.push_front(parent_hash.clone());
                    parent_hash = self.get_block_header(&parent_hash)?.context("parent hash doesn't exist")?.previous_hash;
                }
                BlockStatus::Unknown => {
                    return Ok(ForkDescription::Orphan);
                }
            }
        }
        Ok(ForkDescription::TooLong)
    }
}

/*

/// Gets a series of hashes used for relaying current block sync state.
async fn get_block_locator_hashes(
    &self,
    points_of_interest: Vec<Digest>,
    oldest_fork_threshold: usize,
) -> Result<Vec<Digest>>;

/// Find hashes to provide for a syncing node given `block_locator_hashes`.
async fn find_sync_blocks(&self, block_locator_hashes: &[Digest], block_count: usize) -> Result<Vec<Digest>>;

/// Gets the block and transaction index of a transaction in a block.
async fn get_transaction_location(&self, transaction_id: Digest) -> Result<Option<TransactionLocation>>;

/// Gets a transaction from a transaction id
async fn get_transaction(&self, transaction_id: Digest) -> Result<SerialTransaction> {
    let location = self
        .get_transaction_location(transaction_id)
        .await?
        .ok_or_else(|| anyhow!("transaction not found"))?;
    let block = self.get_block(&location.block_hash).await?;
    if let Some(transaction) = block.transactions.get(location.index as usize) {
        Ok(transaction.clone())
    } else {
        Err(anyhow!("missing transaction in block"))
    }
}

/// Gets all known commitments for canon chain in block-number ascending order
async fn get_commitments(&self, block_start: u32) -> Result<Vec<Digest>>;

/// Gets all known serial numbers for canon chain in block-number ascending order
async fn get_serial_numbers(&self, block_start: u32) -> Result<Vec<Digest>>;

/// Gets all known memos for canon chain in block-number ascending order
async fn get_memos(&self, block_start: u32) -> Result<Vec<Digest>>;

/// Gets all known ledger digests for canon chain in block-number ascending order
async fn get_ledger_digests(&self, block_start: u32) -> Result<Vec<Digest>>;

/// Resets stored ledger state. A maintenance function, not intended for general use.
async fn reset_ledger(
    &self,
    commitments: Vec<Digest>,
    serial_numbers: Vec<Digest>,
    memos: Vec<Digest>,
    digests: Vec<Digest>,
) -> Result<()>;

/// Gets a dump of all stored canon blocks, in block-number ascending order. A maintenance function, not intended for general use.
async fn get_canon_blocks(&self, limit: Option<u32>) -> Result<Vec<SerialBlock>>;

/// Similar to `Storage::get_canon_blocks`, gets hashes of all blocks subject to `filter` and `limit` in filter-defined order. A maintenance function, not intended for general use.
async fn get_block_hashes(&self, limit: Option<u32>, filter: BlockFilter) -> Result<Vec<Digest>>;

#[cfg(feature = "test")]
async fn reset(&self) -> Result<()>;

/// Removes non-canon blocks and transactions from the storage.
async fn trim(&self) -> Result<()>;

*/
