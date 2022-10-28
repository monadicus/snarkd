use super::{FinalizeType, Identifier};

#[derive(Clone, PartialEq, Eq)]
pub struct MapObject {
    pub name: Identifier,
    pub finalize_type: FinalizeType,
}
