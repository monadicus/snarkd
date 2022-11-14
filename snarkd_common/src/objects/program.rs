use super::{Closure, Function, Identifier, Interface, Mapping, ProgramID, RecordType};
use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramDefinition {
    Mapping,
    Interface,
    Record,
    Closure,
    Function,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub id: ProgramID,
    pub imports: IndexMap<ProgramID, ProgramID>,
    pub identifiers: IndexMap<Identifier, ProgramDefinition>,
    pub mappings: IndexMap<Identifier, Mapping>,
    pub interfaces: IndexMap<Identifier, Interface>,
    pub records: IndexMap<Identifier, RecordType>,
    pub closures: IndexMap<Identifier, Closure>,
    pub functions: IndexMap<Identifier, Function>,
}
