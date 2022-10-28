use crate::Todo;

#[derive(Clone, Debug)]
pub struct Address {
    pub address: Todo,
    pub bytes: Vec<u8>,
}
