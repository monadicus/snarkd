use crate::{assert_scope, circuit::traits::FromBits};

use super::*;

fn check_from_bits_le(
    name: &str,
    expected: bool,
    candidate: &Boolean,
    num_constants: u64,
    num_public: u64,
    num_private: u64,
    num_constraints: u64,
) {
    Circuit::scope(name, || {
        let candidate = Boolean::from_bits_le(&[(*candidate).clone()]);
        assert_eq!(expected, candidate.eject_value());
        assert_scope!(num_constants, num_public, num_private, num_constraints);
    });
    Circuit::reset();
}

fn check_from_bits_be(
    name: &str,
    expected: bool,
    candidate: &Boolean,
    num_constants: u64,
    num_public: u64,
    num_private: u64,
    num_constraints: u64,
) {
    Circuit::scope(name, || {
        let candidate = Boolean::from_bits_be(&[(*candidate).clone()]);
        assert_eq!(expected, candidate.eject_value());
        assert_scope!(num_constants, num_public, num_private, num_constraints);
    });
    Circuit::reset();
}

#[test]
fn test_from_bits_constant() {
    let candidate = Boolean::new(Mode::Constant, true);
    check_from_bits_le("Constant", true, &candidate, 0, 0, 0, 0);
    check_from_bits_be("Constant", true, &candidate, 0, 0, 0, 0);

    let candidate = Boolean::new(Mode::Constant, false);
    check_from_bits_le("Constant", false, &candidate, 0, 0, 0, 0);
    check_from_bits_be("Constant", false, &candidate, 0, 0, 0, 0);
}

#[test]
fn test_from_bits_public() {
    let candidate = Boolean::new(Mode::Public, true);
    check_from_bits_le("Public", true, &candidate, 0, 0, 0, 0);
    check_from_bits_be("Public", true, &candidate, 0, 0, 0, 0);

    let candidate = Boolean::new(Mode::Public, false);
    check_from_bits_le("Public", false, &candidate, 0, 0, 0, 0);
    check_from_bits_be("Public", false, &candidate, 0, 0, 0, 0);
}

#[test]
fn test_from_bits_private() {
    let candidate = Boolean::new(Mode::Private, true);
    check_from_bits_le("Private", true, &candidate, 0, 0, 0, 0);
    check_from_bits_be("Private", true, &candidate, 0, 0, 0, 0);

    let candidate = Boolean::new(Mode::Private, false);
    check_from_bits_le("Private", false, &candidate, 0, 0, 0, 0);
    check_from_bits_be("Private", false, &candidate, 0, 0, 0, 0);
}
