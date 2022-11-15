use std::ops::Not;

use crate::{
    assert_count, assert_output_mode,
    circuit::{
        circuit::Circuit,
        helpers::Mode,
        traits::{Eject, Inject},
        types::Boolean,
        Environment,
    },
};

fn check_not(name: &str, expected: bool, candidate_input: Boolean) {
    Circuit::scope(name, || {
        let mode = candidate_input.mode();
        let candidate_output = !candidate_input;
        assert_eq!(expected, candidate_output.eject_value());
        assert_count!(Not(Boolean) => Boolean, &mode);
        assert_output_mode!(Not(Boolean) => Boolean, &mode, candidate_output);
    });
}

#[test]
fn test_not_constant() {
    // NOT false
    let expected = true;
    let candidate_input = Boolean::new(Mode::Constant, false);
    check_not("NOT false", expected, candidate_input);

    // NOT true
    let expected = false;
    let candidate_input = Boolean::new(Mode::Constant, true);
    check_not("NOT true", expected, candidate_input);
}

#[test]
fn test_not_public() {
    // NOT false
    let expected = true;
    let candidate_input = Boolean::new(Mode::Public, false);
    check_not("NOT false", expected, candidate_input);

    // NOT true
    let expected = false;
    let candidate_input = Boolean::new(Mode::Public, true);
    check_not("NOT true", expected, candidate_input);
}

#[test]
fn test_not_private() {
    // NOT false
    let expected = true;
    let candidate_input = Boolean::new(Mode::Private, false);
    check_not("NOT false", expected, candidate_input);

    // NOT true
    let expected = false;
    let candidate_input = Boolean::new(Mode::Private, true);
    check_not("NOT true", expected, candidate_input);
}
