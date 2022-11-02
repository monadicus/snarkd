use super::*;

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
                .ok_or_else(|| IRError::unset("Group coordinate"))?
            {
                ir::operand::group_coordinate::GroupCoordinate::GroupField(v) => {
                    Self::GroupField(v)
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
                    ir::operand::group_coordinate::GroupCoordinate::GroupField(v)
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
                .ok_or_else(|| IRError::tuple_group_set_side_unset("left"))?
                .try_into()?,
            right: value
                .right
                .ok_or_else(|| IRError::tuple_group_set_side_unset("right"))?
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
    Single(ir::operand::Field),
    Tuple(TupleGroup),
}

impl TryFrom<ir::operand::Group> for Group {
    type Error = Error;

    fn try_from(value: ir::operand::Group) -> Result<Self> {
        Ok(
            match value.group.ok_or_else(|| IRError::unset("Group value"))? {
                ir::operand::group::Group::Single(v) => Self::Single(v),
                ir::operand::group::Group::Tuple(v) => Self::Tuple(v.try_into()?),
            },
        )
    }
}

impl From<Group> for ir::operand::Group {
    fn from(value: Group) -> Self {
        Self {
            group: Some(match value {
                Group::Single(v) => ir::operand::group::Group::Single(v),
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
