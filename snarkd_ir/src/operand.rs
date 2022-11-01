use std::fmt;

use super::ir;
use anyhow::{anyhow, bail, Error, Ok, Result};
use bech32::ToBase32;

pub use ir::operand::{Address, Scalar, Visibility};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupCoordinate {
    GroupField(super::Field),
    SignHigh,
    SignLow,
    Inferred,
}

impl TryFrom<ir::operand::GroupCoordinate> for GroupCoordinate {
    type Error = Error;

    fn try_from(value: ir::operand::GroupCoordinate) -> Result<Self> {
        Ok(
            match value
                .group_coordinate
                .ok_or_else(|| anyhow!("group coordinate unset"))?
            {
                ir::operand::group_coordinate::GroupCoordinate::GroupField(v) => {
                    Self::GroupField(v.try_into()?)
                }
                ir::operand::group_coordinate::GroupCoordinate::SignHigh(_) => Self::SignHigh,
                ir::operand::group_coordinate::GroupCoordinate::SignLow(_) => Self::SignLow,
                ir::operand::group_coordinate::GroupCoordinate::Inferred(_) => Self::Inferred,
            },
        )
    }
}

impl From<GroupCoordinate> for ir::operand::GroupCoordinate {
    fn from(v: GroupCoordinate) -> Self {
        Self {
            group_coordinate: Some(match v {
                GroupCoordinate::GroupField(v) => {
                    ir::operand::group_coordinate::GroupCoordinate::GroupField(v.into())
                }
                GroupCoordinate::SignHigh => {
                    ir::operand::group_coordinate::GroupCoordinate::SignHigh(Default::default())
                }
                GroupCoordinate::SignLow => {
                    ir::operand::group_coordinate::GroupCoordinate::SignLow(Default::default())
                }
                GroupCoordinate::Inferred => {
                    ir::operand::group_coordinate::GroupCoordinate::Inferred(Default::default())
                }
            }),
        }
    }
}

impl fmt::Display for GroupCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GroupCoordinate::GroupField(v) => v.fmt(f),
            GroupCoordinate::SignHigh => write!(f, "+"),
            GroupCoordinate::SignLow => write!(f, "-"),
            GroupCoordinate::Inferred => write!(f, "_"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TupleGroup {
    pub left: GroupCoordinate,
    pub right: GroupCoordinate,
}

impl TryFrom<ir::operand::TupleGroup> for TupleGroup {
    type Error = Error;

    fn try_from(value: ir::operand::TupleGroup) -> Result<Self> {
        Ok(Self {
            left: value
                .left
                .ok_or_else(|| anyhow!("left value of TupleGroup unset"))?
                .try_into()?,
            right: value
                .right
                .ok_or_else(|| anyhow!("right value of TupleGroup unset"))?
                .try_into()?,
        })
    }
}

impl From<TupleGroup> for ir::operand::TupleGroup {
    fn from(value: TupleGroup) -> Self {
        Self {
            left: Some(value.left.into()),
            right: Some(value.right.into()),
        }
    }
}

impl fmt::Display for TupleGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.left, self.right)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Group {
    Single(Field),
    Tuple(TupleGroup),
}

impl TryFrom<ir::operand::Group> for Group {
    type Error = Error;

    fn try_from(value: ir::operand::Group) -> Result<Self> {
        Ok(
            match value.group.ok_or_else(|| anyhow!("group value unset"))? {
                ir::operand::group::Group::Single(v) => Self::Single(v.try_into()?),
                ir::operand::group::Group::Tuple(v) => Self::Tuple(v.try_into()?),
            },
        )
    }
}

impl From<Group> for ir::operand::Group {
    fn from(value: Group) -> Self {
        Self {
            group: Some(match value {
                Group::Single(v) => ir::operand::group::Group::Single(v.into()),
                Group::Tuple(v) => ir::operand::group::Group::Tuple(v.into()),
            }),
        }
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Group::Single(v) => v.fmt(f)?,
            Group::Tuple(v) => v.fmt(f)?,
        };
        write!(f, "group")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub negate: bool,
    pub values: Vec<u64>,
}

impl From<ir::operand::Field> for Field {
    fn from(value: ir::operand::Field) -> Self {
        Self {
            negate: value.negate,
            values: value.values,
        }
    }
}

impl From<Field> for ir::operand::Field {
    fn from(value: Field) -> Self {
        Self {
            negate: value.negate,
            values: value.values,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Struct {
    pub values: Vec<Operand>,
}

impl TryFrom<ir::operand::Struct> for Struct {
    type Error = Error;

    fn try_from(value: ir::operand::Struct) -> Result<Self> {
        Ok(Self {
            values: value
                .values
                .into_iter()
                .map(|v| v.try_into())
                .collect::<Result<_>>()?,
        })
    }
}

impl From<Struct> for ir::operand::Struct {
    fn from(value: Struct) -> Self {
        Self {
            values: value.values.into_iter().map(|v| v.into()).collect(),
        }
    }
}

impl fmt::Display for Struct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "struct(")?;
        for (i, item) in self.values.iter().enumerate() {
            write!(
                f,
                "{}{}",
                item,
                if i == self.values.len() - 1 { "" } else { ", " }
            )?;
        }
        write!(f, ")")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisibleData {
    pub value: Operand,
    pub visibility: ir::operand::Visibility,
}

impl TryFrom<ir::operand::VisibleData> for VisibleData {
    type Error = Error;

    fn try_from(value: ir::operand::VisibleData) -> Result<Self> {
        Ok(Self {
            value: (*value
                .value
                .ok_or_else(|| anyhow!("record data value unset"))?)
            .try_into()?,
            visibility: ir::operand::Visibility::from_i32(value.visibility)
                .ok_or_else(|| anyhow!("invalid visibility for VisibleData"))?,
        })
    }
}

impl From<VisibleData> for ir::operand::VisibleData {
    fn from(value: VisibleData) -> Self {
        Self {
            value: Some(Box::new(value.value.into())),
            visibility: value.visibility as i32,
        }
    }
}

impl fmt::Display for VisibleData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.value, self.visibility)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    /// address
    pub owner: VisibleData,
    /// u64
    pub gates: VisibleData,
    /// any type
    pub data: Vec<VisibleData>,
    /// group
    pub nonce: VisibleData,
}

impl TryFrom<ir::operand::Record> for Record {
    type Error = Error;

    fn try_from(value: ir::operand::Record) -> Result<Self> {
        // TODO this restriction should be conveyed in the protobuf
        let owner =
            VisibleData::try_from(*(value.owner.ok_or_else(|| anyhow!("record owner unset")))?)?;
        if !matches!(owner.value, Operand::Address(_)) {
            bail!("owner must be an address");
        }
        let gates =
            VisibleData::try_from(*(value.gates.ok_or_else(|| anyhow!("record gates unset")))?)?;
        if !matches!(gates.value, Operand::U64(_)) {
            bail!("gates must be a u64");
        }
        let nonce =
            VisibleData::try_from(*(value.nonce.ok_or_else(|| anyhow!("record nonce unset")))?)?;
        if !matches!(nonce.value, Operand::Group(_)) {
            bail!("nonce must be a group");
        }
        Ok(Self {
            owner,
            gates,
            data: value
                .data
                .into_iter()
                .map(|i| i.try_into())
                .collect::<Result<_>>()?,
            nonce,
        })
    }
}

impl From<Record> for ir::operand::Record {
    fn from(value: Record) -> Self {
        Self {
            owner: Some(Box::new(value.owner.into())),
            gates: Some(Box::new(value.gates.into())),
            data: value.data.into_iter().map(|v| v.into()).collect(),
            nonce: Some(Box::new(value.nonce.into())),
        }
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "record(owner: {}, gates: {}, data: (",
            self.owner, self.gates
        )?;
        for (i, item) in self.data.iter().enumerate() {
            write!(
                f,
                "{item}{}",
                if i == self.data.len() - 1 { "" } else { ", " }
            )?;
        }
        write!(f, "), nonce: {})", self.nonce)
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

impl fmt::Display for ir::operand::Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ir::operand::Visibility::Constant => write!(f, "constant"),
            ir::operand::Visibility::Private => write!(f, "private"),
            ir::operand::Visibility::Public => write!(f, "public"),
        }
    }
}

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
            match value.operand.ok_or_else(|| anyhow!("operand unset"))? {
                ir::operand::operand::Operand::Address(v) => Self::Address(v),
                ir::operand::operand::Operand::Boolean(v) => Self::Boolean(v),
                ir::operand::operand::Operand::Field(v) => Self::Field(v.try_into()?),
                ir::operand::operand::Operand::Group(v) => Self::Group(v.try_into()?),
                ir::operand::operand::Operand::U8(v) => Self::U8(v.try_into()?),
                ir::operand::operand::Operand::U16(v) => Self::U16(v.try_into()?),
                ir::operand::operand::Operand::U32(v) => Self::U32(v),
                ir::operand::operand::Operand::U64(v) => Self::U64(v),
                ir::operand::operand::Operand::U128(v) => Self::U128(u128::from_be_bytes(
                    v.try_into()
                        .map_err(|_| anyhow!("invalid bytes for i128"))?,
                )),
                ir::operand::operand::Operand::I8(v) => Self::I8(v.try_into()?),
                ir::operand::operand::Operand::I16(v) => Self::I16(v.try_into()?),
                ir::operand::operand::Operand::I32(v) => Self::I32(v),
                ir::operand::operand::Operand::I64(v) => Self::I64(v),
                ir::operand::operand::Operand::I128(v) => Self::I128(i128::from_be_bytes(
                    v.try_into()
                        .map_err(|_| anyhow!("invalid bytes for i128"))?,
                )),
                ir::operand::operand::Operand::Ref(v) => Self::Ref(v),
                ir::operand::operand::Operand::Scalar(v) => Self::Scalar(v),
                ir::operand::operand::Operand::String(v) => Self::String(v),
                ir::operand::operand::Operand::Record(v) => {
                    Self::Record(Box::new((*v).try_into()?))
                }
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
                Operand::Field(v) => ir::operand::operand::Operand::Field(v.into()),
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
                Operand::Record(v) => ir::operand::operand::Operand::Record(Box::new((*v).into())),
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
