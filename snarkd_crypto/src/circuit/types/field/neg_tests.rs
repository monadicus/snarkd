use std::ops::Neg;

use crate::{
    assert_count, assert_output_mode,
    bls12_377::Field as FieldTrait,
    circuit::{
        helpers::Mode,
        rng_test_struct::{TestRng, Uniform},
    },
};

use super::*;

const ITERATIONS: u64 = 1_000;

fn check_neg(name: &str, mode: Mode, rng: &mut TestRng) {
    let check_neg = |given: Fp| {
        // Compute it's negation.
        let expected = given.neg();
        let candidate = Field::new(mode, given);

        // Check negation.
        Circuit::scope(name, || {
            let result = candidate.neg();
            assert_eq!(expected, result.eject_value());
            assert_count!(Neg(Field) => Field, &mode);
            assert_output_mode!(Neg(Field) => Field, &mode, result);
        });
    };

    for _ in 0..ITERATIONS {
        // Sample a random element.
        let given = Uniform::rand(rng);
        check_neg(given)
    }
    // Check zero case.
    check_neg(Fp::ZERO);
    // Check one case.
    check_neg(Fp::ONE);
}

#[test]
fn test_neg() {
    let mut rng = TestRng::default();

    check_neg("Constant", Mode::Constant, &mut rng);
    check_neg("Public", Mode::Public, &mut rng);
    check_neg("Private", Mode::Private, &mut rng);
}

#[test]
fn test_zero() {
    let zero = Fp::ZERO;

    let candidate = Field::zero();
    assert_eq!(zero, candidate.eject_value());
    assert_eq!(zero, (-&candidate).eject_value());
    assert_eq!(zero, (-(-candidate)).eject_value());

    let candidate = Field::new(Mode::Public, zero);
    assert_eq!(zero, candidate.eject_value());
    assert_eq!(zero, (-&candidate).eject_value());
    assert_eq!(zero, (-(-candidate)).eject_value());

    let candidate = Field::new(Mode::Private, zero);
    assert_eq!(zero, candidate.eject_value());
    assert_eq!(zero, (-&candidate).eject_value());
    assert_eq!(zero, (-(-candidate)).eject_value());
}

#[test]
fn test_one() {
    let one = Fp::ONE;

    let candidate = Field::one();
    assert_eq!(one, candidate.eject_value());
    assert_eq!(-one, (-&candidate).eject_value());
    assert_eq!(one, (-(-candidate)).eject_value());

    let candidate = Field::new(Mode::Public, one);
    assert_eq!(one, candidate.eject_value());
    assert_eq!(-one, (-&candidate).eject_value());
    assert_eq!(one, (-(-candidate)).eject_value());

    let candidate = Field::new(Mode::Private, one);
    assert_eq!(one, candidate.eject_value());
    assert_eq!(-one, (-&candidate).eject_value());
    assert_eq!(one, (-(-candidate)).eject_value());
}
