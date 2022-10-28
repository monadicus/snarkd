#[derive(Clone, PartialEq, Eq)]
pub struct Metadata {
    pub network: u16,
    pub round: u64,
    pub height: u32,
    pub coinbase_target: u64,
    pub proof_target: u64,
    pub timestamp: i64,
}
