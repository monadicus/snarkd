// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use crate::snark::marlin::ahp::matrices::{make_matrices_square, padded_matrix_dim, to_matrix_helper};
use snarkvm_fields::Field;
use snarkvm_r1cs::{errors::SynthesisError, ConstraintSystem as CS, Index as VarIndex, LinearCombination, Variable};
use snarkvm_utilities::serialize::*;

/// Stores constraints during index generation.
pub(crate) struct ConstraintSystem<F: Field> {
    pub(crate) a: Vec<Vec<(F, VarIndex)>>,
    pub(crate) b: Vec<Vec<(F, VarIndex)>>,
    pub(crate) c: Vec<Vec<(F, VarIndex)>>,
    pub(crate) num_public_variables: usize,
    pub(crate) num_private_variables: usize,
    pub(crate) num_constraints: usize,
}

impl<F: Field> ConstraintSystem<F> {
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
    pub(crate) fn a_matrix(&self) -> Vec<Vec<(F, usize)>> {
        to_matrix_helper(&self.a, self.num_public_variables)
    }

    #[inline]
    pub(crate) fn b_matrix(&self) -> Vec<Vec<(F, usize)>> {
        to_matrix_helper(&self.b, self.num_public_variables)
    }

    #[inline]
    pub(crate) fn c_matrix(&self) -> Vec<Vec<(F, usize)>> {
        to_matrix_helper(&self.c, self.num_public_variables)
    }

    #[inline]
    pub(crate) fn make_matrices_square(&mut self) {
        let num_variables = self.num_public_variables + self.num_private_variables;
        let matrix_dim = padded_matrix_dim(num_variables, self.num_constraints);
        make_matrices_square(self, num_variables);
        assert_eq!(self.num_public_variables + self.num_private_variables, self.num_constraints, "padding failed!");
        assert_eq!(
            self.num_public_variables + self.num_private_variables,
            matrix_dim,
            "padding does not result in expected matrix size!"
        );
    }

    #[inline]
    fn make_row(l: &LinearCombination<F>) -> Vec<(F, VarIndex)> {
        l.as_ref().iter().map(|(var, coeff)| (*coeff, var.get_unchecked())).collect()
    }
}

impl<F: Field> CS<F> for ConstraintSystem<F> {
    type Root = Self;

    #[inline]
    fn alloc<Fn, A, AR>(&mut self, _: A, _: Fn) -> Result<Variable, SynthesisError>
    where
        Fn: FnOnce() -> Result<F, SynthesisError>,
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
    fn alloc_input<Fn, A, AR>(&mut self, _: A, _: Fn) -> Result<Variable, SynthesisError>
    where
        Fn: FnOnce() -> Result<F, SynthesisError>,
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
        LA: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
        LB: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
        LC: FnOnce(LinearCombination<F>) -> LinearCombination<F>,
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
