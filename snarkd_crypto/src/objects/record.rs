use super::{Field, Group};

pub struct Record {
    pub owner: Field,
    pub gates: Field,
    pub data: IndexMap<(Field, u8), Vec<u8>>,
    pub nonce: Group,
}
