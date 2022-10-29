use super::{Field, Metadata};

#[derive(Clone, PartialEq, Eq)]
pub struct BlockHeader {
    pub previous_state_root: Field,
    pub transactions_root: Field,
    pub metadata: Metadata,
}
