use crate::ConstraintSystem;

use super::*;
use std::fmt::{Debug, Display};

#[derive(Clone, Debug)]
pub struct ConstrainedField<F: Field>(pub F);

impl<F: Field> ConstrainedField<F> {
    pub fn conditionally_select<CS: ConstraintSystem<F>>(
        mut cs: CS,
        cond: &ConstrainedBool,
        first: &Self,
        second: &Self,
    ) -> Result<Self> {
        todo!()
    }

    pub fn constant<CS: ConstraintSystem<F>>(
        cs: &mut CS,
        field: &snarkd_ir::Field,
    ) -> Result<Self> {
        todo!()
    }

    pub fn from_input<G: Parameters, CS: ConstraintSystem<F>>(
        cs: &mut CS,
        name: &str,
        operand: Operand,
    ) -> Result<ConstrainedValue<F, G>> {
        todo!()
    }
}

impl<F: Field> Display for ConstrainedField<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "field")
    }
}
