use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisibleData {
    pub value: Operand,
    pub visibility: ir::operand::Visibility,
}

impl TryFrom<ir::operand::VisibleData> for VisibleData {
    type Error = Error;

    fn try_from(value: ir::operand::VisibleData) -> Result<Self> {
        Ok(Self {
            value: value
                .value
                .ok_or_else(|| IRError::unset("Record data value"))?
                .try_into()?,
            visibility: ir::operand::Visibility::from_i32(value.visibility)
                .ok_or_else(|| IRError::invalid_visibility("VisibleData"))?,
        })
    }
}

impl From<VisibleData> for ir::operand::VisibleData {
    fn from(value: VisibleData) -> Self {
        Self {
            value: Some(value.value.into()),
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
    pub owner: Address,
    pub owner_visibility: Visibility,
    pub gates: u64,
    pub gates_visibility: Visibility,
    pub data: Vec<VisibleData>,
    pub nonce: Group,
    pub nonce_visibility: Visibility,
}

impl TryFrom<ir::operand::Record> for Record {
    type Error = Error;

    fn try_from(value: ir::operand::Record) -> Result<Self> {
        Ok(Self {
            owner: value.owner.ok_or_else(|| IRError::unset("Record owner"))?,
            owner_visibility: Visibility::from_i32(value.owner_visibility)
                .ok_or_else(|| IRError::invalid_visibility("Record owner"))?,
            gates: value.gates,
            gates_visibility: Visibility::from_i32(value.gates_visibility)
                .ok_or_else(|| IRError::invalid_visibility("Record gates"))?,
            data: value
                .data
                .into_iter()
                .map(|i| i.try_into())
                .collect::<Result<_>>()?,
            nonce: value
                .nonce
                .ok_or_else(|| IRError::unset("Record nonce"))?
                .try_into()?,
            nonce_visibility: Visibility::from_i32(value.nonce_visibility)
                .ok_or_else(|| IRError::invalid_visibility("Record nonce"))?,
        })
    }
}

impl From<Record> for ir::operand::Record {
    fn from(value: Record) -> Self {
        Self {
            owner: Some(value.owner),
            owner_visibility: value.owner_visibility as i32,
            gates: value.gates,
            gates_visibility: value.gates_visibility as i32,
            data: value.data.into_iter().map(|v| v.into()).collect(),
            nonce: Some(value.nonce.into()),
            nonce_visibility: value.nonce_visibility as i32,
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
