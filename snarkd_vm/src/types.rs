use std::fmt;

use crate::{ir, visibility::Visibility};

use anyhow::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RecordType {
    pub owner: Visibility,
    pub gates: Visibility,
    pub data: Vec<(String, Type, Visibility)>,
    pub nonce: Visibility,
}

impl RecordType {
    pub fn decode(ir: ir::Type) -> Result<Self> {
        if ir.visibilities.len() < 3 {
            return Err(anyhow!("invalid visibilities for record"));
        }
        Ok(Self {
            owner: ir.visibilities[0].try_into()?,
            gates: ir.visibilities[1].try_into()?,
            nonce: ir.visibilities[2].try_into()?,
            data: ir
                .visibilities
                .into_iter()
                .skip(3)
                .zip(ir.subtype_names.into_iter())
                .zip(ir.subtypes.into_iter())
                .map(|((vis, name), ty)| Ok((name, Type::decode(ty)?, Visibility::try_from(vis)?)))
                .collect::<Result<_>>()?,
        })
    }

    pub fn visibilities(&self) -> Vec<i32> {
        let mut v = vec![self.owner as i32, self.gates as i32, self.nonce as i32];
        v.extend(self.data.iter().map(|(_, _, v)| *v as i32));
        v
    }
}

impl fmt::Display for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "record{{owner: address.{}, gates: u64.{}, ",
            // these are just the visibilities... not the values
            self.owner,
            self.gates
        )?;
        for item in &self.data {
            write!(f, "{}: {}.{}, ", item.0, item.1, item.2,)?;
        }
        write!(f, "nonce: {}}}", self.nonce)
    }
}

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
    Record(RecordType),
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
            x if x == ir::TypeClass::TypeRecord as i32 => Type::Record(RecordType::decode(type_)?),
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
                Type::Record(_) => ir::TypeClass::TypeRecord as i32,
            },
            subtypes: match self {
                Type::Struct(items) => items.iter().map(|(_, x)| x.encode()).collect(),
                Type::Record(RecordType { data, .. }) => {
                    data.iter().map(|(_, x, _)| x.encode()).collect()
                }
                _ => Vec::new(),
            },
            subtype_names: match self {
                Type::Struct(items) => items.iter().map(|(x, _)| x.clone()).collect(),
                Type::Record(RecordType { data, .. }) => {
                    data.iter().map(|(x, _, _)| x.clone()).collect()
                }
                _ => Vec::new(),
            },
            visibilities: match self {
                Type::Record(record) => record.visibilities(),
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
                write!(f, "struct{{")?;
                for (i, (name, type_)) in inner.iter().enumerate() {
                    write!(f, "{}{}: {}", if i != 0 { ", " } else { "" }, name, type_)?;
                }
                write!(f, "}}")
            }
            Type::Record(inner) => write!(f, "{inner}"),
        }
    }
}
