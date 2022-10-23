use super::{Identifier, MapObject};

#[derive(Clone, PartialEq, Eq)]
pub struct Mapping {
    pub name: Identifier,
    pub key: MapObject,
    pub value: MapObject,
}
