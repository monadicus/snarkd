use snarkd_crypto::Field;

use super::*;
use crate::{ConstrainedBool, ConstraintSystem, Todo};
use anyhow::Result;

/// An integer type enum wrapping the integer value.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum ConstrainedInteger {
    U8(Todo),
    U16(Todo),
    U32(Todo),
    U64(Todo),
    U128(Todo),

    I8(Todo),
    I16(Todo),
    I32(Todo),
    I64(Todo),
    I128(Todo),
}

impl ConstrainedInteger {
    pub fn conditionally_select<F: Field, CS: ConstraintSystem<F>>(
        mut cs: CS,
        cond: &ConstrainedBool,
        first: &Self,
        second: &Self,
    ) -> Result<Self> {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IntegerType {
    U8,
    U16,
    U32,
    U64,
    U128,

    I8,
    I16,
    I32,
    I64,
    I128,
}
