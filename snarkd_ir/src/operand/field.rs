use super::*;

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
