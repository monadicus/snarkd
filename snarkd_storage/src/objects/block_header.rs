use snarkd_common::Digest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerialBlockHeader {
    /// Hash of the previous block - 32 bytes
    pub previous_block_hash: Digest,

    /// Merkle root representing the transactions in the block - 32 bytes
    pub merkle_root_hash: Digest,

    /// Merkle root of the transactions in the block using a Pedersen hash - 32 bytes
    pub pedersen_merkle_root_hash: Digest,

    /// Proof of Succinct Work
    pub proof: Vec<u8>,

    /// The block timestamp is a Unix epoch time (UTC) when the miner
    /// started hashing the header (according to the miner). - 8 bytes
    pub time: i64,

    /// Proof of work algorithm difficulty target for this block - 8 bytes
    pub difficulty_target: u64,

    /// Nonce for solving the PoW puzzle - 4 bytes
    pub nonce: u32,
}

// impl SerialBlockHeader {
//     pub fn hash(&self) -> Digest {
//         let mut out = vec![];
//         self.write_le(&mut out).expect("failed to serialize block header");
//         double_sha256(&out)[..].into()
//     }

//     pub fn to_difficulty_hash(&self) -> u64 {
//         sha256d_to_u64(&self.proof.0[..])
//     }
// }
