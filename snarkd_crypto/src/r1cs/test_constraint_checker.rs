use anyhow::Result;

use crate::{ConstraintSystem, Field, Index, LinearCombination, Variable};

/// Constraint system for testing purposes.
pub struct TestConstraintChecker<F: Field> {
    // the list of currently applicable input variables
    public_variables: Vec<F>,
    // the list of currently applicable auxiliary variables
    private_variables: Vec<F>,
    // whether or not unsatisfactory constraint has been found
    found_unsatisfactory_constraint: bool,
    // number of constraints
    num_constraints: usize,
    // constraint path segments in the stack
    segments: Vec<String>,
    // the first unsatisfied constraint
    first_unsatisfied_constraint: Option<String>,
}

impl<F: Field> Default for TestConstraintChecker<F> {
    fn default() -> Self {
        Self {
            public_variables: vec![F::ONE],
            private_variables: vec![],
            found_unsatisfactory_constraint: false,
            num_constraints: 0,
            segments: vec![],
            first_unsatisfied_constraint: None,
        }
    }
}

impl<F: Field> TestConstraintChecker<F> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn which_is_unsatisfied(&self) -> Option<String> {
        self.first_unsatisfied_constraint.clone()
    }

    #[inline]
    pub fn is_satisfied(&self) -> bool {
        !self.found_unsatisfactory_constraint
    }

    #[inline]
    pub fn num_constraints(&self) -> usize {
        self.num_constraints
    }

    #[inline]
    pub fn public_inputs(&self) -> Vec<F> {
        self.public_variables[1..].to_vec()
    }
}

impl<F: Field> ConstraintSystem for TestConstraintChecker<F> {
    type Root = Self;
    type Field = F;

    fn alloc<Fn, A, AR>(&mut self, _annotation: A, f: Fn) -> Result<Variable>
    where
        Fn: FnOnce() -> Result<Self::Field>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        let index = self.private_variables.len();
        self.private_variables.push(f()?);
        let var = Variable::new_unchecked(Index::Private(index));

        Ok(var)
    }

    fn alloc_input<Fn, A, AR>(&mut self, _annotation: A, f: Fn) -> Result<Variable>
    where
        Fn: FnOnce() -> Result<Self::Field>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        let index = self.public_variables.len();
        self.public_variables.push(f()?);
        let var = Variable::new_unchecked(Index::Public(index));

        Ok(var)
    }

    fn enforce<A, AR, LA, LB, LC>(&mut self, annotation: A, a: LA, b: LB, c: LC)
    where
        A: FnOnce() -> AR,
        AR: AsRef<str>,
        LA: FnOnce(LinearCombination<Self::Field>) -> LinearCombination<Self::Field>,
        LB: FnOnce(LinearCombination<Self::Field>) -> LinearCombination<Self::Field>,
        LC: FnOnce(LinearCombination<Self::Field>) -> LinearCombination<Self::Field>,
    {
        self.num_constraints += 1;

        let eval_lc = |lc: Vec<(Variable, F)>| -> F {
            lc.into_iter()
                .map(|(var, coeff)| {
                    let value = match var.get_unchecked() {
                        Index::Public(index) => self.public_variables[index],
                        Index::Private(index) => self.private_variables[index],
                    };
                    value * coeff
                })
                .sum::<Self::Field>()
        };

        let a = eval_lc(a(LinearCombination::zero()).0);
        let b = eval_lc(b(LinearCombination::zero()).0);
        let c = eval_lc(c(LinearCombination::zero()).0);

        if a * b != c && self.first_unsatisfied_constraint.is_none() {
            self.found_unsatisfactory_constraint = true;

            let new = annotation().as_ref().to_string();
            assert!(!new.contains('/'), "'/' is not allowed in names");

            let mut path = self.segments.clone();
            path.push(new);
            self.first_unsatisfied_constraint = Some(path.join("/"));
        }
    }

    fn push_namespace<NR: AsRef<str>, N: FnOnce() -> NR>(&mut self, name_fn: N) {
        let new = name_fn().as_ref().to_string();
        assert!(!new.contains('/'), "'/' is not allowed in names");

        self.segments.push(new)
    }

    fn pop_namespace(&mut self) {
        self.segments.pop();
    }

    #[inline]
    fn get_root(&mut self) -> &mut Self::Root {
        self
    }

    #[inline]
    fn num_constraints(&self) -> usize {
        self.num_constraints()
    }

    #[inline]
    fn num_public_variables(&self) -> usize {
        self.public_variables.len()
    }

    #[inline]
    fn num_private_variables(&self) -> usize {
        self.private_variables.len()
    }

    #[inline]
    fn is_in_setup_mode(&self) -> bool {
        false
    }
}
