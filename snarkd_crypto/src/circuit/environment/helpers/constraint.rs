use std::fmt;

use super::{LinearCombination, Scope};

#[derive(Clone, Debug)]
pub(crate) struct Constraint(
    pub(crate) Scope,
    pub(crate) LinearCombination,
    pub(crate) LinearCombination,
    pub(crate) LinearCombination,
);

impl Constraint {
    /// Returns the number of gates consumed by this constraint.
    pub(crate) fn num_gates(&self) -> u64 {
        let (a, b, c) = (&self.1, &self.2, &self.3);
        1 + a.num_additions() + b.num_additions() + c.num_additions()
    }

    /// Returns `true` if the constraint is satisfied.
    pub(crate) fn is_satisfied(&self) -> bool {
        let (scope, a, b, c) = (&self.0, &self.1, &self.2, &self.3);
        let a = a.value();
        let b = b.value();
        let c = c.value();

        match a * b == c {
            true => true,
            false => {
                eprintln!("Failed constraint at {scope}:\n\t({a} * {b}) != {c}");
                false
            }
        }
    }

    /// Returns a reference to the terms `(a, b, c)`.
    pub(crate) fn to_terms(&self) -> (&LinearCombination, &LinearCombination, &LinearCombination) {
        (&self.1, &self.2, &self.3)
    }
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (scope, a, b, c) = (&self.0, &self.1, &self.2, &self.3);
        let a = a.value();
        let b = b.value();
        let c = c.value();

        match (a * b) == c {
            true => write!(f, "Constraint {scope}:\n\t{a} * {b} == {c}\n"),
            false => write!(f, "Constraint {scope}:\n\t{a} * {b} != {c} (Unsatisfied)\n"),
        }
    }
}
