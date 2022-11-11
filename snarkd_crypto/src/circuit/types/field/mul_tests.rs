use std::ops::Mul;

use crate::{
    assert_count, assert_output_mode,
    bls12_377::Field as FieldTrait,
    circuit::{
        helpers::CircuitType,
        rng_test_struct::{TestRng, Uniform},
    },
};

use super::*;

const ITERATIONS: u64 = 100;

fn check_mul(name: &str, expected: &Fp, a: &Field, b: &Field) {
    Circuit::scope(name, || {
        let candidate = a * b;
        assert_eq!(
            *expected,
            candidate.eject_value(),
            "({} * {})",
            a.eject_value(),
            b.eject_value()
        );
        assert_count!(Mul(Field, Field) => Field, &(a.eject_mode(), b.eject_mode()));
        assert_output_mode!(Mul(Field, Field) => Field, &(CircuitType::from(a), CircuitType::from(b)), candidate);
    });
}

fn check_mul_assign(name: &str, expected: &Fp, a: &Field, b: &Field) {
    Circuit::scope(name, || {
        let mut candidate = a.clone();
        candidate *= b;
        assert_eq!(
            *expected,
            candidate.eject_value(),
            "({} * {})",
            a.eject_value(),
            b.eject_value()
        );
        assert_count!(Mul(Field, Field) => Field, &(a.eject_mode(), b.eject_mode()));
        assert_output_mode!(Mul(Field, Field) => Field, &(CircuitType::from(a), CircuitType::from(b)), candidate);
    });
}

fn run_test(mode_a: Mode, mode_b: Mode) {
    let mut rng = TestRng::default();

    for i in 0..ITERATIONS {
        let first = Uniform::rand(&mut rng);
        let second = Uniform::rand(&mut rng);

        let expected = first * second;
        let a = Field::new(mode_a, first);
        let b = Field::new(mode_b, second);

        let name = format!("Mul: a + b {}", i);
        check_mul(&name, &expected, &a, &b);
        let name = format!("MulAssign: a + b {}", i);
        check_mul_assign(&name, &expected, &a, &b);

        // Test identity.
        let name = format!("Mul: a * 1 {}", i);
        let one = Field::new(mode_b, Fp::ONE);
        check_mul(&name, &first, &a, &one);
        let name = format!("MulAssign: a * 1 {}", i);
        check_mul_assign(&name, &first, &a, &one);

        let name = format!("Mul: 1 * b {}", i);
        let one = Field::new(mode_a, Fp::ONE);
        check_mul(&name, &second, &one, &b);
        let name = format!("MulAssign: 1 * b {}", i);
        check_mul_assign(&name, &second, &one, &b);

        // Test zero.
        let name = format!("Mul: a * 0 {}", i);
        let zero = Field::new(mode_b, Fp::ZERO);
        check_mul(&name, &Fp::ZERO, &a, &zero);
        let name = format!("MulAssign: a * 0 {}", i);
        check_mul_assign(&name, &Fp::ZERO, &a, &zero);

        let name = format!("Mul: 0 * b {}", i);
        let zero = Field::new(mode_a, Fp::ZERO);
        check_mul(&name, &Fp::ZERO, &zero, &b);
        let name = format!("MulAssign: 0 * b {}", i);
        check_mul_assign(&name, &Fp::ZERO, &zero, &b);
    }
}

#[test]
fn test_constant_times_constant() {
    run_test(Mode::Constant, Mode::Constant);
}

#[test]
fn test_constant_times_public() {
    run_test(Mode::Constant, Mode::Public);
}

#[test]
fn test_constant_times_private() {
    run_test(Mode::Constant, Mode::Private);
}

#[test]
fn test_public_times_constant() {
    run_test(Mode::Public, Mode::Constant);
}

#[test]
fn test_private_times_constant() {
    run_test(Mode::Private, Mode::Constant);
}

#[test]
fn test_public_times_public() {
    run_test(Mode::Public, Mode::Public);
}

#[test]
fn test_public_times_private() {
    run_test(Mode::Public, Mode::Private);
}

#[test]
fn test_private_times_public() {
    run_test(Mode::Private, Mode::Public);
}

#[test]
fn test_private_times_private() {
    run_test(Mode::Private, Mode::Private);
}

#[test]
fn test_mul_matches() {
    let mut rng = TestRng::default();

    // Sample two random elements.
    let a = Uniform::rand(&mut rng);
    let b = Uniform::rand(&mut rng);
    let expected = a * b;

    // Constant
    let first = Field::new(Mode::Constant, a);
    let second = Field::new(Mode::Constant, b);
    let candidate_a = first * second;
    assert_eq!(expected, candidate_a.eject_value());

    // Private
    let first = Field::new(Mode::Private, a);
    let second = Field::new(Mode::Private, b);
    let candidate_b = first * second;
    assert_eq!(expected, candidate_b.eject_value());
}

#[test]
fn test_0_times_0() {
    let zero = Fp::ZERO;

    let candidate = Field::zero() * Field::zero();
    assert_eq!(zero, candidate.eject_value());

    let candidate = Field::zero() * &Field::zero();
    assert_eq!(zero, candidate.eject_value());

    let candidate = Field::new(Mode::Public, zero) * Field::new(Mode::Public, zero);
    assert_eq!(zero, candidate.eject_value());

    let candidate = Field::new(Mode::Public, zero) * Field::new(Mode::Private, zero);
    assert_eq!(zero, candidate.eject_value());

    let candidate = Field::new(Mode::Private, zero) * Field::new(Mode::Private, zero);
    assert_eq!(zero, candidate.eject_value());
}

#[test]
fn test_0_times_1() {
    let zero = Fp::ZERO;
    let one = Fp::ONE;

    let candidate = Field::zero() * Field::one();
    assert_eq!(zero, candidate.eject_value());

    let candidate = Field::zero() * &Field::one();
    assert_eq!(zero, candidate.eject_value());

    let candidate = Field::one() * Field::zero();
    assert_eq!(zero, candidate.eject_value());

    let candidate = Field::one() * &Field::zero();
    assert_eq!(zero, candidate.eject_value());

    let candidate = Field::new(Mode::Public, one) * Field::new(Mode::Public, zero);
    assert_eq!(zero, candidate.eject_value());

    let candidate = Field::new(Mode::Public, one) * Field::new(Mode::Private, zero);
    assert_eq!(zero, candidate.eject_value());

    let candidate = Field::new(Mode::Private, one) * Field::new(Mode::Private, zero);
    assert_eq!(zero, candidate.eject_value());
}

#[test]
fn test_1_times_1() {
    let one = Fp::ONE;

    let candidate = Field::one() * Field::one();
    assert_eq!(one, candidate.eject_value());

    let candidate = Field::one() * &Field::one();
    assert_eq!(one, candidate.eject_value());

    let candidate = Field::new(Mode::Public, one) * Field::new(Mode::Public, one);
    assert_eq!(one, candidate.eject_value());

    let candidate = Field::new(Mode::Private, one) * Field::new(Mode::Public, one);
    assert_eq!(one, candidate.eject_value());

    let candidate = Field::new(Mode::Private, one) * Field::new(Mode::Private, one);
    assert_eq!(one, candidate.eject_value());
}

#[test]
fn test_2_times_2() {
    let one = Fp::ONE;
    let two = one + one;
    let four = two + two;

    let candidate_two = Field::one() + Field::one();
    let candidate = candidate_two * (Field::one() + Field::one());
    assert_eq!(four, candidate.eject_value());

    let candidate = Field::new(Mode::Public, two) * Field::new(Mode::Public, two);
    assert_eq!(four, candidate.eject_value());

    let candidate = Field::new(Mode::Private, two) * Field::new(Mode::Public, two);
    assert_eq!(four, candidate.eject_value());

    let candidate = Field::new(Mode::Private, two) * Field::new(Mode::Private, two);
    assert_eq!(four, candidate.eject_value());
}
