use crate::{
    bls12_377::{Field as FieldTrait, Fp},
    circuit::{
        helpers::Mode,
        traits::{Eject, Inject},
        types::Field,
        Environment,
    },
};

use super::circuit::Circuit;

/// Compute 2^EXPONENT - 1, in a purposefully constraint-inefficient manner for testing.
fn create_example_circuit() -> Field {
    let one = Fp::ONE;
    let two = one + one;

    const EXPONENT: u64 = 64;

    // Compute 2^EXPONENT - 1, in a purposefully constraint-inefficient manner for testing.
    let mut candidate = Field::new(Mode::Public, one);
    let mut accumulator = Field::new(Mode::Private, two);
    for _ in 0..EXPONENT {
        candidate += &accumulator;
        accumulator *= Field::new(Mode::Private, two);
    }

    assert_eq!(
        (accumulator - Field::one()).eject_value(),
        candidate.eject_value()
    );
    assert_eq!(2, Circuit::num_public());
    assert_eq!(2 * EXPONENT + 1, Circuit::num_private());
    assert_eq!(EXPONENT, Circuit::num_constraints());
    assert!(Circuit::is_satisfied());

    candidate
}

#[test]
fn test_print_circuit() {
    let _candidate = create_example_circuit();
    let output = format!("{}", Circuit);
    println!("{}", output);
}

#[test]
fn test_circuit_scope() {
    Circuit::scope("test_circuit_scope", || {
        assert_eq!(0, Circuit::num_constants());
        assert_eq!(1, Circuit::num_public());
        assert_eq!(0, Circuit::num_private());
        assert_eq!(0, Circuit::num_constraints());

        assert_eq!(0, Circuit::num_constants_in_scope());
        assert_eq!(0, Circuit::num_public_in_scope());
        assert_eq!(0, Circuit::num_private_in_scope());
        assert_eq!(0, Circuit::num_constraints_in_scope());
    })
}
