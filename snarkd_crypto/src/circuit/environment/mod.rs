pub mod helpers;
pub mod traits;

use core::{fmt, hash};

use crate::bls12_377::{Field, Fp};

use self::{
    helpers::{Assignment, LinearCombination, Mode, Variable, R1CS},
    traits::Inject,
};

pub trait Environment:
    'static + Copy + Clone + fmt::Debug + fmt::Display + Eq + PartialEq + hash::Hash
{
    /// Returns the `zero` constant.
    fn zero() -> LinearCombination;

    /// Returns the `one` constant.
    fn one() -> LinearCombination;

    /// Returns a new variable of the given mode and value.
    fn new_variable(mode: Mode, value: Fp) -> Variable;

    /// Returns a new witness of the given mode and value.
    fn new_witness<Fn: FnOnce() -> Output::Primitive, Output: Inject>(
        mode: Mode,
        value: Fn,
    ) -> Output;

    /// Enters a new scope for the environment.
    fn scope<S: Into<String>, Fn, Output>(name: S, logic: Fn) -> Output
    where
        Fn: FnOnce() -> Output;

    /// Adds one constraint enforcing that `(A * B) == C`.
    fn enforce<Fn, A, B, C>(constraint: Fn)
    where
        Fn: FnOnce() -> (A, B, C),
        A: Into<LinearCombination>,
        B: Into<LinearCombination>,
        C: Into<LinearCombination>;

    /// Adds one constraint enforcing that the given boolean is `true`.
    fn assert<Boolean: Into<LinearCombination>>(boolean: Boolean) {
        Self::enforce(|| (boolean, Self::one(), Self::one()))
    }

    /// Adds one constraint enforcing that the `A == B`.
    fn assert_eq<A, B>(a: A, b: B)
    where
        A: Into<LinearCombination>,
        B: Into<LinearCombination>,
    {
        Self::enforce(|| (a, Self::one(), b))
    }

    /// Adds one constraint enforcing that the `A != B`.
    fn assert_neq<A, B>(a: A, b: B)
    where
        A: Into<LinearCombination>,
        B: Into<LinearCombination>,
    {
        let (a, b) = (a.into(), b.into());
        let mode = Mode::witness_mode([&a, &b]);

        // Compute `(a - b)`.
        let a_minus_b = a - b;

        // Compute `multiplier` as `1 / (a - b)`.
        let multiplier = if let Some(inverse) = a_minus_b.value().inverse() {
            Self::new_variable(mode, inverse).into()
        } else {
            Self::zero()
        };

        // Enforce `(a - b) * multiplier == 1`.
        Self::enforce(|| (a_minus_b, multiplier, Self::one()));
    }

    /// Returns `true` if all constraints in the environment are satisfied.
    fn is_satisfied() -> bool;

    /// Returns `true` if all constraints in the current scope are satisfied.
    fn is_satisfied_in_scope() -> bool;

    /// Returns the number of constants in the entire environment.
    fn num_constants() -> u64;

    /// Returns the number of public variables in the entire environment.
    fn num_public() -> u64;

    /// Returns the number of private variables in the entire environment.
    fn num_private() -> u64;

    /// Returns the number of constraints in the entire environment.
    fn num_constraints() -> u64;

    /// Returns the number of gates in the entire environment.
    fn num_gates() -> u64;

    /// Returns a tuple containing the number of constants, public variables, private variables, constraints, and gates in the entire environment.
    fn count() -> (u64, u64, u64, u64, u64) {
        (
            Self::num_constants(),
            Self::num_public(),
            Self::num_private(),
            Self::num_constraints(),
            Self::num_gates(),
        )
    }

    /// Returns the number of constants for the current scope.
    fn num_constants_in_scope() -> u64;

    /// Returns the number of public variables for the current scope.
    fn num_public_in_scope() -> u64;

    /// Returns the number of private variables for the current scope.
    fn num_private_in_scope() -> u64;

    /// Returns the number of constraints for the current scope.
    fn num_constraints_in_scope() -> u64;

    /// Returns the number of gates for the current scope.
    fn num_gates_in_scope() -> u64;

    /// Returns a tuple containing the number of constants, public variables, private variables, constraints, and gates for the current scope.
    fn count_in_scope() -> (u64, u64, u64, u64, u64) {
        (
            Self::num_constants_in_scope(),
            Self::num_public_in_scope(),
            Self::num_private_in_scope(),
            Self::num_constraints_in_scope(),
            Self::num_gates_in_scope(),
        )
    }

    /// Halts the program from further synthesis, evaluation, and execution in the current environment.
    fn halt<D: fmt::Display, T>(message: D) -> T {
        panic!("{message}")
    }

    /// Returns the R1CS circuit, resetting the circuit.
    fn inject_r1cs(r1cs: R1CS);

    /// Returns the R1CS circuit, resetting the circuit.
    fn eject_r1cs_and_reset() -> R1CS;

    // TODO
    /// Returns the R1CS assignment of the circuit, resetting the circuit.
    fn eject_assignment_and_reset() -> Assignment;

    /// Clears and initializes an empty environment.
    fn reset();
}
