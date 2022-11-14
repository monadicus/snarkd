use sha2::{Digest, Sha256};

use crate::Digest32;

use super::{compute_key::ComputeKey, Field, Scalar, Transaction};

type BlockHash = Digest32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub challenge: Scalar,
    pub response: Scalar,
    pub compute_key: ComputeKey,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockHeader {
    pub block_hash: BlockHash,
    pub previous_hash: BlockHash,
    pub previous_state_root: Field,
    pub transactions_root: Field,
    pub metadata: Metadata,
    pub signature: Signature,
}

impl BlockHeader {
    pub fn hash(&self) -> BlockHash {
        let mut sha = Sha256::default();
        sha.update(&self.previous_hash[..]);
        sha.update(&self.previous_state_root[..]);
        sha.update(&self.transactions_root[..]);
        sha.update(&self.metadata.network.to_le_bytes()[..]);
        sha.update(&self.metadata.round.to_le_bytes()[..]);
        sha.update(&self.metadata.height.to_le_bytes()[..]);
        sha.update(&self.metadata.coinbase_target.to_le_bytes()[..]);
        sha.update(&self.metadata.proof_target.to_le_bytes()[..]);
        sha.update(&self.metadata.timestamp.to_le_bytes()[..]);
        let output = sha.finalize();
        let sha = sha2::Sha256::digest(output);
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&sha[..]);
        BlockHash::from(hash)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    pub network: u16,
    pub round: u64,
    pub height: u32,
    pub coinbase_target: u64,
    pub proof_target: u64,
    pub timestamp: i64,
}
