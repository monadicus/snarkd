use super::{Field, Origin};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Input {
    Constant(Field, Option<Vec<u8>>),
    Public(Field, Option<Vec<u8>>),
    Private(Field, Option<Vec<u8>>),
    Record(Field, Field, Origin),
    ExternalRecord(Field),
}
