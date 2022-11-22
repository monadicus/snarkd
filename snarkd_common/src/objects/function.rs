use super::{Identifier, Instruction, ValueEntry};
use indexmap::IndexSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: Identifier,
    pub inputs: IndexSet<ValueEntry>,
    pub instructions: Vec<Instruction>,
    pub outputs: IndexSet<ValueEntry>,
}
