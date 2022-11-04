use crate::ConstraintSystem;

use super::*;
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, Debug)]
pub struct ConstrainedBool(pub bool);

impl ConstrainedBool {
    pub fn conditionally_select<F: Field, CS: ConstraintSystem<F>>(
        mut cs: CS,
        cond: &ConstrainedBool,
        first: &Self,
        second: &Self,
    ) -> Result<Self> {
        todo!()
    }
}

impl Display for ConstrainedBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "boolean")
    }
}
