use super::Field;

type StateRoot = [u8; 32];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Origin {
    Commitment(Field),
    StateRoot(StateRoot),
}
