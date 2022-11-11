use std::{cell::RefCell, fmt, rc::Rc};

use crate::{bls12_377::Fp, circuit::helpers::R1CS};

use super::{
    helpers::{Assignment, Constraint, LinearCombination, Mode, Variable},
    traits::Inject,
    Environment,
};

thread_local! {
  pub(super) static CIRCUIT: Rc<RefCell<R1CS>> = Rc::new(RefCell::new(R1CS::new()));
  pub(super) static IN_WITNESS: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Circuit;

impl Environment for Circuit {
    /// Returns the `zero` constant.
    fn zero() -> LinearCombination {
        LinearCombination::zero()
    }

    /// Returns the `one` constant.
    fn one() -> LinearCombination {
        LinearCombination::one()
    }

    /// Returns a new variable of the given mode and value.
    fn new_variable(mode: Mode, value: Fp) -> Variable {
        IN_WITNESS.with(|in_witness| {
            // Ensure we are not in witness mode.
            if !(*(**in_witness).borrow()) {
                CIRCUIT.with(|circuit| match mode {
                    Mode::Constant => (**circuit).borrow_mut().new_constant(value),
                    Mode::Public => (**circuit).borrow_mut().new_public(value),
                    Mode::Private => (**circuit).borrow_mut().new_private(value),
                })
            } else {
                Self::halt("Tried to initialize a new variable in witness mode")
            }
        })
    }

    /// Returns a new witness of the given mode and value.
    fn new_witness<Fn: FnOnce() -> Output::Primitive, Output: Inject>(
        mode: Mode,
        logic: Fn,
    ) -> Output {
        IN_WITNESS.with(|in_witness| {
            // Set the entire environment to witness mode.
            *(**in_witness).borrow_mut() = true;

            // Run the logic.
            let output = logic();

            // Return the entire environment from witness mode.
            *(**in_witness).borrow_mut() = false;

            Inject::new(mode, output)
        })
    }

    /// Enters a new scope for the environment.
    fn scope<S: Into<String>, Fn, Output>(name: S, logic: Fn) -> Output
    where
        Fn: FnOnce() -> Output,
    {
        IN_WITNESS.with(|in_witness| {
            // Ensure we are not in witness mode.
            if !(*(**in_witness).borrow()) {
                CIRCUIT.with(|circuit| {
                    // Set the entire environment to the new scope.
                    let name = name.into();
                    if let Err(error) = (**circuit).borrow_mut().push_scope(&name) {
                        Self::halt(error)
                    }

                    // Run the logic.
                    let output = logic();

                    // Return the entire environment to the previous scope.
                    if let Err(error) = (**circuit).borrow_mut().pop_scope(name) {
                        Self::halt(error)
                    }

                    output
                })
            } else {
                Self::halt("Tried to initialize a new scope in witness mode")
            }
        })
    }

    /// Adds one constraint enforcing that `(A * B) == C`.
    fn enforce<Fn, A, B, C>(constraint: Fn)
    where
        Fn: FnOnce() -> (A, B, C),
        A: Into<LinearCombination>,
        B: Into<LinearCombination>,
        C: Into<LinearCombination>,
    {
        IN_WITNESS.with(|in_witness| {
            // Ensure we are not in witness mode.
            if !(*(**in_witness).borrow()) {
                CIRCUIT.with(|circuit| {
                    let (a, b, c) = constraint();
                    let (a, b, c) = (a.into(), b.into(), c.into());

                    // Ensure the constraint is not comprised of constants.
                    match a.is_constant() && b.is_constant() && c.is_constant() {
                        true => {
                            // Evaluate the constant constraint.
                            assert_eq!(
                                a.value() * b.value(),
                                c.value(),
                                "Constant constraint failed: ({} * {}) =?= {}",
                                a,
                                b,
                                c
                            );

                            // match self.counter.scope().is_empty() {
                            //     true => println!("Enforced constraint with constant terms: ({} * {}) =?= {}", a, b, c),
                            //     false => println!(
                            //         "Enforced constraint with constant terms ({}): ({} * {}) =?= {}",
                            //         self.counter.scope(), a, b, c
                            //     ),
                            // }
                        }
                        false => {
                            // Construct the constraint object.
                            let constraint = Constraint((**circuit).borrow().scope(), a, b, c);
                            // Append the constraint.
                            (**circuit).borrow_mut().enforce(constraint)
                        }
                    }
                });
            }
        })
    }

    /// Returns `true` if all constraints in the environment are satisfied.
    fn is_satisfied() -> bool {
        CIRCUIT.with(|circuit| (**circuit).borrow().is_satisfied())
    }

    /// Returns `true` if all constraints in the current scope are satisfied.
    fn is_satisfied_in_scope() -> bool {
        CIRCUIT.with(|circuit| (**circuit).borrow().is_satisfied_in_scope())
    }

    /// Returns the number of constants in the entire circuit.
    fn num_constants() -> u64 {
        CIRCUIT.with(|circuit| (**circuit).borrow().num_constants())
    }

    /// Returns the number of public variables in the entire circuit.
    fn num_public() -> u64 {
        CIRCUIT.with(|circuit| (**circuit).borrow().num_public())
    }

    /// Returns the number of private variables in the entire circuit.
    fn num_private() -> u64 {
        CIRCUIT.with(|circuit| (**circuit).borrow().num_private())
    }

    /// Returns the number of constraints in the entire circuit.
    fn num_constraints() -> u64 {
        CIRCUIT.with(|circuit| (**circuit).borrow().num_constraints())
    }

    /// Returns the number of gates in the entire circuit.
    fn num_gates() -> u64 {
        CIRCUIT.with(|circuit| (**circuit).borrow().num_gates())
    }

    /// Returns the number of constants for the current scope.
    fn num_constants_in_scope() -> u64 {
        CIRCUIT.with(|circuit| (**circuit).borrow().num_constants_in_scope())
    }

    /// Returns the number of public variables for the current scope.
    fn num_public_in_scope() -> u64 {
        CIRCUIT.with(|circuit| (**circuit).borrow().num_public_in_scope())
    }

    /// Returns the number of private variables for the current scope.
    fn num_private_in_scope() -> u64 {
        CIRCUIT.with(|circuit| (**circuit).borrow().num_private_in_scope())
    }

    /// Returns the number of constraints for the current scope.
    fn num_constraints_in_scope() -> u64 {
        CIRCUIT.with(|circuit| (**circuit).borrow().num_constraints_in_scope())
    }

    /// Returns the number of gates for the current scope.
    fn num_gates_in_scope() -> u64 {
        CIRCUIT.with(|circuit| (**circuit).borrow().num_gates_in_scope())
    }

    /// Halts the program from further synthesis, evaluation, and execution in the current environment.
    fn halt<D: fmt::Display, T>(message: D) -> T {
        panic!("{message}")
    }

    /// Circuits should not have easy access to this during synthesis.
    /// Returns the R1CS circuit, resetting the circuit.
    fn inject_r1cs(r1cs: R1CS) {
        CIRCUIT.with(|circuit| {
            // Ensure the circuit is empty before injecting.
            assert_eq!(0, (**circuit).borrow().num_constants());
            assert_eq!(1, (**circuit).borrow().num_public());
            assert_eq!(0, (**circuit).borrow().num_private());
            assert_eq!(0, (**circuit).borrow().num_constraints());
            // Inject the R1CS instance.
            let r1cs = circuit.replace(r1cs);
            // Ensure the circuit that was replaced is empty.
            assert_eq!(0, r1cs.num_constants());
            assert_eq!(1, r1cs.num_public());
            assert_eq!(0, r1cs.num_private());
            assert_eq!(0, r1cs.num_constraints());
        })
    }

    /// Circuits should not have easy access to this during synthesis.
    /// Returns the R1CS circuit, resetting the circuit.
    fn eject_r1cs_and_reset() -> R1CS {
        CIRCUIT.with(|circuit| {
            // Eject the R1CS instance.
            let r1cs = circuit.replace(R1CS::new());
            // Ensure the circuit is now empty.
            assert_eq!(0, (**circuit).borrow().num_constants());
            assert_eq!(1, (**circuit).borrow().num_public());
            assert_eq!(0, (**circuit).borrow().num_private());
            assert_eq!(0, (**circuit).borrow().num_constraints());
            // Return the R1CS instance.
            r1cs
        })
    }

    /// Circuits should not have easy access to this during synthesis.
    /// Returns the R1CS assignment of the circuit, resetting the circuit.
    fn eject_assignment_and_reset() -> Assignment {
        CIRCUIT.with(|circuit| {
            // Eject the R1CS instance.
            let r1cs = circuit.replace(R1CS::new());
            assert_eq!(0, (**circuit).borrow().num_constants());
            assert_eq!(1, (**circuit).borrow().num_public());
            assert_eq!(0, (**circuit).borrow().num_private());
            assert_eq!(0, (**circuit).borrow().num_constraints());
            // Convert the R1CS instance to an assignment.
            Assignment::from(r1cs)
        })
    }

    /// Clears the circuit and initializes an empty environment.
    fn reset() {
        CIRCUIT.with(|circuit| {
            *(**circuit).borrow_mut() = R1CS::new();
            assert_eq!(0, (**circuit).borrow().num_constants());
            assert_eq!(1, (**circuit).borrow().num_public());
            assert_eq!(0, (**circuit).borrow().num_private());
            assert_eq!(0, (**circuit).borrow().num_constraints());
        });
    }
}

impl fmt::Display for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        CIRCUIT.with(|circuit| write!(f, "{}", (**circuit).borrow()))
    }
}
