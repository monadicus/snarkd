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
            value: (*value
                .value
                .ok_or_else(|| IRError::unset("Record data value unset."))?)
            .try_into()?,
            visibility: ir::operand::Visibility::from_i32(value.visibility)
                .ok_or_else(|| IRError::invalid_visibility("VisibleData"))?,
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
            VisibleData::try_from(*(value.owner.ok_or_else(|| IRError::unset("Record owner")))?)?;
        if !matches!(owner.value, Operand::Address(_)) {
            todo!("owner must be an address");
        }
        let gates =
            VisibleData::try_from(*(value.gates.ok_or_else(|| IRError::unset("Record gates")))?)?;
        if !matches!(gates.value, Operand::U64(_)) {
            todo!("gates must be a u64");
        }
        let nonce =
            VisibleData::try_from(*(value.nonce.ok_or_else(|| IRError::unset("Record nonce")))?)?;
        if !matches!(nonce.value, Operand::Group(_)) {
            todo!("nonce must be a group");
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
