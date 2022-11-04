use snarkd_common::Digest32;

use super::{BlockHeader, Signature, Transaction};

type BlockHash = Digest32;

#[derive(Clone, PartialEq, Eq)]
pub struct Block {
    pub block_hash: BlockHash,
    pub previous_hash: BlockHash,
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub signature: Signature,
}
