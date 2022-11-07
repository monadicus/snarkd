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
                .ok_or_else(|| IRError::unset("StructTypeEntry type"))?
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

impl fmt::Display for StructTypeEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.type_)
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

impl fmt::Display for StructType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "struct(")?;
        for (i, item) in self.fields.iter().enumerate() {
            write!(
                f,
                "{}{}",
                item,
                if i == self.fields.len() - 1 { "" } else { ", " }
            )?;
        }
        write!(f, ")")
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
                .ok_or_else(|| IRError::unset("RecordTypeEntry type"))?
                .try_into()?,
            visibility: Visibility::from_i32(value.visibility)
                .ok_or_else(|| IRError::invalid_visibility("RecordTypeEntry"))?,
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

impl fmt::Display for RecordTypeEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}.{}", self.name, self.type_, self.visibility)
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
                .ok_or_else(|| IRError::invalid_visibility("owner"))?,
            gates: Visibility::from_i32(value.gates)
                .ok_or_else(|| IRError::invalid_visibility("gates"))?,
            data: value
                .data
                .into_iter()
                .map(|v| v.try_into())
                .collect::<Result<_>>()?,
            nonce: Visibility::from_i32(value.nonce)
                .ok_or_else(|| IRError::invalid_visibility("nonce"))?,
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

impl fmt::Display for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "record(owner: {}, gates: {}, data: (",
            self.owner, self.gates
        )?;
        for item in self.data.iter() {
            write!(f, "{item},")?;
        }
        write!(f, "), nonce: {})", self.nonce)
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

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Address => write!(f, "address"),
            Type::Boolean => write!(f, "boolean"),
            Type::Field => write!(f, "field"),
            Type::Group => write!(f, "group"),
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
            Type::Scalar => write!(f, "scalar"),
            Type::String => write!(f, "string"),
            Type::Struct(s) => s.fmt(f),
            Type::Record(r) => r.fmt(f),
        }
    }
}

impl TryFrom<ir::operand::Type> for Type {
    type Error = Error;

    fn try_from(value: ir::operand::Type) -> Result<Self> {
        Ok(
            match value.r#type.ok_or_else(|| IRError::unset("type value"))? {
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
