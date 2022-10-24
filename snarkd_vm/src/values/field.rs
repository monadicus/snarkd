use std::fmt;

use crate::ir;

use serde::Serialize;

/// An unbounded field value
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Field {
    pub negate: bool,
    pub values: Vec<u64>,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let negate = if self.negate { "-" } else { "" };
        write!(f, "{}{:?}", negate, self.values)
    }
}

impl Field {
    pub(crate) fn decode(from: ir::Field) -> Self {
        Self {
            negate: from.negate,
            values: from.values,
        }
    }

    pub(crate) fn encode(&self) -> ir::Field {
        ir::Field {
            negate: self.negate,
            values: self.values.clone(),
        }
    }
}
