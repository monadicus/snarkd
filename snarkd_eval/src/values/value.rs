use crate::Todo;

use super::*;

#[derive(Clone, Debug)]
pub enum ConstrainedValue<Todo, G: GroupType<Todo>> {
    Address(Address),
    Boolean(Todo),
    Field(FieldType<Todo>),
    Group(G),
    Integer(Integer),
    Scalar(Scalar),
    String(String),
    Struct(Structure<Todo, G>),
    Record(Record<Todo, G>),
}
