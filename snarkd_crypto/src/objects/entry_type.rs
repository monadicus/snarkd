use super::PlaintextType;

#[derive(Clone, PartialEq, Eq)]
pub enum EntryType {
    Constant(PlaintextType),
    Public(PlaintextType),
    Private(PlaintextType),
}
