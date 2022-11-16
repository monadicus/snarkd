use super::{Field, Group, Identifier};
use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub owner: Field,
    pub gates: Field,
    pub data: IndexMap<Identifier, Vec<u8>>,
    pub nonce: Group,
}
