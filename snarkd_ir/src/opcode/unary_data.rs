use std::fmt;

use super::ir;
use crate::Operand;

use snarkd_errors::{Error, IRError, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryData {
    pub operand: Operand,
    pub dest: u32,
}

impl TryFrom<ir::UnaryData> for UnaryData {
    type Error = Error;

    fn try_from(value: ir::UnaryData) -> Result<Self> {
        Ok(Self {
            operand: value
                .operand
                .ok_or_else(|| IRError::missing_operand("unary operation"))?
                .try_into()?,
            dest: value.dest,
        })
    }
}

impl From<UnaryData> for ir::UnaryData {
    fn from(value: UnaryData) -> Self {
        Self {
            operand: Some(value.operand.into()),
            dest: value.dest,
        }
    }
}

impl fmt::Display for UnaryData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.operand, self.dest)
    }
}
