use std::fmt;

use super::ir;
use crate::Operand;
use anyhow::{anyhow, Error, Result};

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
                .ok_or_else(|| anyhow!("missing operand for unary operation"))?
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
                .ok_or_else(|| anyhow!("missing lhs for binary operation"))?
                .try_into()?,
            rhs: value
                .rhs
                .ok_or_else(|| anyhow!("missing rhs for binary operation"))?
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
                .ok_or_else(|| anyhow!("missing cond for ternary operation"))?
                .try_into()?,
            lhs: value
                .lhs
                .ok_or_else(|| anyhow!("missing lhs for ternary operation"))?
                .try_into()?,
            rhs: value
                .rhs
                .ok_or_else(|| anyhow!("missing rhs for ternary operation"))?
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
                .ok_or_else(|| anyhow!("missing lhs for assert operation"))?
                .try_into()?,
            rhs: value
                .rhs
                .ok_or_else(|| anyhow!("missing rhs for assert operation"))?
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
