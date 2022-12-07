use itertools::Itertools;

use crate::{assert_scope, circuit::traits::ToBits};

use super::*;

fn check_to_bits_le(name: &str, expected: &[bool], candidate: &Boolean) {
    Circuit::scope(name, || {
        let candidate = candidate.to_bits_le();
        assert_eq!(1, candidate.len());
        for (expected_bit, candidate_bit) in expected.iter().zip_eq(candidate.iter()) {
            assert_eq!(*expected_bit, candidate_bit.eject_value());
        }
        assert_scope!(0, 0, 0, 0);
    });
}

fn check_to_bits_be(name: &str, expected: &[bool], candidate: Boolean) {
    Circuit::scope(name, || {
        let candidate = candidate.to_bits_be();
        assert_eq!(1, candidate.len());
        for (expected_bit, candidate_bit) in expected.iter().zip_eq(candidate.iter()) {
            assert_eq!(*expected_bit, candidate_bit.eject_value());
        }
        assert_scope!(0, 0, 0, 0);
    });
}

#[test]
fn test_to_bits_constant() {
    let candidate = Boolean::new(Mode::Constant, true);
    check_to_bits_le("Constant", &[true], &candidate);
    check_to_bits_be("Constant", &[true], candidate);

    let candidate = Boolean::new(Mode::Constant, false);
    check_to_bits_le("Constant", &[false], &candidate);
    check_to_bits_be("Constant", &[false], candidate);
}

#[test]
fn test_to_bits_public() {
    let candidate = Boolean::new(Mode::Public, true);
    check_to_bits_le("Public", &[true], &candidate);
    check_to_bits_be("Public", &[true], candidate);

    let candidate = Boolean::new(Mode::Public, false);
    check_to_bits_le("Public", &[false], &candidate);
    check_to_bits_be("Public", &[false], candidate);
}

#[test]
fn test_to_bits_private() {
    let candidate = Boolean::new(Mode::Private, true);
    check_to_bits_le("Private", &[true], &candidate);
    check_to_bits_be("Private", &[true], candidate);

    let candidate = Boolean::new(Mode::Private, false);
    check_to_bits_le("Private", &[false], &candidate);
    check_to_bits_be("Private", &[false], candidate);
}
