use super::{Field, Record};

#[derive(Clone, PartialEq, Eq)]
pub enum Output {
    Constant(Field, Option<Vec<u8>>),
    Public(Field, Option<Vec<u8>>),
    Private(Field, Option<Vec<u8>>),
    Record(Field, Field, Option<Box<Record>>),
    ExternalRecord(Field),
}
