use std::ops::Sub;

use rayon::prelude::*;

use crate::{
    assert_count, assert_output_mode,
    bls12_377::Field as FieldTrait,
    circuit::{
        helpers::CircuitType,
        rng_test_struct::{TestRng, Uniform},
    },
};

use super::*;

const ITERATIONS: u64 = 10_000;

fn check_sub(name: &str, expected: &Fp, a: &Field, b: &Field) {
    Circuit::scope(name, || {
        let candidate = a - b;
        assert_eq!(
            *expected,
            candidate.eject_value(),
            "({} - {})",
            a.eject_value(),
            b.eject_value()
        );
        assert_count!(Sub(Field, Field) => Field, &(a.eject_mode(), b.eject_mode()));
        assert_output_mode!(Sub(Field, Field) => Field, &(CircuitType::from(a), CircuitType::from(b)), candidate);
    });
}

fn check_sub_assign(name: &str, expected: &Fp, a: &Field, b: &Field) {
    Circuit::scope(name, || {
        let mut candidate = a.clone();
        candidate -= b;
        assert_eq!(
            *expected,
            candidate.eject_value(),
            "({} - {})",
            a.eject_value(),
            b.eject_value()
        );
        assert_count!(Sub(Field, Field) => Field, &(a.eject_mode(), b.eject_mode()));
        assert_output_mode!(Sub(Field, Field) => Field, &(CircuitType::from(a), CircuitType::from(b)), candidate);
    });
}

fn run_test(mode_a: Mode, mode_b: Mode) {
    (0..ITERATIONS).into_par_iter().for_each(|i| {
        let mut rng = TestRng::default();
        let first = Uniform::rand(&mut rng);
        let second = Uniform::rand(&mut rng);

        let expected = first - second;
        let a = Field::new(mode_a, first);
        let b = Field::new(mode_b, second);

        let name = format!("Sub: a - b {}", i);
        check_sub(&name, &expected, &a, &b);
        let name = format!("SubAssign: a - b {}", i);
        check_sub_assign(&name, &expected, &a, &b);

        // Test identity.
        let name = format!("Sub: a - 0 {}", i);
        let zero = Field::new(mode_b, Fp::ZERO);
        check_sub(&name, &first, &a, &zero);
        let name = format!("SubAssign: a - 0 {}", i);
        check_sub_assign(&name, &first, &a, &zero);

        // Test negation.
        let name = format!("Sub: 0 - b {}", i);
        let zero = Field::new(mode_a, Fp::ZERO);
        check_sub(&name, &(-second), &zero, &b);
        let name = format!("SubAssign: 0 - b {}", i);
        check_sub_assign(&name, &(-second), &zero, &b);
    });
}

#[test]
fn test_constant_plus_constant() {
    run_test(Mode::Constant, Mode::Constant);
}

#[test]
fn test_constant_plus_public() {
    run_test(Mode::Constant, Mode::Public);
}

#[test]
fn test_public_plus_constant() {
    run_test(Mode::Public, Mode::Constant);
}

#[test]
fn test_constant_plus_private() {
    run_test(Mode::Constant, Mode::Private);
}

#[test]
fn test_private_plus_constant() {
    run_test(Mode::Private, Mode::Constant);
}

#[test]
fn test_public_plus_public() {
    run_test(Mode::Public, Mode::Public);
}

#[test]
fn test_public_plus_private() {
    run_test(Mode::Public, Mode::Private);
}

#[test]
fn test_private_plus_public() {
    run_test(Mode::Private, Mode::Public);
}

#[test]
fn test_private_plus_private() {
    run_test(Mode::Private, Mode::Private);
}

#[test]
fn test_0_minus_0() {
    let zero = Fp::ZERO;

    let candidate = Field::zero() - Field::zero();
    assert_eq!(zero, candidate.eject_value());

    let candidate = Field::zero() - &Field::zero();
    assert_eq!(zero, candidate.eject_value());

    let candidate = Field::new(Mode::Public, zero) - Field::new(Mode::Public, zero);
    assert_eq!(zero, candidate.eject_value());

    let candidate = Field::new(Mode::Public, zero) - Field::new(Mode::Private, zero);
    assert_eq!(zero, candidate.eject_value());

    let candidate = Field::new(Mode::Private, zero) - Field::new(Mode::Private, zero);
    assert_eq!(zero, candidate.eject_value());
}

#[test]
fn test_1_minus_0() {
    let zero = Fp::ZERO;
    let one = Fp::ONE;

    let candidate = Field::one() - Field::zero();
    assert_eq!(one, candidate.eject_value());

    let candidate = Field::one() - &Field::zero();
    assert_eq!(one, candidate.eject_value());

    let candidate = Field::new(Mode::Public, one) - Field::new(Mode::Public, zero);
    assert_eq!(one, candidate.eject_value());

    let candidate = Field::new(Mode::Public, one) - Field::new(Mode::Private, zero);
    assert_eq!(one, candidate.eject_value());

    let candidate = Field::new(Mode::Private, one) - Field::new(Mode::Private, zero);
    assert_eq!(one, candidate.eject_value());
}

#[test]
fn test_1_minus_1() {
    let zero = Fp::ZERO;
    let one = Fp::ONE;

    let candidate = Field::one() - Field::one();
    assert_eq!(zero, candidate.eject_value());

    let candidate = Field::one() - &Field::one();
    assert_eq!(zero, candidate.eject_value());

    let candidate = Field::new(Mode::Public, one) - Field::new(Mode::Public, one);
    assert_eq!(zero, candidate.eject_value());

    let candidate = Field::new(Mode::Private, one) - Field::new(Mode::Public, one);
    assert_eq!(zero, candidate.eject_value());

    let candidate = Field::new(Mode::Private, one) - Field::new(Mode::Private, one);
    assert_eq!(zero, candidate.eject_value());
}

#[test]
fn test_2_minus_1() {
    let one = Fp::ONE;
    let two = one + one;

    let candidate_two = Field::one() + Field::one();
    let candidate = candidate_two - Field::one();
    assert_eq!(one, candidate.eject_value());

    let candidate_two = Field::one() + &Field::one();
    let candidate = candidate_two - &Field::one();
    assert_eq!(one, candidate.eject_value());

    let candidate = Field::new(Mode::Public, two) - Field::new(Mode::Public, one);
    assert_eq!(one, candidate.eject_value());

    let candidate = Field::new(Mode::Private, two) - Field::new(Mode::Public, one);
    assert_eq!(one, candidate.eject_value());

    let candidate = Field::new(Mode::Private, two) - Field::new(Mode::Private, one);
    assert_eq!(one, candidate.eject_value());
}
