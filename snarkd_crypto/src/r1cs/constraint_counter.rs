use crate::{
    bls12_377::Fp,
    r1cs::{ConstraintSystem, LinearCombination, Variable},
};
use anyhow::Result;

use super::Index;

/// Constraint counter for testing purposes.
#[derive(Default)]
pub struct ConstraintCounter {
    pub num_public_variables: usize,
    pub num_private_variables: usize,
    pub num_constraints: usize,
}

impl ConstraintSystem for ConstraintCounter {
    type Root = Self;

    fn alloc<FN, A, AR>(&mut self, _: A, _: FN) -> Result<Variable>
    where
        FN: FnOnce() -> Result<Fp>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        let var = Variable::new_unchecked(Index::Private(self.num_private_variables));
        self.num_private_variables += 1;
        Ok(var)
    }

    fn alloc_input<FN, A, AR>(&mut self, _: A, _: FN) -> Result<Variable>
    where
        FN: FnOnce() -> Result<Fp>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        let var = Variable::new_unchecked(Index::Public(self.num_public_variables));
        self.num_public_variables += 1;

        Ok(var)
    }

    fn enforce<A, AR, LA, LB, LC>(&mut self, _: A, _: LA, _: LB, _: LC)
    where
        A: FnOnce() -> AR,
        AR: AsRef<str>,
        LA: FnOnce(LinearCombination<Fp>) -> LinearCombination<Fp>,
        LB: FnOnce(LinearCombination<Fp>) -> LinearCombination<Fp>,
        LC: FnOnce(LinearCombination<Fp>) -> LinearCombination<Fp>,
    {
        self.num_constraints += 1;
    }

    fn push_namespace<NR, N>(&mut self, _: N)
    where
        NR: AsRef<str>,
        N: FnOnce() -> NR,
    {
    }

    fn pop_namespace(&mut self) {}

    fn get_root(&mut self) -> &mut Self::Root {
        self
    }

    fn num_constraints(&self) -> usize {
        self.num_constraints
    }

    fn num_public_variables(&self) -> usize {
        self.num_public_variables
    }

    fn num_private_variables(&self) -> usize {
        self.num_private_variables
    }

    fn is_in_setup_mode(&self) -> bool {
        true
    }
}
