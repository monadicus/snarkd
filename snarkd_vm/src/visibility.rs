use crate::ir;
use anyhow::{anyhow, Error, Result};
use serde::Serialize;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[repr(i32)]
pub enum Visibility {
    Constant,
    Private,
    Public,
}

impl Visibility {
    pub fn decode(from: ir::Visibility) -> Result<Self> {
        Ok(match from {
            ir::Visibility::Constant => Self::Constant,
            ir::Visibility::Private => Self::Private,
            ir::Visibility::Public => Self::Public,
        })
    }

    pub fn encode(&self) -> ir::Visibility {
        match self {
            Self::Constant => ir::Visibility::Constant,
            Self::Private => ir::Visibility::Private,
            Self::Public => ir::Visibility::Public,
        }
    }
}

impl TryFrom<i32> for Visibility {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Constant),
            1 => Ok(Self::Private),
            2 => Ok(Self::Public),
            _ => Err(anyhow!("can't convert {value} to visibility")),
        }
    }
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Visibility::Constant => write!(f, "constant"),
            Visibility::Private => write!(f, "private"),
            Visibility::Public => write!(f, "public"),
        }
    }
}
