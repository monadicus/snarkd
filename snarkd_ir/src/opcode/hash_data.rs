use std::fmt;

use super::ir;
use crate::Operand;

use snarkd_errors::{Error, IRError, Result};

pub use ir::opcode::HashMethod;

impl fmt::Display for HashMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashMethod::HashBhp256 => write!(f, "bhp256"),
            HashMethod::HashBhp512 => write!(f, "bhp512"),
            HashMethod::HashBhp768 => write!(f, "bhp768"),
            HashMethod::HashBhp1024 => write!(f, "bhp1024"),
            HashMethod::HashPed64 => write!(f, "ped64"),
            HashMethod::HashPed128 => write!(f, "ped128"),
            HashMethod::HashPsd2 => write!(f, "psd2"),
            HashMethod::HashPsd4 => write!(f, "psd4"),
            HashMethod::HashPsd8 => write!(f, "psd8"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HashData {
    pub method: HashMethod,
    pub chunk: Operand,
    pub dest: u32,
}

impl TryFrom<ir::opcode::HashData> for HashData {
    type Error = Error;

    fn try_from(value: ir::opcode::HashData) -> Result<Self> {
        Ok(Self {
            method: HashMethod::from_i32(value.method).ok_or_else(IRError::invalid_hash_method)?,
            chunk: value
                .chunk
                .ok_or_else(|| IRError::unset("HashData chunk"))?
                .try_into()?,
            dest: value.dest,
        })
    }
}

impl From<HashData> for ir::opcode::HashData {
    fn from(value: HashData) -> Self {
        Self {
            method: value.method as i32,
            chunk: Some(value.chunk.into()),
            dest: value.dest,
        }
    }
}

impl fmt::Display for HashData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.method, self.chunk, self.dest)
    }
}
