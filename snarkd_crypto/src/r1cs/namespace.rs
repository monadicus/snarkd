use crate::{r1cs::ConstraintSystem, LinearCombination, Variable};

use anyhow::Result;

/// This is a "namespaced" constraint system which borrows a constraint system
/// (pushing a namespace context) and, when dropped, pops out of the namespace context.
pub struct Namespace<'a, CS: ConstraintSystem>(pub(crate) &'a mut CS);

impl<CS: ConstraintSystem> ConstraintSystem for Namespace<'_, CS> {
    type Root = CS::Root;
    type Field = CS::Field;

    fn one() -> Variable {
        CS::one()
    }

    fn alloc<FN, A, AR>(&mut self, annotation: A, f: FN) -> Result<Variable>
    where
        FN: FnOnce() -> Result<Self::Field>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        self.0.alloc(annotation, f)
    }

    fn alloc_input<FN, A, AR>(&mut self, annotation: A, f: FN) -> Result<Variable>
    where
        FN: FnOnce() -> Result<Self::Field>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        self.0.alloc_input(annotation, f)
    }

    fn enforce<A, AR, LA, LB, LC>(&mut self, annotation: A, a: LA, b: LB, c: LC)
    where
        A: FnOnce() -> AR,
        AR: AsRef<str>,
        LA: FnOnce(LinearCombination<Self::Field>) -> LinearCombination<Self::Field>,
        LB: FnOnce(LinearCombination<Self::Field>) -> LinearCombination<Self::Field>,
        LC: FnOnce(LinearCombination<Self::Field>) -> LinearCombination<Self::Field>,
    {
        self.0.enforce(annotation, a, b, c)
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
        self.0.get_root()
    }

    fn num_constraints(&self) -> usize {
        self.0.num_constraints()
    }

    fn num_public_variables(&self) -> usize {
        self.0.num_public_variables()
    }

    fn num_private_variables(&self) -> usize {
        self.0.num_private_variables()
    }

    fn is_in_setup_mode(&self) -> bool {
        self.0.is_in_setup_mode()
    }
}

impl<CS: ConstraintSystem> Drop for Namespace<'_, CS> {
    fn drop(&mut self) {
        self.get_root().pop_namespace()
    }
}
