use std::marker::PhantomData;

use crate::{ConstraintSystem, Field, Index, LinearCombination, Variable};
use anyhow::Result;

/// Constraint counter for testing purposes.
#[derive(Default)]
pub struct ConstraintCounter<F: Field> {
    pub num_public_variables: usize,
    pub num_private_variables: usize,
    pub num_constraints: usize,
    // TODO see if we can avoid
    _pd: PhantomData<F>,
}

impl<F: Field> ConstraintSystem for ConstraintCounter<F> {
    type Root = Self;
    type Field = F;

    fn alloc<FN, A, AR>(&mut self, _: A, _: FN) -> Result<Variable>
    where
        FN: FnOnce() -> Result<Self::Field>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        let var = Variable::new_unchecked(Index::Private(self.num_private_variables));
        self.num_private_variables += 1;
        Ok(var)
    }

    fn alloc_input<FN, A, AR>(&mut self, _: A, _: FN) -> Result<Variable>
    where
        FN: FnOnce() -> Result<Self::Field>,
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
        LA: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
        LB: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
        LC: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
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
