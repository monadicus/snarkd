use anyhow::{anyhow, bail, Context, Result};
use log::warn;
use rusqlite::{params, Connection, OptionalExtension, ToSql};
use snarkd_common::{
    objects::{
        Block, BlockHeader, DeployTransaction, Deployment, ExecuteTransaction, Execution,
        Identifier, ProgramID, Transaction, Transition,
    },
    Digest,
};

use crate::{db::InnerDatabase, Database};

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

fn write_deployment(connection: &Connection, deployment: &Deployment) -> Result<i32> {
    let mut deployment_query = connection.prepare_cached(
        r"
        INSERT INTO deployments (
            edition,
            program,
            verifying_key_id,
            verifying_key,
            certificate
        )
        VALUES (
            ?,
            ?,
            ?,
            ?,
            ?
        )
    ",
    )?;

    deployment_query.execute::<&[&dyn ToSql]>(&[
        &(deployment.edition as i32),
        &deployment.program,
        &deployment.verifying_key_id,
        &deployment.verifying_key,
        &deployment.certificate,
    ])?;
    Ok(connection.last_insert_rowid() as i32)
}

fn read_transitions(
    connection: &Connection,
    transaction_id: i64,
) -> Result<Vec<(i32, Transition)>> {
    let mut transition_query = connection.prepare_cached(
        r"
        SELECT (
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
            fee,
            deployment_id
        ) FROM transitions
        WHERE transaction_id = ?
        ORDER BY transaction_order ASC
    ",
    )?;
    let mut rows = transition_query.query(&[&transaction_id])?;
    let mut out = vec![];
    while let Some(row) = rows.next()? {
        out.push((
            row.get(0)?,
            Transition {
                id: row.get(1)?,
                program_id: ProgramID {
                    name: row.get(2)?,
                    network: row.get(3)?,
                },
                function_name: row.get(4)?,
                inputs: row.get(5)?,
                outputs: row.get(6)?,
                finalize: row.get(7)?,
                proof: row.get(8)?,
                tpk: row.get(9)?,
                tcm: row.get(10)?,
                fee: row.get(11)?,
            },
        ));
    }

    Ok(out)
}

fn write_transition(
    connection: &Connection,
    transaction_id: i64,
    transaction_order: i32,
    transition: &Transition,
    deployment_id: Option<i32>,
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
            fee,
            deployment_id
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
        &transition.inputs,
        &transition.outputs,
        &transition.finalize,
        &transition.proof,
        &transition.tpk,
        &transition.tcm,
        &transition.fee,
        &deployment_id,
    ])?;
    Ok(())
}

impl Database {
    pub async fn insert_block(&self, block: Block) -> Result<()> {
        self.call(|x| x.insert_block(block)).await
    }

    pub async fn get_block(&self, digest: Digest) -> Result<Option<Block>> {
        self.call(move |x| x.get_block(&digest)).await
    }

    pub async fn get_block_state(&self, digest: Digest) -> Result<BlockStatus> {
        self.call(move |x| x.get_block_state(&digest)).await
    }

    pub async fn get_block_header(&self, hash: Digest) -> Result<Option<BlockHeader>> {
        self.call(move |x| x.get_block_header(&hash)).await
    }

    pub async fn get_block_hash(&self, block_num: u32) -> Result<Option<Digest>> {
        self.call(move |x| x.get_block_hash(block_num)).await
    }

    pub async fn delete_block(&self, hash: Digest) -> Result<()> {
        self.call(move |x| x.delete_block(&hash)).await
    }
}

impl InnerDatabase {
    /// Inserts a block into storage, not committing it.
    pub fn insert_block(&mut self, block: Block) -> Result<()> {
        self.optimize()?;
        let hash = block.header.hash();

        match self.get_block_state(&hash)? {
            BlockStatus::Unknown => (),
            BlockStatus::Committed(_) | BlockStatus::Uncommitted => {
                // metrics::increment_counter!(snarkos_metrics::blocks::DUPLICATES);
                return Err(anyhow!("duplicate block insertion"));
            }
        }

        let transaction = self.connection.transaction()?;
        {
            let mut block_query = transaction.prepare_cached(
                r"
            INSERT INTO blocks (
                hash,
                previous_block_id,
                previous_block_hash,
                nonce,
                network,
                height,
                coinbase_target,
                timestamp
            )
            VALUES (
                ?,
                (SELECT id from blocks where hash = ?),
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
                &block.header.nonce,
                // &block.header.previous_state_root,
                // &block.header.transactions_root,
                &block.header.network,
                // &block.header.metadata.round,
                &block.header.height,
                &block.header.coinbase_target,
                // &block.header.metadata.proof_target,
                &block.header.timestamp,
                // &block.header.signature.challenge,
                // &block.header.signature.response,
                // &block.header.signature.compute_key.public_key_signature,
                // &block
                //     .header
                //     .signature
                //     .compute_key
                //     .public_randomness_signature,
                // &block.header.signature.compute_key.prf_secret_key,
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
                    execute_edition,
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
                        let deployment_id = write_deployment(&transaction, &deployment)?;

                        transaction_query.execute(params![&id, &None::<i32>, &"deploy",])?;
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
                        write_transition(
                            &transaction,
                            transaction_id,
                            0,
                            &transition,
                            Some(deployment_id),
                        )?;
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
                        transaction_query.execute(params![
                            &id,
                            &Some(edition as i32),
                            &"execute",
                        ])?;
                        let transaction_id = transaction.query_row(
                            r"SELECT id FROM transactions WHERE transaction_id = ?",
                            [&id],
                            |row| row.get::<usize, i64>(0),
                        )?;
                        if let Some(transition) = transition {
                            write_transition(&transaction, transaction_id, -1, &transition, None)?;
                        }
                        for (i, transition) in transitions.into_iter().enumerate() {
                            write_transition(
                                &transaction,
                                transaction_id,
                                i as i32,
                                &transition,
                                None,
                            )?;
                        }
                        transaction_block_query.execute(params![&id, block_id as usize, i])?;
                    }
                }
            }
        }
        transaction.commit()?;
        Ok(())
    }

    /// Gets a block header and transaction blob for a given hash.
    pub fn get_block(&mut self, hash: &Digest) -> Result<Option<Block>> {
        let Some((block_id, header)) = self.get_block_header_and_id(hash)? else {
            return Ok(None);
        };
        let mut transaction_query = self.connection.prepare_cached(
            r"
            SELECT (
                id,
                transaction_id,
                execute_edition
                transaction_type,
                edition,
                program,
                verifying_key_id,
                verifying_key,
                certificate
            )
            FROM transactions
            INNER JOIN transaction_blocks ON transaction_blocks.transaction_id = transactions.id
            LEFT JOIN deployments ON transactions.deployment_id IS NOT NULL AND deployments.id = transactions.deployment_id
            WHERE transaction_blocks.block_id = ?
            ORDER BY transaction_blocks.block_order ASC
        ",
        )?;
        let mut rows = transaction_query.query(&[&block_id])?;
        let mut out = vec![];
        while let Some(row) = rows.next()? {
            let transaction_db_id: i64 = row.get(0)?;
            let transaction_id: Digest = row.get(1)?;
            match &*row.get::<_, String>(3)? {
                "deploy" => {
                    let transitions = read_transitions(&self.connection, transaction_db_id)?;
                    if transitions.len() != 1 {
                        bail!("invalid number of transitions for transaction {transaction_db_id}");
                    }
                    out.push(Transaction::Deploy(DeployTransaction {
                        id: transaction_id,
                        deployment: Deployment {
                            edition: row
                                .get::<_, Option<i32>>(4)?
                                .context("missing edition for deploy transaction")?
                                as u16,
                            program: row
                                .get::<_, Option<Vec<u8>>>(5)?
                                .context("missing program for deploy transaction")?,
                            verifying_key_id: row
                                .get::<_, Option<Identifier>>(6)?
                                .context("missing verifying_key_id for deploy transaction")?,
                            verifying_key: row
                                .get::<_, Option<Digest>>(7)?
                                .context("missing verifying_key for deploy transaction")?,
                            certificate: row
                                .get::<_, Option<Digest>>(8)?
                                .context("missing certificate for deploy transaction")?,
                        },
                        transition: transitions.into_iter().next().unwrap().1,
                    }));
                }
                "execute" => {
                    let execute_edition: u16 = row
                        .get::<_, Option<i32>>(2)?
                        .context("missing execute_edition for execute transaction")?
                        as u16;
                    let mut transitions = read_transitions(&self.connection, transaction_db_id)?;
                    let extra_transition = if !transitions.is_empty() && transitions[0].0 == -1 {
                        Some(transitions.remove(0).1)
                    } else {
                        None
                    };
                    out.push(Transaction::Execute(ExecuteTransaction {
                        id: transaction_id,
                        execution: Execution {
                            edition: execute_edition,
                            transitions: transitions.into_iter().map(|x| x.1).collect(),
                        },
                        transition: extra_transition,
                    }));
                }
                type_ => {
                    warn!("invalid transaction type in block: {type_}");
                }
            }
        }
        Ok(Some(Block {
            header,
            transactions: out,
        }))
    }

    /// Deletes a block from storage, including any associated data. Must not be called on a committed block.
    pub fn delete_block(&mut self, hash: &Digest) -> Result<()> {
        self.optimize()?;
        let transaction = self.connection.transaction()?;

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
    }

    /// Gets a hash for a canon block number, if it exists
    pub fn get_block_hash(&mut self, block_num: u32) -> Result<Option<Digest>> {
        self.optimize()?;

        Ok(self
            .connection
            .query_row::<Vec<u8>, _, _>(
                r"SELECT hash FROM blocks WHERE canon_height = ?",
                [block_num],
                |row| row.get(0),
            )
            .optional()?
            .map(|x| Digest::from(&x[..])))
    }

    /// Gets a block header for a given hash
    pub fn get_block_header(&mut self, hash: &Digest) -> Result<Option<BlockHeader>> {
        self.get_block_header_and_id(hash).map(|x| x.map(|x| x.1))
    }

    fn get_block_header_and_id(&mut self, hash: &Digest) -> Result<Option<(i32, BlockHeader)>> {
        self.optimize()?;

        self.connection
            .query_row(
                r"
        SELECT
            previous_hash,
            nonce,
            network,
            height,
            coinbase_target,
            timestamp,
            id
        FROM blocks WHERE hash = ?",
                [&hash[..]],
                |row| {
                    Ok((
                        row.get(6)?,
                        BlockHeader {
                            block_hash: hash.clone(),
                            previous_hash: row.get(0)?,
                            // previous_state_root: row.get(1)?,
                            // transactions_root: row.get(2)?,
                            nonce: row.get(1)?,
                            network: row.get(2)?,
                            // round: row.get(4)?,
                            height: row.get(3)?,
                            coinbase_target: row.get(4)?,
                            // proof_target: row.get(7)?,
                            timestamp: row.get(5)?,
                            // signature: Signature {
                            //     challenge: row.get(9)?,
                            //     response: row.get(10)?,
                            //     compute_key: ComputeKey {
                            //         public_key_signature: row.get(11)?,
                            //         public_randomness_signature: row.get(12)?,
                            //         prf_secret_key: row.get(13)?,
                            //     },
                            // },
                        },
                    ))
                },
            )
            .optional()
            .map_err(Into::into)
    }

    /// Gets a block status for a given hash
    pub fn get_block_state(&mut self, hash: &Digest) -> Result<BlockStatus> {
        self.optimize()?;

        let output: Option<Option<usize>> = self
            .connection
            .query_row(
                r"SELECT canon_height FROM blocks WHERE hash = ?",
                [hash],
                |row| row.get(0),
            )
            .optional()?;

        Ok(match output {
            None => BlockStatus::Unknown,
            Some(None) => BlockStatus::Uncommitted,
            Some(Some(n)) => BlockStatus::Committed(n),
        })
    }

    /// Bulk operation of `Storage::get_block_state`, gets many block statuses for many hashes.
    pub async fn get_block_states(
        &mut self,
        hashes: impl IntoIterator<Item = Digest>,
    ) -> Result<Vec<BlockStatus>> {
        self.optimize()?;

        // intentional N+1 query since rusqlite doesn't support WHERE ... IN here and it doesn't matter at the moment
        let hashes = hashes.into_iter();
        let mut out = Vec::with_capacity(hashes.size_hint().0);
        for hash in hashes {
            let state = self.get_block_state(&hash)?;
            out.push(state);
        }
        Ok(out)
    }
}
