use anyhow::Result;
use std::{fmt, rc::Rc};

use crate::bls12_377::{Field, Fp};

use super::{Constraint, Counter, Variable};

pub type Scope = String;

#[derive(Debug)]
pub struct R1CS {
    constants: Vec<Variable>,
    public: Vec<Variable>,
    private: Vec<Variable>,
    constraints: Vec<Constraint>,
    counter: Counter,
    gates: u64,
}

impl R1CS {
    /// Returns a new instance of a constraint system.
    pub(crate) fn new() -> Self {
        Self {
            constants: Default::default(),
            public: vec![Variable::Public(0u64, Rc::new(Fp::ONE))],
            private: Default::default(),
            constraints: Default::default(),
            counter: Default::default(),
            gates: 0,
        }
    }

    /// Appends the given scope to the current environment.
    pub(crate) fn push_scope<S: Into<String>>(&mut self, name: S) -> Result<()> {
        self.counter.push(name)
    }

    /// Removes the given scope from the current environment.
    pub(crate) fn pop_scope<S: Into<String>>(&mut self, name: S) -> Result<()> {
        self.counter.pop(name)
    }

    /// Returns a new constant with the given value and scope.
    pub(crate) fn new_constant(&mut self, value: Fp) -> Variable {
        let variable = Variable::Constant(Rc::new(value));
        self.constants.push(variable.clone());
        self.counter.increment_constant();
        variable
    }

    /// Returns a new public variable with the given value and scope.
    pub(crate) fn new_public(&mut self, value: Fp) -> Variable {
        let variable = Variable::Public(self.public.len() as u64, Rc::new(value));
        self.public.push(variable.clone());
        self.counter.increment_public();
        variable
    }

    /// Returns a new private variable with the given value and scope.
    pub(crate) fn new_private(&mut self, value: Fp) -> Variable {
        let variable = Variable::Private(self.private.len() as u64, Rc::new(value));
        self.private.push(variable.clone());
        self.counter.increment_private();
        variable
    }

    /// Adds one constraint enforcing that `(A * B) == C`.
    pub(crate) fn enforce(&mut self, constraint: Constraint) {
        self.gates += constraint.num_gates();
        self.constraints.push(constraint.clone());
        self.counter.add_constraint(constraint);
    }

    /// Returns `true` if all constraints in the environment are satisfied.
    pub(crate) fn is_satisfied(&self) -> bool {
        self.constraints
            .iter()
            .all(|constraint| constraint.is_satisfied())
    }

    /// Returns `true` if all constraints in the current scope are satisfied.
    pub(crate) fn is_satisfied_in_scope(&self) -> bool {
        self.counter.is_satisfied_in_scope()
    }

    /// Returns the current scope.
    pub(crate) fn scope(&self) -> Scope {
        self.counter.scope()
    }

    /// Returns the number of constants in the constraint system.
    pub(crate) fn num_constants(&self) -> u64 {
        self.constants.len() as u64
    }

    /// Returns the number of public variables in the constraint system.
    pub(crate) fn num_public(&self) -> u64 {
        self.public.len() as u64
    }

    /// Returns the number of private variables in the constraint system.
    pub(crate) fn num_private(&self) -> u64 {
        self.private.len() as u64
    }

    /// Returns the number of constraints in the constraint system.
    pub(crate) fn num_constraints(&self) -> u64 {
        self.constraints.len() as u64
    }

    /// Returns the number of gates in the constraint system.
    pub(crate) fn num_gates(&self) -> u64 {
        self.gates
    }

    /// Returns the number of constants for the current scope.
    pub(crate) fn num_constants_in_scope(&self) -> u64 {
        self.counter.num_constants_in_scope()
    }

    /// Returns the number of public variables for the current scope.
    pub(crate) fn num_public_in_scope(&self) -> u64 {
        self.counter.num_public_in_scope()
    }

    /// Returns the number of private variables for the current scope.
    pub(crate) fn num_private_in_scope(&self) -> u64 {
        self.counter.num_private_in_scope()
    }

    /// Returns the number of constraints for the current scope.
    pub(crate) fn num_constraints_in_scope(&self) -> u64 {
        self.counter.num_constraints_in_scope()
    }

    /// Returns the number of gates for the current scope.
    pub(crate) fn num_gates_in_scope(&self) -> u64 {
        self.counter.num_gates_in_scope()
    }

    /// Returns the public variables in the constraint system.
    pub(crate) fn to_public_variables(&self) -> &Vec<Variable> {
        &self.public
    }

    /// Returns the private variables in the constraint system.
    pub(crate) fn to_private_variables(&self) -> &Vec<Variable> {
        &self.private
    }

    /// Returns the constraints in the constraint system.
    pub(crate) fn to_constraints(&self) -> &Vec<Constraint> {
        &self.constraints
    }
}

impl fmt::Display for R1CS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::default();
        for constraint in self.to_constraints() {
            output += &constraint.to_string();
        }
        output += "\n";

        write!(f, "{}", output)
    }
}
