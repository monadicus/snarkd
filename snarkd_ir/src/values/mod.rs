use std::{convert::TryFrom, fmt};

use crate::{
    ir::{self, ProtoBuf},
    Type,
};

use anyhow::*;
use serde::Serialize;

mod address;
mod field;
mod record;
pub use field::*;
mod group;
pub use address::Address;
pub use group::*;
pub use record::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Integer {
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
}

impl TryFrom<Integer> for u32 {
    type Error = anyhow::Error;

    fn try_from(int: Integer) -> Result<Self, Self::Error> {
        match int {
            Integer::U8(n) => Ok(n as u32),
            Integer::U16(n) => Ok(n as u32),
            Integer::U32(n) => Ok(n),
            Integer::U64(n) => u32::try_from(n)
                .map_err(|e| anyhow!("failed to get u32 from u64 int value `{}`: `{}`", n, e)),
            Integer::U128(n) => u32::try_from(n)
                .map_err(|e| anyhow!("failed to get u32 from u128 int value `{}`: `{}`", n, e)),
            _ => Err(anyhow!("cant get u32 from signed int value `{}`", int)),
        }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Integer::U8(x) => write!(f, "{x}"),
            Integer::U16(x) => write!(f, "{x}"),
            Integer::U32(x) => write!(f, "{x}"),
            Integer::U64(x) => write!(f, "{x}"),
            Integer::U128(x) => write!(f, "{x}"),
            Integer::I8(x) => write!(f, "{x}"),
            Integer::I16(x) => write!(f, "{x}"),
            Integer::I32(x) => write!(f, "{x}"),
            Integer::I64(x) => write!(f, "{x}"),
            Integer::I128(x) => write!(f, "{x}"),
        }
    }
}

/// A constant value in IR representation or variable reference
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Value {
    Address(Address),
    Boolean(bool),
    Field(Field),
    Group(Group),
    Integer(Integer),
    Struct(Vec<Value>),
    Str(String),
    Ref(u32), // reference to a variable
    Scalar(Vec<u64>),
    Record(Record),
}

impl Value {
    pub fn matches_input_type(&self, type_: &Type) -> bool {
        match (self, type_) {
            (Value::Address(_), Type::Address)
            | (Value::Boolean(_), Type::Boolean)
            | (Value::Field(_), Type::Field)
            | (Value::Group(_), Type::Group)
            | (Value::Integer(Integer::I8(_)), Type::I8)
            | (Value::Integer(Integer::I16(_)), Type::I16)
            | (Value::Integer(Integer::I32(_)), Type::I32)
            | (Value::Integer(Integer::I64(_)), Type::I64)
            | (Value::Integer(Integer::I128(_)), Type::I128)
            | (Value::Integer(Integer::U8(_)), Type::U8)
            | (Value::Integer(Integer::U16(_)), Type::U16)
            | (Value::Integer(Integer::U32(_)), Type::U32)
            | (Value::Integer(Integer::U64(_)), Type::U64)
            | (Value::Integer(Integer::U128(_)), Type::U128) => true,
            (Value::Struct(values), Type::Struct(types)) => values
                .iter()
                .zip(types.iter())
                .all(|(value, (_, type_))| value.matches_input_type(type_)),
            (Value::Record(values), Type::Record(types)) => {
                values
                    .data
                    .iter()
                    .zip(types.data.iter())
                    .all(|(data, (_, type_, vis))| {
                        data.value.matches_input_type(type_) && data.visibility == *vis
                    })
                    && values.owner.visibility == types.owner
                    && values.gates.visibility == types.gates
                    && values.nonce.visibility == types.nonce
            }
            (Value::Ref(_), _) => panic!("illegal ref in input type"),
            (_, _) => false,
        }
    }
}

impl ProtoBuf for Value {
    type Target = ir::Operand;

    fn encode(&self) -> Self::Target {
        match self {
            Value::Address(address) => ir::Operand {
                address: Some(address.encode()),
                ..Default::default()
            },
            Value::Boolean(value) => ir::Operand {
                boolean: Some(ir::Bool { boolean: *value }),
                ..Default::default()
            },
            Value::Field(field) => ir::Operand {
                field: Some(field.encode()),
                ..Default::default()
            },
            Value::Group(Group::Single(inner)) => ir::Operand {
                group_single: Some(inner.encode()),
                ..Default::default()
            },
            Value::Group(Group::Tuple(left, right)) => ir::Operand {
                group_tuple: Some(ir::Group {
                    left: Some(left.encode()),
                    right: Some(right.encode()),
                }),
                ..Default::default()
            },
            Value::Integer(i) => match i {
                Integer::U8(i) => ir::Operand {
                    u8: Some(ir::U8 { u8: *i as u32 }),
                    ..Default::default()
                },
                Integer::U16(i) => ir::Operand {
                    u16: Some(ir::U16 { u16: *i as u32 }),
                    ..Default::default()
                },
                Integer::U32(i) => ir::Operand {
                    u32: Some(ir::U32 { u32: *i }),
                    ..Default::default()
                },
                Integer::U64(i) => ir::Operand {
                    u64: Some(ir::U64 { u64: *i }),
                    ..Default::default()
                },
                Integer::U128(i) => ir::Operand {
                    u128: Some(ir::U128 {
                        u128: i.to_le_bytes().to_vec(),
                    }),
                    ..Default::default()
                },
                Integer::I8(i) => ir::Operand {
                    i8: Some(ir::I8 { i8: *i as i32 }),
                    ..Default::default()
                },
                Integer::I16(i) => ir::Operand {
                    i16: Some(ir::I16 { i16: *i as i32 }),
                    ..Default::default()
                },
                Integer::I32(i) => ir::Operand {
                    i32: Some(ir::I32 { i32: *i }),
                    ..Default::default()
                },
                Integer::I64(i) => ir::Operand {
                    i64: Some(ir::I64 { i64: *i as i64 }),
                    ..Default::default()
                },
                Integer::I128(i) => ir::Operand {
                    i128: Some(ir::I128 {
                        i128: i.to_le_bytes().to_vec(),
                    }),
                    ..Default::default()
                },
            },
            Value::Ref(variable_ref) => ir::Operand {
                variable_ref: Some(ir::VariableRef {
                    variable_ref: *variable_ref,
                }),
                ..Default::default()
            },
            Value::Struct(items) => ir::Operand {
                structure: Some(ir::Struct {
                    values: items.iter().map(|x| x.encode()).collect(),
                }),
                ..Default::default()
            },
            Value::Str(str) => ir::Operand {
                string: Some(ir::String {
                    string: str.clone(),
                }),
                ..Default::default()
            },
            Value::Scalar(items) => ir::Operand {
                scalar: Some(ir::Scalar {
                    values: items.clone(),
                }),
                ..Default::default()
            },
            Value::Record(record) => ir::Operand {
                record: Some(Box::new(record.encode())),
                ..Default::default()
            },
        }
    }

    fn decode(target: Self::Target) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(match target {
            ir::Operand {
                address: Some(address),
                ..
            } => Value::Address(Address::decode(address)?),
            ir::Operand {
                boolean: Some(boolean),
                ..
            } => Value::Boolean(boolean.boolean),
            ir::Operand {
                field: Some(field), ..
            } => Value::Field(Field::decode(field)?),
            ir::Operand {
                group_single: Some(group_single),
                ..
            } => Value::Group(Group::Single(Field::decode(group_single)?)),
            ir::Operand {
                group_tuple:
                    Some(ir::Group {
                        left: Some(left),
                        right: Some(right),
                    }),
                ..
            } => Value::Group(Group::Tuple(
                GroupCoordinate::decode(left)?,
                GroupCoordinate::decode(right)?,
            )),
            ir::Operand { u8: Some(u8), .. } => Value::Integer(Integer::U8(u8.u8 as u8)),
            ir::Operand { u16: Some(u16), .. } => Value::Integer(Integer::U16(u16.u16 as u16)),
            ir::Operand { u32: Some(u32), .. } => Value::Integer(Integer::U32(u32.u32)),
            ir::Operand { u64: Some(u64), .. } => Value::Integer(Integer::U64(u64.u64)),
            ir::Operand {
                u128: Some(u128), ..
            } => {
                let mut raw = [0u8; 16];
                let len = u128.u128.len().min(16);
                raw[..len].copy_from_slice(&u128.u128[..len]);
                Value::Integer(Integer::U128(u128::from_le_bytes(raw)))
            }
            ir::Operand { i8: Some(i8), .. } => Value::Integer(Integer::I8(i8.i8 as i8)),
            ir::Operand { i16: Some(i16), .. } => Value::Integer(Integer::I16(i16.i16 as i16)),
            ir::Operand { i32: Some(i32), .. } => Value::Integer(Integer::I32(i32.i32)),
            ir::Operand { i64: Some(i64), .. } => Value::Integer(Integer::I64(i64.i64)),
            ir::Operand {
                i128: Some(i128), ..
            } => {
                let mut raw = [0u8; 16];
                let len = i128.i128.len().min(16);
                raw[..len].copy_from_slice(&i128.i128[..len]);
                Value::Integer(Integer::I128(i128::from_le_bytes(raw)))
            }
            ir::Operand {
                variable_ref: Some(variable_ref),
                ..
            } => Value::Ref(variable_ref.variable_ref),
            ir::Operand {
                structure: Some(structure),
                ..
            } => Value::Struct(
                structure
                    .values
                    .into_iter()
                    .map(Value::decode)
                    .collect::<Result<Vec<_>>>()?,
            ),
            ir::Operand {
                scalar: Some(scalar),
                ..
            } => Value::Scalar(scalar.values),
            ir::Operand {
                string: Some(str), ..
            } => Value::Str(str.string),
            ir::Operand {
                record: Some(record),
                ..
            } => Value::Record(Record::decode(*record)?),
            x => return Err(anyhow!("illegal value data: {:?}", x)),
        })
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Address(address) => write!(f, "{address}"),
            Value::Boolean(x) => write!(f, "{x}"),
            Value::Field(field) => write!(f, "{field}"),
            Value::Group(group) => write!(f, "{group}"),
            Value::Integer(x) => write!(f, "{x}"),
            Value::Struct(items) => {
                write!(f, "struct(")?;
                for (i, item) in items.iter().enumerate() {
                    write!(
                        f,
                        "{}{}",
                        item,
                        if i == items.len() - 1 { "" } else { ", " }
                    )?;
                }
                write!(f, ")")
            }
            Value::Str(s) => write!(f, "\"{s}\""),
            Value::Ref(x) => write!(f, "{x}"),
            Value::Scalar(x) => write!(f, "scalar{x:?}"),
            Value::Record(record) => write!(f, "{}", record),
        }
    }
}
