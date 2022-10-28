use super::{Identifier, LiteralType};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum PlaintextType {
    Literal(LiteralType),
    Interface(Identifier),
}
