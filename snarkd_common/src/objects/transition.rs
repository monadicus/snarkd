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

impl Transition {
    pub fn size(&self) -> usize {
        self.id.len() + self.program_id.name.field.len() + self.program_id.network.field.len() + self.function_name.field.len() + self.inputs.len() + self.outputs.len() + self.finalize.as_ref().map(Vec::len).unwrap_or_default() + self.proof.len() + self.tpk.len() + self.tcm.len() + std::mem::size_of::<i64>()
    }
}