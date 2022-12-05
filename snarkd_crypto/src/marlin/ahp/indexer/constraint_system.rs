use crate::{
    bls12_377::Scalar,
    marlin::ahp::matrices::{make_matrices_square, padded_matrix_dim, to_matrix_helper},
    r1cs::{ConstraintSystem as CS, Index as VarIndex, LinearCombination, Variable},
};
use anyhow::Result;

/// Stores constraints during index generation.
pub(crate) struct ConstraintSystem {
    pub(crate) a: Vec<Vec<(Scalar, VarIndex)>>,
    pub(crate) b: Vec<Vec<(Scalar, VarIndex)>>,
    pub(crate) c: Vec<Vec<(Scalar, VarIndex)>>,
    pub(crate) num_public_variables: usize,
    pub(crate) num_private_variables: usize,
    pub(crate) num_constraints: usize,
}

impl ConstraintSystem {
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            a: Vec::new(),
            b: Vec::new(),
            c: Vec::new(),
            num_public_variables: 1,
            num_private_variables: 0,
            num_constraints: 0,
        }
    }

    #[inline]
    pub(crate) fn a_matrix(&self) -> Vec<Vec<(Scalar, usize)>> {
        to_matrix_helper(&self.a, self.num_public_variables)
    }

    #[inline]
    pub(crate) fn b_matrix(&self) -> Vec<Vec<(Scalar, usize)>> {
        to_matrix_helper(&self.b, self.num_public_variables)
    }

    #[inline]
    pub(crate) fn c_matrix(&self) -> Vec<Vec<(Scalar, usize)>> {
        to_matrix_helper(&self.c, self.num_public_variables)
    }

    #[inline]
    pub(crate) fn make_matrices_square(&mut self) {
        let num_variables = self.num_public_variables + self.num_private_variables;
        let matrix_dim = padded_matrix_dim(num_variables, self.num_constraints);
        make_matrices_square(self, num_variables);
        assert_eq!(
            self.num_public_variables + self.num_private_variables,
            self.num_constraints,
            "padding failed!"
        );
        assert_eq!(
            self.num_public_variables + self.num_private_variables,
            matrix_dim,
            "padding does not result in expected matrix size!"
        );
    }

    #[inline]
    fn make_row(l: &LinearCombination<Scalar>) -> Vec<(Scalar, VarIndex)> {
        l.as_ref()
            .iter()
            .map(|(var, coeff)| (*coeff, var.get_unchecked()))
            .collect()
    }
}

impl CS for ConstraintSystem {
    type Root = Self;
    type Field = Scalar;

    #[inline]
    fn alloc<Fn, A, AR>(&mut self, _: A, _: Fn) -> Result<Variable>
    where
        Fn: FnOnce() -> Result<Scalar>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        // There is no assignment, so we don't invoke the
        // function for obtaining one.

        let index = self.num_private_variables;
        self.num_private_variables += 1;

        Ok(Variable::new_unchecked(VarIndex::Private(index)))
    }

    #[inline]
    fn alloc_input<Fn, A, AR>(&mut self, _: A, _: Fn) -> Result<Variable>
    where
        Fn: FnOnce() -> Result<Scalar>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        // There is no assignment, so we don't invoke the
        // function for obtaining one.

        let index = self.num_public_variables;
        self.num_public_variables += 1;

        Ok(Variable::new_unchecked(VarIndex::Public(index)))
    }

    fn enforce<A, AR, LA, LB, LC>(&mut self, _: A, a: LA, b: LB, c: LC)
    where
        A: FnOnce() -> AR,
        AR: AsRef<str>,
        LA: FnOnce(LinearCombination<Scalar>) -> LinearCombination<Scalar>,
        LB: FnOnce(LinearCombination<Scalar>) -> LinearCombination<Scalar>,
        LC: FnOnce(LinearCombination<Scalar>) -> LinearCombination<Scalar>,
    {
        self.a.push(Self::make_row(&a(LinearCombination::zero())));
        self.b.push(Self::make_row(&b(LinearCombination::zero())));
        self.c.push(Self::make_row(&c(LinearCombination::zero())));

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
        true
    }
}
