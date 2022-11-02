use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructTypeEntry {
    pub name: String,
    pub type_: Type,
}

impl TryFrom<ir::operand::StructTypeEntry> for StructTypeEntry {
    type Error = Error;

    fn try_from(value: ir::operand::StructTypeEntry) -> Result<Self> {
        Ok(Self {
            name: value.name,
            type_: value
                .r#type
                .ok_or_else(|| anyhow!("StructTypeEntry type unset"))?
                .try_into()?,
        })
    }
}

impl From<StructTypeEntry> for ir::operand::StructTypeEntry {
    fn from(value: StructTypeEntry) -> Self {
        Self {
            name: value.name,
            r#type: Some(value.type_.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructType {
    pub fields: Vec<StructTypeEntry>,
}

impl TryFrom<ir::operand::StructType> for StructType {
    type Error = Error;

    fn try_from(value: ir::operand::StructType) -> Result<Self> {
        Ok(Self {
            fields: value
                .fields
                .into_iter()
                .map(|f| f.try_into())
                .collect::<Result<_>>()?,
        })
    }
}

impl From<StructType> for ir::operand::StructType {
    fn from(value: StructType) -> Self {
        Self {
            fields: value.fields.into_iter().map(|f| f.into()).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordTypeEntry {
    pub name: String,
    pub type_: Type,
    pub visibility: Visibility,
}

impl TryFrom<ir::operand::RecordTypeEntry> for RecordTypeEntry {
    type Error = Error;

    fn try_from(value: ir::operand::RecordTypeEntry) -> Result<Self> {
        Ok(Self {
            name: value.name,
            type_: value
                .r#type
                .ok_or_else(|| anyhow!("RecordTypeEntry type unset"))?
                .try_into()?,
            visibility: Visibility::from_i32(value.visibility)
                .ok_or_else(|| anyhow!("RecordTypeEntry invalid visibility"))?,
        })
    }
}

impl From<RecordTypeEntry> for ir::operand::RecordTypeEntry {
    fn from(value: RecordTypeEntry) -> Self {
        Self {
            name: value.name,
            r#type: Some(value.type_.into()),
            visibility: value.visibility as i32,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordType {
    pub owner: Visibility,
    pub gates: Visibility,
    pub data: Vec<RecordTypeEntry>,
    pub nonce: Visibility,
}

impl TryFrom<ir::operand::RecordType> for RecordType {
    type Error = Error;

    fn try_from(value: ir::operand::RecordType) -> Result<Self> {
        Ok(RecordType {
            owner: Visibility::from_i32(value.owner)
                .ok_or_else(|| anyhow!("invalid owner visibility"))?,
            gates: Visibility::from_i32(value.gates)
                .ok_or_else(|| anyhow!("invalid gates visibility"))?,
            data: value
                .data
                .into_iter()
                .map(|v| v.try_into())
                .collect::<Result<_>>()?,
            nonce: Visibility::from_i32(value.nonce)
                .ok_or_else(|| anyhow!("invalid visibility"))?,
        })
    }
}

impl From<RecordType> for ir::operand::RecordType {
    fn from(value: RecordType) -> Self {
        Self {
            owner: value.owner as i32,
            gates: value.gates as i32,
            data: value.data.into_iter().map(|v| v.into()).collect(),
            nonce: value.nonce as i32,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Address,
    Boolean,
    Field,
    Group,
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
    Scalar,
    String,
    Struct(StructType),
    Record(RecordType),
}

impl TryFrom<ir::operand::Type> for Type {
    type Error = Error;

    fn try_from(value: ir::operand::Type) -> Result<Self> {
        Ok(
            match value.r#type.ok_or_else(|| anyhow!("type value not set"))? {
                ir::operand::r#type::Type::Address(_) => Self::Address,
                ir::operand::r#type::Type::Boolean(_) => Self::Boolean,
                ir::operand::r#type::Type::Field(_) => Self::Field,
                ir::operand::r#type::Type::Group(_) => Self::Group,
                ir::operand::r#type::Type::U8(_) => Self::U8,
                ir::operand::r#type::Type::U16(_) => Self::U16,
                ir::operand::r#type::Type::U32(_) => Self::U32,
                ir::operand::r#type::Type::U64(_) => Self::U64,
                ir::operand::r#type::Type::U128(_) => Self::U128,
                ir::operand::r#type::Type::I8(_) => Self::I8,
                ir::operand::r#type::Type::I16(_) => Self::I16,
                ir::operand::r#type::Type::I32(_) => Self::I32,
                ir::operand::r#type::Type::I64(_) => Self::I64,
                ir::operand::r#type::Type::I128(_) => Self::I128,
                ir::operand::r#type::Type::Scalar(_) => Self::Scalar,
                ir::operand::r#type::Type::String(_) => Self::String,
                ir::operand::r#type::Type::Struct(v) => Self::Struct(v.try_into()?),
                ir::operand::r#type::Type::Record(v) => Self::Record(v.try_into()?),
            },
        )
    }
}

impl From<Type> for ir::operand::Type {
    fn from(value: Type) -> Self {
        Self {
            r#type: Some(match value {
                Type::Address => ir::operand::r#type::Type::Address(Default::default()),
                Type::Boolean => ir::operand::r#type::Type::Boolean(Default::default()),
                Type::Field => ir::operand::r#type::Type::Field(Default::default()),
                Type::Group => ir::operand::r#type::Type::Group(Default::default()),
                Type::U8 => ir::operand::r#type::Type::U8(Default::default()),
                Type::U16 => ir::operand::r#type::Type::U16(Default::default()),
                Type::U32 => ir::operand::r#type::Type::U32(Default::default()),
                Type::U64 => ir::operand::r#type::Type::U64(Default::default()),
                Type::U128 => ir::operand::r#type::Type::U128(Default::default()),
                Type::I8 => ir::operand::r#type::Type::I8(Default::default()),
                Type::I16 => ir::operand::r#type::Type::I16(Default::default()),
                Type::I32 => ir::operand::r#type::Type::I32(Default::default()),
                Type::I64 => ir::operand::r#type::Type::I64(Default::default()),
                Type::I128 => ir::operand::r#type::Type::I128(Default::default()),
                Type::Scalar => ir::operand::r#type::Type::Scalar(Default::default()),
                Type::String => ir::operand::r#type::Type::String(Default::default()),
                Type::Struct(v) => ir::operand::r#type::Type::Struct(v.into()),
                Type::Record(v) => ir::operand::r#type::Type::Record(v.into()),
            }),
        }
    }
}
