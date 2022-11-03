mod group;
pub use group::*;

mod record;
pub use record::*;

mod structs;
pub use structs::*;

mod types;
pub use types::*;

use crate::ir;
use bech32::ToBase32;
pub use ir::operand::{Address, Field, Scalar, Visibility};
use snarkd_errors::{Error, IRError, IntoSnarkdError, Result};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Address(Address),
    Boolean(bool),
    Field(Field),
    Group(Group),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Ref(u32),
    Scalar(Scalar),
    String(String),
    Record(Box<Record>),
    Struct(Struct),
}

impl TryFrom<ir::operand::Operand> for Operand {
    type Error = Error;

    fn try_from(value: ir::operand::Operand) -> Result<Self> {
        Ok(
            match value.operand.ok_or_else(|| IRError::unset("Operand"))? {
                ir::operand::operand::Operand::Address(v) => Self::Address(v),
                ir::operand::operand::Operand::Boolean(v) => Self::Boolean(v),
                ir::operand::operand::Operand::Field(v) => Self::Field(v),
                ir::operand::operand::Operand::Group(v) => Self::Group(v.try_into()?),
                ir::operand::operand::Operand::U8(v) => {
                    Self::U8(v.try_into().to_error(IRError::cast_int_error)?)
                }
                ir::operand::operand::Operand::U16(v) => {
                    Self::U16(v.try_into().to_error(IRError::cast_int_error)?)
                }
                ir::operand::operand::Operand::U32(v) => Self::U32(v),
                ir::operand::operand::Operand::U64(v) => Self::U64(v),
                ir::operand::operand::Operand::U128(v) => Self::U128(u128::from_be_bytes(
                    v.try_into()
                        .map_err(|_| IRError::invalid_num_of_bytes("u128"))?,
                )),
                ir::operand::operand::Operand::I8(v) => {
                    Self::I8(v.try_into().to_error(IRError::cast_int_error)?)
                }
                ir::operand::operand::Operand::I16(v) => {
                    Self::I16(v.try_into().to_error(IRError::cast_int_error)?)
                }
                ir::operand::operand::Operand::I32(v) => Self::I32(v),
                ir::operand::operand::Operand::I64(v) => Self::I64(v),
                ir::operand::operand::Operand::I128(v) => Self::I128(i128::from_be_bytes(
                    v.try_into()
                        .map_err(|_| IRError::invalid_num_of_bytes("i128"))?,
                )),
                ir::operand::operand::Operand::Ref(v) => Self::Ref(v),
                ir::operand::operand::Operand::Scalar(v) => Self::Scalar(v),
                ir::operand::operand::Operand::String(v) => Self::String(v),
                ir::operand::operand::Operand::Record(v) => Self::Record(Box::new((v).try_into()?)),
                ir::operand::operand::Operand::Struct(v) => Self::Struct(v.try_into()?),
            },
        )
    }
}

impl From<Operand> for ir::operand::Operand {
    fn from(value: Operand) -> Self {
        Self {
            operand: Some(match value {
                Operand::Address(v) => ir::operand::operand::Operand::Address(v),
                Operand::Boolean(v) => ir::operand::operand::Operand::Boolean(v),
                Operand::Field(v) => ir::operand::operand::Operand::Field(v),
                Operand::Group(v) => ir::operand::operand::Operand::Group(v.into()),
                Operand::U8(v) => ir::operand::operand::Operand::U8(v.into()),
                Operand::U16(v) => ir::operand::operand::Operand::U16(v.into()),
                Operand::U32(v) => ir::operand::operand::Operand::U32(v),
                Operand::U64(v) => ir::operand::operand::Operand::U64(v),
                Operand::U128(v) => ir::operand::operand::Operand::U128(v.to_be_bytes().to_vec()),
                Operand::I8(v) => ir::operand::operand::Operand::I8(v as i32),
                Operand::I16(v) => ir::operand::operand::Operand::I16(v as i32),
                Operand::I32(v) => ir::operand::operand::Operand::I32(v),
                Operand::I64(v) => ir::operand::operand::Operand::I64(v),
                Operand::I128(v) => ir::operand::operand::Operand::I128(v.to_be_bytes().to_vec()),
                Operand::Ref(v) => ir::operand::operand::Operand::Ref(v),
                Operand::Scalar(v) => ir::operand::operand::Operand::Scalar(v),
                Operand::String(v) => ir::operand::operand::Operand::String(v),
                Operand::Record(v) => ir::operand::operand::Operand::Record((*v).into()),
                Operand::Struct(v) => ir::operand::operand::Operand::Struct(v.into()),
            }),
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Address(v) => v.fmt(f),
            Operand::Boolean(v) => v.fmt(f),
            Operand::Field(v) => v.fmt(f),
            Operand::Group(v) => v.fmt(f),
            Operand::U8(v) => v.fmt(f),
            Operand::U16(v) => v.fmt(f),
            Operand::U32(v) => v.fmt(f),
            Operand::U64(v) => v.fmt(f),
            Operand::U128(v) => v.fmt(f),
            Operand::I8(v) => v.fmt(f),
            Operand::I16(v) => v.fmt(f),
            Operand::I32(v) => v.fmt(f),
            Operand::I64(v) => v.fmt(f),
            Operand::I128(v) => v.fmt(f),
            Operand::Ref(v) => v.fmt(f),
            Operand::Scalar(v) => v.fmt(f),
            Operand::String(v) => v.fmt(f),
            Operand::Record(v) => v.fmt(f),
            Operand::Struct(v) => v.fmt(f),
        }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            bech32::encode("aleo", self.address.to_base32(), bech32::Variant::Bech32)
                .unwrap_or_default()
        )
    }
}

impl Eq for Address {}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "scalar{:?}", self.values)
    }
}
impl Eq for Scalar {}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Visibility::Constant => write!(f, "constant"),
            Visibility::Private => write!(f, "private"),
            Visibility::Public => write!(f, "public"),
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.negate {
            write!(f, "-")?
        };
        write!(f, "{:?}", self.values)
    }
}
impl Eq for Field {}
