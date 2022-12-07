use crate::{
    bls12_377::{Field, Fp},
    circuit::{
        circuit::Circuit,
        helpers::Mode,
        traits::{Eject, Ternary},
        Environment,
    },
};

use super::Boolean;

impl Ternary for Boolean {
    /// Returns `first` if `condition` is `true`, otherwise returns `second`.
    fn ternary(condition: &Boolean, first: &Self, second: &Self) -> Self {
        // Constant `condition`
        if condition.is_constant() {
            match condition.eject_value() {
                true => first.clone(),
                false => second.clone(),
            }
        }
        // Constant `first`
        else if first.is_constant() {
            match first.eject_value() {
                true => condition | second,
                false => !condition & second,
            }
        }
        // Constant `second`
        else if second.is_constant() {
            match second.eject_value() {
                true => !condition | first,
                false => condition & first,
            }
        }
        // Variables
        else {
            // Compute the witness value, based on the condition.
            let witness = match condition.eject_value() {
                true => first.eject_value(),
                false => second.eject_value(),
            };

            // Declare a new variable with the expected output as witness.
            // Note: The constraint below will ensure `output` is either 0 or 1,
            // assuming `self` and `other` are well-formed (they are either 0 or 1).
            let output = Boolean(
                Circuit::new_variable(
                    Mode::Private,
                    match witness {
                        true => Fp::ONE,
                        false => Fp::ZERO,
                    },
                )
                .into(),
            );

            //
            // Ternary Enforcement
            // -------------------------------------------------------
            //    output = condition * a + (1 - condition) * b
            // => output = b + condition * (a - b)
            // => condition * (a - b) = output - b
            //
            // See `Field::ternary()` for the proof of correctness.
            //
            Circuit::enforce(|| (condition, (&first.0 - &second.0), (&output.0 - &second.0)));

            output
        }
    }
}
