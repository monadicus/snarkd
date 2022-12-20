use snarkd_common::objects::{Block, BlockHeader};

use crate::proto;
use anyhow::{Result, Context};

impl TryInto<Block> for proto::Block {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<Block> {
        let header = self.header.context("missing header")?;
        Ok(Block {
            header: BlockHeader {
                height: header.canon_height,
                block_hash: header.hash.context("missing hash")?,
                previous_hash: header.previous_hash.context("missing previous_hash")?,
                nonce: header.nonce,
                network: header.network.try_into()?,
                coinbase_target: header.coinbase_target,
                timestamp: header.timestamp as i64,
            },
            transactions: vec![]
            // transactions: block.transactions.into_iter().map(|transaction| snarkd_network::proto::Transaction {
                
            // }),
        })
    }
}

impl From<Block> for proto::Block {
    fn from(block: Block) -> proto::Block {
        proto::Block {
            header: Some(proto::BlockHeader {
                canon_height: block.header.height,
                hash: Some(block.header.block_hash),
                previous_hash: Some(block.header.previous_hash),
                nonce: block.header.nonce,
                network: block.header.network as u32,
                coinbase_target: block.header.coinbase_target,
                timestamp: block.header.timestamp as u64,
            }),
            transactions: vec![]
            // transactions: block.transactions.into_iter().map(|transaction| snarkd_network::proto::Transaction {
                
            // }),
        }
    }
}