use std::fmt;

use crate::{ir, Field};

use anyhow::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum GroupCoordinate {
    Field(Field),
    SignHigh,
    SignLow,
    Inferred,
}

impl fmt::Display for GroupCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GroupCoordinate::Field(field) => write!(f, "{}", field),
            GroupCoordinate::SignHigh => write!(f, "+"),
            GroupCoordinate::SignLow => write!(f, "-"),
            GroupCoordinate::Inferred => write!(f, "_"),
        }
    }
}

impl GroupCoordinate {
    pub(crate) fn decode(from: ir::GroupCoordinate) -> Result<GroupCoordinate> {
        match from.coordinate_type {
            x if x == ir::GroupCoordinateType::GroupField as i32 => Ok(GroupCoordinate::Field(Field::decode(
                from.field.ok_or_else(|| anyhow!("missing field value"))?,
            ))),
            x if x == ir::GroupCoordinateType::SignHigh as i32 => Ok(GroupCoordinate::SignHigh),
            x if x == ir::GroupCoordinateType::SignLow as i32 => Ok(GroupCoordinate::SignLow),
            x if x == ir::GroupCoordinateType::Inferred as i32 => Ok(GroupCoordinate::Inferred),
            x => Err(anyhow!("unknown group coordinate type: {}", x)),
        }
    }

    pub(crate) fn encode(&self) -> ir::GroupCoordinate {
        ir::GroupCoordinate {
            coordinate_type: match self {
                GroupCoordinate::Field(_) => ir::GroupCoordinateType::GroupField as i32,
                GroupCoordinate::SignHigh => ir::GroupCoordinateType::SignHigh as i32,
                GroupCoordinate::SignLow => ir::GroupCoordinateType::SignLow as i32,
                GroupCoordinate::Inferred => ir::GroupCoordinateType::Inferred as i32,
            },
            field: match self {
                GroupCoordinate::Field(f) => Some(f.encode()),
                _ => None,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Group {
    Single(Field),
    Tuple(GroupCoordinate, GroupCoordinate),
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Group::Single(field) => write!(f, "{}group", field),
            Group::Tuple(left, right) => write!(f, "({}, {})group", left, right),
        }
    }
}
