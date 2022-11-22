use anyhow::{anyhow, Result};
use rusqlite::{params, Connection, OptionalExtension, ToSql};
use snarkd_common::{
    objects::{
        Block, BlockHeader, ComputeKey, DeployTransaction, ExecuteTransaction, Execution, Metadata,
        Signature, Transaction, Transition,
    },
    Digest,
};

use crate::Database;

/// Current state of a block in storage
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockStatus {
    /// Block not known/not found
    Unknown,
    /// Block on canon chain @ height
    Committed(usize),
    /// Block known, but not in canon chain
    Uncommitted,
}

fn write_transition(
    connection: &Connection,
    transaction_id: i64,
    transaction_order: i32,
    transition: &Transition,
) -> Result<()> {
    let mut transition_query = connection.prepare_cached(
        r"
        INSERT OR IGNORE INTO transitions (
            transaction_id,
            transaction_order,
            transition_id,
            program_name,
            program_network,
            function_name,
            inputs,
            outputs,
            finalize,
            proof,
            tpk,
            tcm,
            fee
        )
        VALUES (
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?
        )            
    ",
    )?;

    transition_query.execute::<&[&dyn ToSql]>(&[
        &transaction_id,
        &transaction_order,
        &transition.id,
        &transition.program_id.name,
        &transition.program_id.network,
        &transition.function_name,
        &[], // TODO: inputs
        &[], // TODO: outputs,
        &[], // TODO: finalize,
        &transition.proof,
        &transition.tpk,
        &transition.tcm,
        &transition.fee,
    ])?;
    Ok(())
}

impl Database {
    /// Inserts a block into storage, not committing it.
    pub async fn insert_block(&self, block: Block) -> Result<()> {
        self.optimize().await?;
        let hash = block.header.hash();

        match self.get_block_state(hash.clone()).await? {
            BlockStatus::Unknown => (),
            BlockStatus::Committed(_) | BlockStatus::Uncommitted => {
                // metrics::increment_counter!(snarkos_metrics::blocks::DUPLICATES);
                return Err(anyhow!("duplicate block insertion"));
            }
        }

        self.call(move |db| {
            let transaction = db.transaction()?;
            {
                let mut block_query = transaction.prepare_cached(
                    r"
                INSERT INTO blocks (
                    hash,
                    previous_block_id,
                    previous_block_hash,
                    previous_state_root,
                    transactions_root,
                    network,
                    round,
                    height,
                    coinbase_target,
                    proof_target,
                    timestamp,
                    challenge,
                    response,
                    compute_key_public_key_signature,
                    compute_key_public_randomness_signature,
                    compute_key_secret_key_program
                )
                VALUES (
                    ?,
                    (SELECT id from blocks where hash = ?),
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?
                )
                ",
                )?;
                block_query.execute::<&[&dyn ToSql]>(&[
                    &hash,
                    &block.header.previous_hash,
                    &block.header.previous_hash,
                    &block.header.previous_state_root,
                    &block.header.transactions_root,
                    &block.header.metadata.network,
                    &block.header.metadata.round,
                    &block.header.metadata.height,
                    &block.header.metadata.coinbase_target,
                    &block.header.metadata.proof_target,
                    &block.header.metadata.timestamp,
                    &block.header.signature.challenge,
                    &block.header.signature.response,
                    &block.header.signature.compute_key.public_key_signature,
                    &block
                        .header
                        .signature
                        .compute_key
                        .public_randomness_signature,
                    &block.header.signature.compute_key.prf_secret_key,
                ])?;
                let block_id = transaction.last_insert_rowid();
                transaction.execute(
                    "UPDATE blocks SET previous_block_id = ? WHERE previous_block_hash = ?",
                    params![block_id, &hash],
                )?;

                let mut transaction_query = transaction.prepare_cached(
                    r"
                    INSERT OR IGNORE INTO transactions (
                        transaction_id,
                        edition,
                        transaction_type
                    )
                    VALUES (
                        ?,
                        ?,
                        ?
                    )
                ",
                )?;
                let mut transaction_block_query = transaction.prepare_cached(
                    r"
                    INSERT INTO transaction_blocks (
                        transaction_id,
                        block_id,
                        block_order
                    )
                    VALUES (
                        ?,
                        ?,
                        ?
                    )
                ",
                )?;

                for (i, to_insert) in block.transactions.into_iter().enumerate() {
                    match to_insert {
                        Transaction::Deploy(DeployTransaction {
                            id,
                            deployment,
                            transition,
                        }) => {
                            /*
                                pub edition: u16,
                                pub program: Program,
                                pub verifying_keys: IndexMap<Identifier, (VerifyingKey, Certificate)>,
                            */
                            //todo: program
                            //todo: verifying keys
                            //todo: transition
                            transaction_query.execute(params![
                                &id,
                                &deployment.edition,
                                &"deploy",
                            ])?;
                            let transaction_id = transaction.query_row(
                                r"SELECT id FROM transactions WHERE transaction_id = ?",
                                [&id],
                                |row| row.get::<usize, i64>(0),
                            )?;
                            transaction_block_query.execute(params![
                                &transaction_id,
                                block_id as usize,
                                i
                            ])?;
                            write_transition(&transaction, transaction_id, 0, &transition)?;
                        }
                        Transaction::Execute(ExecuteTransaction {
                            id,
                            execution:
                                Execution {
                                    edition,
                                    transitions,
                                },
                            transition,
                        }) => {
                            transaction_query.execute(params![&id, &edition, &"execute",])?;
                            let transaction_id = transaction.query_row(
                                r"SELECT id FROM transactions WHERE transaction_id = ?",
                                [&id],
                                |row| row.get::<usize, i64>(0),
                            )?;
                            if let Some(transition) = transition {
                                write_transition(&transaction, transaction_id, -1, &transition)?;
                            }
                            for (i, transition) in transitions.into_iter().enumerate() {
                                write_transition(
                                    &transaction,
                                    transaction_id,
                                    i as i32,
                                    &transition,
                                )?;
                            }
                            transaction_block_query.execute(params![&id, block_id as usize, i])?;
                        }
                    }
                }
            }
            transaction.commit()?;
            Ok(())
        })
        .await
    }

    /// Gets a block header and transaction blob for a given hash.
    pub async fn get_block(&self, hash: Digest) -> Result<Block> {
        todo!()
    }

    /// Deletes a block from storage, including any associated data. Must not be called on a committed block.
    pub async fn delete_block(&self, hash: Digest) -> Result<()> {
        self.optimize().await?;
        self.call(move |db| {
            let transaction = db.transaction()?;

            transaction.execute(
                r"
                DELETE FROM blocks WHERE hash = ?
            ",
                [hash],
            )?;

            // clean messy sqlite fk constraints
            transaction.execute(
                r"
                DELETE FROM transactions
                WHERE id IN (
                    SELECT t.id FROM transactions t
                    LEFT JOIN transaction_blocks tb ON tb.transaction_id = t.id WHERE tb.id IS NULL
                );
            ",
                [],
            )?;
            transaction.commit()?;
            Ok(())
        })
        .await
    }

    /// Gets a hash for a canon block number, if it exists
    pub async fn get_block_hash(&self, block_num: u32) -> Result<Option<Digest>> {
        self.optimize().await?;

        self.call(move |db| {
            Ok(db
                .query_row::<Vec<u8>, _, _>(
                    r"SELECT hash FROM blocks WHERE canon_height = ?",
                    [block_num],
                    |row| row.get(0),
                )
                .optional()?
                .map(|x| Digest::from(&x[..])))
        })
        .await
    }

    /// Gets a block header for a given hash
    pub async fn get_block_header(&self, hash: Digest) -> Result<BlockHeader> {
        self.optimize().await?;

        self.call(move |db| {
            db.query_row(
                r"
            SELECT
                previous_hash,
                previous_state_root,
                transactions_root,
                network,
                round,
                height,
                coinbase_target,
                proof_target,
                timestamp,
                challenge,
                response,
                compute_key_public_key_signature,
                compute_key_public_randomness_signature,
                compute_key_secret_key_program
            FROM blocks WHERE hash = ?",
                [&hash[..]],
                |row| {
                    Ok(BlockHeader {
                        block_hash: hash.clone(),
                        previous_hash: row.get(0)?,
                        previous_state_root: row.get(1)?,
                        transactions_root: row.get(2)?,
                        metadata: Metadata {
                            network: row.get(3)?,
                            round: row.get(4)?,
                            height: row.get(5)?,
                            coinbase_target: row.get(6)?,
                            proof_target: row.get(7)?,
                            timestamp: row.get(8)?,
                        },
                        signature: Signature {
                            challenge: row.get(9)?,
                            response: row.get(10)?,
                            compute_key: ComputeKey {
                                public_key_signature: row.get(11)?,
                                public_randomness_signature: row.get(12)?,
                                prf_secret_key: row.get(13)?,
                            },
                        },
                    })
                },
            )
            .map_err(Into::into)
        })
        .await
    }

    /// Gets a block status for a given hash
    pub async fn get_block_state(&self, hash: Digest) -> Result<BlockStatus> {
        self.optimize().await?;

        let output: Option<Option<usize>> = self
            .call(move |db| {
                db.query_row(
                    r"SELECT canon_height FROM blocks WHERE hash = ?",
                    [hash],
                    |row| row.get(0),
                )
                .optional()
            })
            .await?;

        Ok(match output {
            None => BlockStatus::Unknown,
            Some(None) => BlockStatus::Uncommitted,
            Some(Some(n)) => BlockStatus::Committed(n),
        })
    }

    /// Bulk operation of `Storage::get_block_state`, gets many block statuses for many hashes.
    pub async fn get_block_states(
        &self,
        hashes: impl IntoIterator<Item = Digest>,
    ) -> Result<Vec<BlockStatus>> {
        self.optimize().await?;

        // intentional N+1 query since rusqlite doesn't support WHERE ... IN here and it doesn't matter at the moment
        let hashes = hashes.into_iter();
        let mut out = Vec::with_capacity(hashes.size_hint().0);
        for hash in hashes {
            let state = self.get_block_state(hash).await?;
            out.push(state);
        }
        Ok(out)
    }
}

/*



/// Finds a fork path from any applicable canon node within `oldest_fork_threshold` to `hash`.
async fn get_fork_path(&self, hash: &Digest, oldest_fork_threshold: usize) -> Result<ForkDescription>;

/// Commits a block into canon.
async fn commit_block(&self, hash: &Digest, digest: Digest) -> Result<BlockStatus>;

/// Attempts to recommit a block and its longest descendent chains blocks into canon, until there are no more ledger digests.
async fn recommit_blockchain(&self, hash: &Digest) -> Result<()>;

/// Attempts to recommit a block into canon if it has a ledger digest.
async fn recommit_block(&self, hash: &Digest) -> Result<BlockStatus>;

/// Decommits a block and all descendent blocks, returning them in ascending order
async fn decommit_blocks(&self, hash: &Digest) -> Result<Vec<SerialBlock>>;

/// Gets the current canon state of storage
async fn canon(&self) -> Result<CanonData>;

/// Gets the longest, committed or uncommitted, chain of blocks originating from `block_hash`, including `block_hash`.
async fn longest_child_path(&self, block_hash: &Digest) -> Result<Vec<Digest>>;

/// Gets a tree structure representing all the descendents of [`block_hash`]
async fn get_block_digest_tree(&self, block_hash: &Digest) -> Result<DigestTree>;

/// Gets the immediate children of `block_hash`.
async fn get_block_children(&self, block_hash: &Digest) -> Result<Vec<Digest>>;

/// Gets a series of hashes used for relaying current block sync state.
async fn get_block_locator_hashes(
    &self,
    points_of_interest: Vec<Digest>,
    oldest_fork_threshold: usize,
) -> Result<Vec<Digest>>;

/// scans uncommitted blocks with a known path to the canon chain for forks
async fn scan_forks(&self, scan_depth: u32) -> Result<Vec<(Digest, Digest)>>;

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

// miner convenience record management functions

/// Gets a list of stored record commitments subject to `limit`.
async fn get_record_commitments(&self, limit: Option<usize>) -> Result<Vec<Digest>>;

/// Gets a record blob given a commitment.
async fn get_record(&self, commitment: Digest) -> Result<Option<SerialRecord>>;

/// Stores a series of new record blobs and their commitments.
async fn store_records(&self, records: &[SerialRecord]) -> Result<()>;

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

/// Stores or updates a collection of [`Peer`]s
async fn store_peers(&self, peers: Vec<Peer>) -> Result<()>;

/// Looks up a series of [`Peer`]s based on socket address.
async fn lookup_peers(&self, addresses: Vec<SocketAddr>) -> Result<Vec<Option<Peer>>>;

/// Looks up all known [`Peer`]s.
async fn fetch_peers(&self) -> Result<Vec<Peer>>;

/// Performs low-level storage validation; it's mostly intended for test purposes, as there is a lower level `KeyValueStorage` interface available outside of them.
async fn validate(&self, limit: Option<u32>, fix_mode: FixMode) -> Vec<ValidatorError>;

/// Stores the given key+value pair in the given column.
#[cfg(feature = "test")]
async fn store_item(&self, col: KeyValueColumn, key: Vec<u8>, value: Vec<u8>) -> Result<()>;

/// Removes the given key and its corresponding value from the given column.
#[cfg(feature = "test")]
async fn delete_item(&self, col: KeyValueColumn, key: Vec<u8>) -> Result<()>;

#[cfg(feature = "test")]
async fn reset(&self) -> Result<()>;

/// Removes non-canon blocks and transactions from the storage.
async fn trim(&self) -> Result<()>;

*/
