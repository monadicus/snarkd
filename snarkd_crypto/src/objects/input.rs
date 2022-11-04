use super::{Field, Origin};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Input {
    Constant(Field, Option<Vec<u8>>),
    //@jds: should this be a blob? is it max-length at all?
    Public(Field, Option<Vec<u8>>),
    //@jds: should this be a blob? is it max-length at all?
    Private(Field, Option<Vec<u8>>),
    //@jds: should this be a blob? is it max-length at all?
    Record(Field, Field, Origin),
    ExternalRecord(Field),
}
