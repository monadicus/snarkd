use crate::{
    bls12_377::{Field, Scalar},
    marlin::ahp::matrices::make_matrices_square,
    r1cs::{ConstraintSystem as CS, Index as VarIndex, LinearCombination, Variable},
};
use anyhow::Result;

pub(crate) struct ConstraintSystem {
    pub(crate) public_variables: Vec<Scalar>,
    pub(crate) private_variables: Vec<Scalar>,
    pub(crate) num_public_variables: usize,
    pub(crate) num_private_variables: usize,
    pub(crate) num_constraints: usize,
}

impl ConstraintSystem {
    pub(crate) fn new() -> Self {
        Self {
            public_variables: vec![Scalar::ONE],
            private_variables: Vec::new(),
            num_public_variables: 1usize,
            num_private_variables: 0usize,
            num_constraints: 0usize,
        }
    }

    /// Formats the public input according to the requirements of the constraint
    /// system
    pub(crate) fn format_public_input(public_input: &[Scalar]) -> Vec<Scalar> {
        let mut input = vec![Scalar::ONE];
        input.extend_from_slice(public_input);
        input
    }

    /// Takes in a previously formatted public input and removes the formatting
    /// imposed by the constraint system.
    pub(crate) fn unformat_public_input(input: &[Scalar]) -> Vec<Scalar> {
        input[1..].to_vec()
    }

    pub(crate) fn make_matrices_square(&mut self) {
        let num_variables = self.num_public_variables + self.num_private_variables;
        make_matrices_square(self, num_variables);
        assert_eq!(
            self.num_public_variables + self.num_private_variables,
            self.num_constraints,
            "padding failed!"
        );
    }
}

impl CS for ConstraintSystem {
    type Root = Self;
    type Field = Scalar;

    #[inline]
    fn alloc<Fn, A, AR>(&mut self, _: A, f: Fn) -> Result<Variable>
    where
        Fn: FnOnce() -> Result<Self::Field>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        let index = self.num_private_variables;
        self.num_private_variables += 1;

        self.private_variables.push(f()?);
        Ok(Variable::new_unchecked(VarIndex::Private(index)))
    }

    #[inline]
    fn alloc_input<Fn, A, AR>(&mut self, _: A, f: Fn) -> Result<Variable>
    where
        Fn: FnOnce() -> Result<Self::Field>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        let index = self.num_public_variables;
        self.num_public_variables += 1;

        self.public_variables.push(f()?);
        Ok(Variable::new_unchecked(VarIndex::Public(index)))
    }

    #[inline]
    fn enforce<A, AR, LA, LB, LC>(&mut self, _: A, _: LA, _: LB, _: LC)
    where
        A: FnOnce() -> AR,
        AR: AsRef<str>,
        LA: FnOnce(LinearCombination<Self::Field>) -> LinearCombination<Self::Field>,
        LB: FnOnce(LinearCombination<Self::Field>) -> LinearCombination<Self::Field>,
        LC: FnOnce(LinearCombination<Self::Field>) -> LinearCombination<Self::Field>,
    {
        self.num_constraints += 1;
    }

    fn push_namespace<NR, N>(&mut self, _: N)
    where
        NR: AsRef<str>,
        N: FnOnce() -> NR,
    {
        // Do nothing; we don't care about namespaces in this context.
    }

    fn pop_namespace(&mut self) {
        // Do nothing; we don't care about namespaces in this context.
    }

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
        false
    }
}
