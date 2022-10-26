use std::fmt;

use crate::ir;

use anyhow::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Type {
    Address,
    Boolean,

    Field,
    Group,
    Scalar,

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

    String,
    Struct(Vec<(String, Type)>),
    Record,
}

impl Type {
    pub(crate) fn decode(type_: ir::Type) -> Result<Self> {
        Ok(match type_.class {
            x if x == ir::TypeClass::TypeAddress as i32 => Type::Address,
            x if x == ir::TypeClass::TypeBoolean as i32 => Type::Boolean,
            x if x == ir::TypeClass::TypeField as i32 => Type::Field,
            x if x == ir::TypeClass::TypeGroup as i32 => Type::Group,
            x if x == ir::TypeClass::TypeScalar as i32 => Type::Scalar,
            x if x == ir::TypeClass::TypeU8 as i32 => Type::U8,
            x if x == ir::TypeClass::TypeU16 as i32 => Type::U16,
            x if x == ir::TypeClass::TypeU32 as i32 => Type::U32,
            x if x == ir::TypeClass::TypeU64 as i32 => Type::U64,
            x if x == ir::TypeClass::TypeU128 as i32 => Type::U128,
            x if x == ir::TypeClass::TypeI8 as i32 => Type::I8,
            x if x == ir::TypeClass::TypeI16 as i32 => Type::I16,
            x if x == ir::TypeClass::TypeI32 as i32 => Type::I32,
            x if x == ir::TypeClass::TypeI64 as i32 => Type::I64,
            x if x == ir::TypeClass::TypeI128 as i32 => Type::I128,
            x if x == ir::TypeClass::TypeString as i32 => Type::String,
            x if x == ir::TypeClass::TypeRecord as i32 => Type::Record,
            x if x == ir::TypeClass::TypeStruct as i32 => Type::Struct(
                type_
                    .subtypes
                    .into_iter()
                    .zip(type_.subtype_names.into_iter())
                    .map(|(x, s)| Ok((s, Type::decode(x)?)))
                    .collect::<Result<Vec<_>>>()?,
            ),
            x => return Err(anyhow!("unknown type enum: {}", x)),
        })
    }

    pub(crate) fn encode(&self) -> ir::Type {
        ir::Type {
            class: match self {
                Type::Address => ir::TypeClass::TypeAddress as i32,
                Type::Boolean => ir::TypeClass::TypeBoolean as i32,
                Type::Field => ir::TypeClass::TypeField as i32,
                Type::Group => ir::TypeClass::TypeGroup as i32,
                Type::Scalar => ir::TypeClass::TypeScalar as i32,
                Type::U8 => ir::TypeClass::TypeU8 as i32,
                Type::U16 => ir::TypeClass::TypeU16 as i32,
                Type::U32 => ir::TypeClass::TypeU32 as i32,
                Type::U64 => ir::TypeClass::TypeU64 as i32,
                Type::U128 => ir::TypeClass::TypeU128 as i32,
                Type::I8 => ir::TypeClass::TypeI8 as i32,
                Type::I16 => ir::TypeClass::TypeI16 as i32,
                Type::I32 => ir::TypeClass::TypeI32 as i32,
                Type::I64 => ir::TypeClass::TypeI64 as i32,
                Type::I128 => ir::TypeClass::TypeI128 as i32,
                Type::String => ir::TypeClass::TypeString as i32,
                Type::Struct(_) => ir::TypeClass::TypeStruct as i32,
                Type::Record => ir::TypeClass::TypeRecord as i32,
            },
            subtypes: match self {
                Type::Struct(items) => items.iter().map(|(_, x)| x.encode()).collect(),
                _ => Vec::new(),
            },
            subtype_names: match self {
                Type::Struct(items) => items.iter().map(|(x, _)| x.clone()).collect(),
                _ => Vec::new(),
            },
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Address => write!(f, "address"),
            Type::Boolean => write!(f, "bool"),
            Type::Field => write!(f, "field"),
            Type::Group => write!(f, "group"),
            Type::Scalar => write!(f, "scalar"),
            Type::U8 => write!(f, "u8"),
            Type::U16 => write!(f, "u16"),
            Type::U32 => write!(f, "u32"),
            Type::U64 => write!(f, "u64"),
            Type::U128 => write!(f, "u128"),
            Type::I8 => write!(f, "i8"),
            Type::I16 => write!(f, "i16"),
            Type::I32 => write!(f, "i32"),
            Type::I64 => write!(f, "i64"),
            Type::I128 => write!(f, "i128"),
            Type::String => write!(f, "string"),
            Type::Struct(inner) => {
                write!(f, "{{")?;
                for (i, (name, type_)) in inner.iter().enumerate() {
                    write!(f, "{}{}: {}", if i != 0 { ", " } else { "" }, name, type_)?;
                }
                write!(f, "}}")
            }
            Type::Record => write!(f, "Record"),
        }
    }
}
