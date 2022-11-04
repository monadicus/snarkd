use std::fmt;

use super::ir;
use crate::Operand;

use snarkd_errors::{Error, IRError, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssertData {
    pub lhs: Operand,
    pub rhs: Operand,
}

impl TryFrom<ir::AssertData> for AssertData {
    type Error = Error;

    fn try_from(value: ir::AssertData) -> Result<Self> {
        Ok(Self {
            lhs: value
                .lhs
                .ok_or_else(|| IRError::missing_operand("assert operation lhs"))?
                .try_into()?,
            rhs: value
                .rhs
                .ok_or_else(|| IRError::missing_operand("assert operation rhs"))?
                .try_into()?,
        })
    }
}

impl From<AssertData> for ir::AssertData {
    fn from(value: AssertData) -> Self {
        Self {
            lhs: Some(value.lhs.into()),
            rhs: Some(value.rhs.into()),
        }
    }
}

impl fmt::Display for AssertData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.lhs, self.rhs)
    }
}
