use crate::Digest32;

use super::{Field, Group, Identifier, Input, Output, ProgramID};

type TransitionID = Digest32;

type Proof = Digest32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transition {
    pub id: TransitionID,
    pub program_id: ProgramID,
    pub function_name: Identifier,
    pub inputs: Input,
    pub outputs: Output,
    pub finalize: Option<Vec<u8>>,
    pub proof: Proof,
    pub tpk: Group,
    pub tcm: Field,
    pub fee: i64,
}
