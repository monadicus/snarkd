use std::marker::PhantomData;

use crate::{
    bls12_377::Field,
    r1cs::{ConstraintSystem, LinearCombination, Variable},
};

use anyhow::Result;

/// This is a "namespaced" constraint system which borrows a constraint system
/// (pushing a namespace context) and, when dropped, pops out of the namespace context.
pub struct Namespace<'a, F: Field, CS: ConstraintSystem<F>> {
    pub(crate) cs: &'a mut CS,
    _pd: PhantomData<F>,
}

impl<'a, F: Field, CS: ConstraintSystem<F>> Namespace<'a, F, CS> {
    pub fn new(cs: &'a mut CS) -> Self {
        Self {
            cs,
            _pd: PhantomData,
        }
    }
}

impl<F: Field, CS: ConstraintSystem<F>> ConstraintSystem<F> for Namespace<'_, F, CS> {
    type Root = CS::Root;

    fn one() -> Variable {
        CS::one()
    }

    fn alloc<FN, A, AR>(&mut self, annotation: A, f: FN) -> Result<Variable>
    where
        FN: FnOnce() -> Result<F>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        self.cs.alloc(annotation, f)
    }

    fn alloc_input<FN, A, AR>(&mut self, annotation: A, f: FN) -> Result<Variable>
    where
        FN: FnOnce() -> Result<F>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        self.cs.alloc_input(annotation, f)
    }

    fn enforce<A, AR, LA, LB, LC>(&mut self, annotation: A, a: LA, b: LB, c: LC)
    where
        A: FnOnce() -> AR,
        AR: AsRef<str>,
        LA: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
        LB: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
        LC: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
    {
        self.cs.enforce(annotation, a, b, c)
    }

    // Downstream users who use `namespace` will never interact with these
    // functions and they will never be invoked because the namespace is
    // never a root constraint system.
    fn push_namespace<NR, N>(&mut self, _: N)
    where
        NR: AsRef<str>,
        N: FnOnce() -> NR,
    {
        panic!("only the root's push_namespace should be called");
    }

    fn pop_namespace(&mut self) {
        panic!("only the root's pop_namespace should be called");
    }

    fn get_root(&mut self) -> &mut Self::Root {
        self.cs.get_root()
    }

    fn num_constraints(&self) -> usize {
        self.cs.num_constraints()
    }

    fn num_public_variables(&self) -> usize {
        self.cs.num_public_variables()
    }

    fn num_private_variables(&self) -> usize {
        self.cs.num_private_variables()
    }

    fn is_in_setup_mode(&self) -> bool {
        self.cs.is_in_setup_mode()
    }
}

impl<F: Field, CS: ConstraintSystem<F>> Drop for Namespace<'_, F, CS> {
    fn drop(&mut self) {
        self.get_root().pop_namespace()
    }
}
