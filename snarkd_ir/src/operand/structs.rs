use super::*;

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
