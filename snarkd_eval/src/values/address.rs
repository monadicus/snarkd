use crate::ConstraintSystem;

use super::*;

#[derive(Clone, Debug)]
pub struct ConstrainedAddress<G: Group> {
    pub address: G,
    pub bytes: Vec<u8>,
}

impl<G: Group> ConstrainedAddress<G> {
    pub fn conditionally_select<F: Field, CS: ConstraintSystem<F>>(
        mut cs: CS,
        cond: &ConstrainedBool,
        first: &Self,
        second: &Self,
    ) -> Result<Self> {
        todo!()
    }

    pub fn constant(address: &Address) -> Result<Self> {
        todo!()
    }

    pub fn from_input<F: Field, CS: ConstraintSystem<F>>(
        cs: &mut CS,
        name: &str,
        operand: Operand,
    ) -> Result<ConstrainedValue<F, G>> {
        todo!()
    }
}
