use super::{EntryType, Identifier};
use indexmap::IndexMap;

#[derive(Clone, PartialEq, Eq)]
pub struct RecordType {
    pub name: Identifier,
    pub owner_is_public: bool,
    pub gates_is_public: bool,
    pub entries: IndexMap<Identifier, EntryType>,
}
