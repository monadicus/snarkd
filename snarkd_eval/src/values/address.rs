use super::*;

#[derive(Clone, Debug)]
pub struct ConstrainedAddress<G: Group> {
    pub address: G,
    pub bytes: Vec<u8>,
}
