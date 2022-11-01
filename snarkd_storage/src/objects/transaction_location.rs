use snarkd_common::Digest;

/// Represents address of certain transaction within block
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TransactionLocation {
    /// Transaction index within the block
    pub index: u32,
    /// Block hash
    pub block_hash: Digest,
}
