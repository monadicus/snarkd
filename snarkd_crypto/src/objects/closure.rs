use super::{Entry, Identifier, Instruction};
use indexmap::IndexSet;

#[derive(Clone, PartialEq, Eq)]
pub struct Closure {
    pub name: Identifier,
    pub inputs: IndexSet<Entry>,
    pub instructions: Vec<Instruction>,
    pub outputs: IndexSet<Entry>,
}
