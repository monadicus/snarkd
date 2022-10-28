use super::{Identifier, PlaintextType};
use indexmap::IndexMap;

#[derive(Clone, PartialEq, Eq)]
pub struct Interface {
    pub name: Identifier,
    pub members: IndexMap<Identifier, PlaintextType>,
}
