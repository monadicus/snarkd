#[derive(Clone, PartialEq, Eq, Hash)]
pub enum LiteralType {
    Address,
    Boolean,
    Field,
    Group,
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    Scalar,
    String,
}
