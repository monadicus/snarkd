use std::fmt;

use super::ir;

use snarkd_errors::{Error, IRError, Result};

use crate::operand::{Operand, Scalar};
pub use ir::opcode::CommitMethod;

impl fmt::Display for CommitMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommitMethod::CommitBhp256 => write!(f, "bhp256"),
            CommitMethod::CommitBhp512 => write!(f, "bhp512"),
            CommitMethod::CommitBhp768 => write!(f, "bhp768"),
            CommitMethod::CommitBhp1024 => write!(f, "bhp1024"),
            CommitMethod::CommitPed64 => write!(f, "ped64"),
            CommitMethod::CommitPed128 => write!(f, "ped128"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommitData {
    pub method: CommitMethod,
    pub chunk: Operand,
    pub randomness: Scalar,
    pub dest: u32,
}

impl TryFrom<ir::opcode::CommitData> for CommitData {
    type Error = Error;

    fn try_from(value: ir::opcode::CommitData) -> Result<Self> {
        Ok(Self {
            method: CommitMethod::from_i32(value.method)
                .ok_or_else(IRError::invalid_commit_method)?,
            chunk: value
                .chunk
                .ok_or_else(|| IRError::unset("HashData chunk"))?
                .try_into()?,
            randomness: value
                .randomness
                .ok_or_else(|| IRError::unset("HashData randomness"))?,
            dest: value.dest,
        })
    }
}

impl From<CommitData> for ir::opcode::CommitData {
    fn from(value: CommitData) -> Self {
        Self {
            method: value.method as i32,
            chunk: Some(value.chunk.into()),
            randomness: Some(value.randomness),
            dest: value.dest,
        }
    }
}

impl fmt::Display for CommitData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.method, self.chunk, self.randomness, self.dest
        )
    }
}
