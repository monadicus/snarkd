use super::{Identifier, PlaintextType};
use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interface {
    pub name: Identifier,
    pub members: IndexMap<Identifier, PlaintextType>,
}
