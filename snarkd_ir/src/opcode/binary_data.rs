use std::fmt;

use super::ir;
use crate::Operand;

use snarkd_errors::{Error, IRError, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryData {
    pub lhs: Operand,
    pub rhs: Operand,
    pub dest: u32,
}

impl TryFrom<ir::BinaryData> for BinaryData {
    type Error = Error;

    fn try_from(value: ir::BinaryData) -> Result<Self> {
        Ok(Self {
            lhs: value
                .lhs
                .ok_or_else(|| IRError::missing_operand("binary operation lhs"))?
                .try_into()?,
            rhs: value
                .rhs
                .ok_or_else(|| IRError::missing_operand("binary operation rhs"))?
                .try_into()?,
            dest: value.dest,
        })
    }
}

impl From<BinaryData> for ir::BinaryData {
    fn from(value: BinaryData) -> Self {
        Self {
            lhs: Some(value.lhs.into()),
            rhs: Some(value.rhs.into()),
            dest: value.dest,
        }
    }
}

impl fmt::Display for BinaryData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.lhs, self.rhs, self.dest)
    }
}
