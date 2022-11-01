use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructType {
    pub subtypes: Vec<Type>,
    pub subtype_names: Vec<String>,
}

impl TryFrom<ir::operand::StructType> for StructType {
    type Error = Error;

    fn try_from(value: ir::operand::StructType) -> Result<Self> {
        Ok(Self {
            subtypes: value
                .subtypes
                .into_iter()
                .map(|t| t.try_into())
                .collect::<Result<_>>()?,
            subtype_names: value.subtype_names,
        })
    }
}

impl From<StructType> for ir::operand::StructType {
    fn from(value: StructType) -> Self {
        Self {
            subtypes: value.subtypes.into_iter().map(|t| t.into()).collect(),
            subtype_names: value.subtype_names,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordType {
    pub subtypes: Vec<Type>,
    pub subtype_names: Vec<String>,
    pub visibilities: Vec<Visibility>,
}

impl TryFrom<ir::operand::RecordType> for RecordType {
    type Error = Error;

    fn try_from(value: ir::operand::RecordType) -> Result<Self> {
        Ok(RecordType {
            subtypes: value
                .subtypes
                .into_iter()
                .map(|t| t.try_into())
                .collect::<Result<_>>()?,
            subtype_names: value.subtype_names,
            visibilities: value
                .visibilities
                .into_iter()
                .map(Visibility::from_i32)
                .collect::<Option<_>>()
                .ok_or_else(|| anyhow!("invalid visibility"))?,
        })
    }
}

impl From<RecordType> for ir::operand::RecordType {
    fn from(value: RecordType) -> Self {
        Self {
            subtypes: value.subtypes.into_iter().map(|v| v.into()).collect(),
            subtype_names: value.subtype_names,
            visibilities: value.visibilities.into_iter().map(|v| v as i32).collect(),
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
