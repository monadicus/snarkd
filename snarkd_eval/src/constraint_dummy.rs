// TODO: DO NOT USE! TEMPORARY

use core::fmt;
use std::marker::PhantomData;

use snarkd_crypto::{Field, Parameters};
use snarkd_ir::Operand;

use crate::ConstrainedValue;

pub type SynthesisError = ();
pub type Variable = ();

pub struct LinearCombination<F>(PhantomData<F>);

pub fn bool_from_input<F: Field, G: Parameters, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: &str,
    value: Operand,
) -> Result<ConstrainedValue<F, G>, String> {
    todo!()
}

#[derive(Debug, Clone)]
pub struct ValueError(String);
impl<T: fmt::Display> From<T> for ValueError {
    fn from(v: T) -> Self {
        Self(v.to_string())
    }
}

pub struct Namespace<'a, F: Field, CS: ConstraintSystem<F>>(&'a mut CS, PhantomData<F>);

impl<'a, F: Field, CS: ConstraintSystem<F>> ConstraintSystem<F> for Namespace<'a, F, CS> {
    type Root = CS::Root;

    fn alloc<FN, A, AR>(&mut self, annotation: A, f: FN) -> Result<Variable, SynthesisError>
    where
        FN: FnOnce() -> Result<F, SynthesisError>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        todo!()
    }

    fn alloc_input<FN, A, AR>(&mut self, annotation: A, f: FN) -> Result<Variable, SynthesisError>
    where
        FN: FnOnce() -> Result<F, SynthesisError>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        todo!()
    }

    fn enforce<A, AR, LA, LB, LC>(&mut self, annotation: A, a: LA, b: LB, c: LC)
    where
        A: FnOnce() -> AR,
        AR: AsRef<str>,
        LA: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
        LB: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
        LC: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
    {
        todo!()
    }

    fn push_namespace<NR, N>(&mut self, name_fn: N)
    where
        NR: AsRef<str>,
        N: FnOnce() -> NR,
    {
        todo!()
    }

    fn pop_namespace(&mut self) {
        todo!()
    }

    fn get_root(&mut self) -> &mut Self::Root {
        todo!()
    }

    fn num_constraints(&self) -> usize {
        todo!()
    }

    fn num_public_variables(&self) -> usize {
        todo!()
    }

    fn num_private_variables(&self) -> usize {
        todo!()
    }

    fn is_in_setup_mode(&self) -> bool {
        todo!()
    }
}

/// Represents the index of either a public variable (input) or a private variable (auxiliary).
#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
pub enum Index {
    /// Index of an public variable.
    Public(usize),
    /// Index of an private variable.
    Private(usize),
}

pub trait ConstraintSystem<F: Field> {
    /// Represents the type of the "root" of this constraint system
    /// so that nested namespaces can minimize indirection.
    type Root: ConstraintSystem<F>;

    /// Return the "one" input variable
    fn one() -> Variable {
        // Variable::new_unchecked(Index::Public(0))
        todo!()
    }

    /// Allocate a private variable in the constraint system. The provided
    /// function is used to determine the assignment of the variable. The
    /// given `annotation` function is invoked in testing contexts in order
    /// to derive a unique name for this variable in the current namespace.
    fn alloc<FN, A, AR>(&mut self, annotation: A, f: FN) -> Result<Variable, SynthesisError>
    where
        FN: FnOnce() -> Result<F, SynthesisError>,
        A: FnOnce() -> AR,
        AR: AsRef<str>;

    /// Allocate a public variable in the constraint system. The provided
    /// function is used to determine the assignment of the variable.
    fn alloc_input<FN, A, AR>(&mut self, annotation: A, f: FN) -> Result<Variable, SynthesisError>
    where
        FN: FnOnce() -> Result<F, SynthesisError>,
        A: FnOnce() -> AR,
        AR: AsRef<str>;

    /// Enforce that `A` * `B` = `C`. The `annotation` function is invoked in
    /// testing contexts in order to derive a unique name for the constraint
    /// in the current namespace.
    fn enforce<A, AR, LA, LB, LC>(&mut self, annotation: A, a: LA, b: LB, c: LC)
    where
        A: FnOnce() -> AR,
        AR: AsRef<str>,
        LA: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
        LB: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
        LC: FnOnce(LinearCombination<F>) -> LinearCombination<F>;

    /// Create a new (sub)namespace and enter into it. Not intended
    /// for downstream use; use `namespace` instead.
    fn push_namespace<NR, N>(&mut self, name_fn: N)
    where
        NR: AsRef<str>,
        N: FnOnce() -> NR;

    /// Exit out of the existing namespace. Not intended for
    /// downstream use; use `namespace` instead.
    fn pop_namespace(&mut self);

    /// Gets the "root" constraint system, bypassing the namespacing.
    /// Not intended for downstream use; use `namespace` instead.
    fn get_root(&mut self) -> &mut Self::Root;

    /// Begin a namespace for this constraint system.
    fn ns<NR, N>(&mut self, name_fn: N) -> Namespace<'_, F, Self::Root>
    where
        NR: AsRef<str>,
        N: FnOnce() -> NR,
    {
        self.get_root().push_namespace(name_fn);

        Namespace(self.get_root(), PhantomData)
    }

    /// Output the number of constraints in the system.
    fn num_constraints(&self) -> usize;

    /// Output the number of public input variables to the system.
    fn num_public_variables(&self) -> usize;

    /// Output the number of private input variables to the system.
    fn num_private_variables(&self) -> usize;

    /// Output whether the constraint system is in the setup mode.
    fn is_in_setup_mode(&self) -> bool;
}
