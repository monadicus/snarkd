use std::fmt;

use super::ir;
use crate::Operand;

use snarkd_errors::{Error, IRError, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TernaryData {
    pub cond: Operand,
    pub lhs: Operand,
    pub rhs: Operand,
    pub dest: u32,
}

impl TryFrom<ir::TernaryData> for TernaryData {
    type Error = Error;

    fn try_from(value: ir::TernaryData) -> Result<Self> {
        Ok(Self {
            cond: value
                .cond
                .ok_or_else(|| IRError::missing_operand("ternary operation condition"))?
                .try_into()?,
            lhs: value
                .lhs
                .ok_or_else(|| IRError::missing_operand("ternary operation lhs"))?
                .try_into()?,
            rhs: value
                .rhs
                .ok_or_else(|| IRError::missing_operand("ternary operation rhs"))?
                .try_into()?,
            dest: value.dest,
        })
    }
}

impl From<TernaryData> for ir::TernaryData {
    fn from(value: TernaryData) -> Self {
        Self {
            cond: Some(value.cond.into()),
            lhs: Some(value.lhs.into()),
            rhs: Some(value.rhs.into()),
            dest: value.dest,
        }
    }
}

impl fmt::Display for TernaryData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.cond, self.lhs, self.rhs, self.dest
        )
    }
}
