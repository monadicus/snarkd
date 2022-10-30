use snarkd_common::Digest32;

use super::{Field, Group, Identifier, Input, Output, ProgramID, Value};

type TransitionID = Digest32;

type Proof = Digest32;

#[derive(Clone, PartialEq, Eq)]
pub struct Transition {
    pub id: TransitionID,
    pub program_id: ProgramID,
    pub function_name: Identifier,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub finalize: Option<Vec<Value>>,
    pub proof: Proof,
    pub tpk: Group,
    pub tcm: Field,
    pub fee: i64,
}
