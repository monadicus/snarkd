use sha2::{Digest, Sha256};

use crate::Digest32;

use super::{Transaction};

type BlockHash = Digest32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockHeader {
    pub block_hash: BlockHash,
    pub previous_hash: BlockHash,
    // pub previous_state_root: Field,
    // pub transactions_root: Field,
    pub nonce: u32,
    pub network: u16,
    // pub round: u64,
    pub height: u32,
    pub coinbase_target: u64,
    // pub proof_target: u64,
    pub timestamp: i64,
    // pub signature: Signature,
}

impl BlockHeader {
    pub fn hash(&self) -> BlockHash {
        let mut sha = Sha256::default();
        sha.update(&self.previous_hash[..]);
        sha.update(&self.nonce.to_be_bytes()[..]);
        // sha.update(&self.previous_state_root[..]);
        // sha.update(&self.transactions_root[..]);
        sha.update(&self.network.to_le_bytes()[..]);
        // sha.update(&self.metadata.round.to_le_bytes()[..]);
        sha.update(&self.height.to_le_bytes()[..]);
        sha.update(&self.coinbase_target.to_le_bytes()[..]);
        // sha.update(&self.metadata.proof_target.to_le_bytes()[..]);
        sha.update(&self.timestamp.to_le_bytes()[..]);
        let output = sha.finalize();
        let sha = sha2::Sha256::digest(output);
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&sha[..]);
        BlockHash::from(hash)
    }

    pub const fn size() -> usize {
        32 * 2 + std::mem::size_of::<Block>()
    }
}
