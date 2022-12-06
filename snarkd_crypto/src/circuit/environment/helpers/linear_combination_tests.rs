use std::rc::Rc;

use crate::{
    bls12_377::{Field, Fp},
    circuit::{
        circuit::Circuit,
        helpers::{LinearCombination, Mode, Variable},
        Environment,
    },
};

#[test]
fn test_zero() {
    let zero = Fp::ZERO;

    let candidate = LinearCombination::zero();
    assert_eq!(zero, candidate.constant);
    assert!(candidate.terms.is_empty());
    assert_eq!(zero, candidate.value());
}

#[test]
fn test_one() {
    let one = Fp::ONE;

    let candidate = LinearCombination::one();
    assert_eq!(one, candidate.constant);
    assert!(candidate.terms.is_empty());
    assert_eq!(one, candidate.value());
}

#[test]
fn test_two() {
    let one = Fp::ONE;
    let two = one + one;

    let candidate = LinearCombination::one() + LinearCombination::one();
    assert_eq!(two, candidate.constant);
    assert!(candidate.terms.is_empty());
    assert_eq!(two, candidate.value());
}

#[test]
fn test_is_constant() {
    let zero = Fp::ZERO;
    let one = Fp::ONE;

    let candidate = LinearCombination::zero();
    assert!(candidate.is_constant());
    assert_eq!(zero, candidate.constant);
    assert_eq!(zero, candidate.value());

    let candidate = LinearCombination::one();
    assert!(candidate.is_constant());
    assert_eq!(one, candidate.constant);
    assert_eq!(one, candidate.value());
}

#[test]
fn test_mul() {
    let zero = Fp::ZERO;
    let one = Fp::ONE;
    let two = one + one;
    let four = two + two;

    let start = LinearCombination::from(Variable::Public(1, Rc::new(one)));
    assert!(!start.is_constant());
    assert_eq!(one, start.value());

    // Compute 1 * 4.
    let candidate = start * four;
    assert_eq!(four, candidate.value());
    assert_eq!(zero, candidate.constant);
    assert_eq!(1, candidate.terms.len());

    let (candidate_variable, candidate_coefficient) = candidate.terms.iter().next().unwrap();
    assert!(candidate_variable.is_public());
    assert_eq!(one, candidate_variable.value());
    assert_eq!(four, *candidate_coefficient);
}

#[test]
fn test_num_additions() {
    let one_public = &Circuit::new_variable(Mode::Public, Fp::ONE);
    let one_private = &Circuit::new_variable(Mode::Private, Fp::ONE);
    let two_private = one_private + one_private;

    let candidate = LinearCombination::zero();
    assert_eq!(0, candidate.num_additions());

    let candidate = LinearCombination::one();
    assert_eq!(0, candidate.num_additions());

    let candidate = LinearCombination::zero() + one_public;
    assert_eq!(0, candidate.num_additions());

    let candidate = LinearCombination::one() + one_public;
    assert_eq!(1, candidate.num_additions());

    let candidate = LinearCombination::zero() + one_public + one_public;
    assert_eq!(0, candidate.num_additions());

    let candidate = LinearCombination::one() + one_public + one_public;
    assert_eq!(1, candidate.num_additions());

    let candidate = LinearCombination::zero() + one_public + one_private;
    assert_eq!(1, candidate.num_additions());

    let candidate = LinearCombination::one() + one_public + one_private;
    assert_eq!(2, candidate.num_additions());

    let candidate = LinearCombination::zero() + one_public + one_private + one_public;
    assert_eq!(1, candidate.num_additions());

    let candidate = LinearCombination::one() + one_public + one_private + one_public;
    assert_eq!(2, candidate.num_additions());

    let candidate = LinearCombination::zero() + one_public + one_private + one_public + one_private;
    assert_eq!(1, candidate.num_additions());

    let candidate = LinearCombination::one() + one_public + one_private + one_public + one_private;
    assert_eq!(2, candidate.num_additions());

    let candidate = LinearCombination::zero()
        + LinearCombination::zero()
        + one_public
        + one_private
        + one_public
        + one_private;
    assert_eq!(1, candidate.num_additions());

    let candidate = LinearCombination::one()
        + LinearCombination::zero()
        + one_public
        + one_private
        + one_public
        + one_private;
    assert_eq!(2, candidate.num_additions());

    let candidate = LinearCombination::one()
        + LinearCombination::zero()
        + LinearCombination::one()
        + one_public
        + one_private
        + one_public
        + one_private;
    assert_eq!(2, candidate.num_additions());

    let candidate = LinearCombination::zero()
        + LinearCombination::zero()
        + one_public
        + one_private
        + one_public
        + one_private
        + &two_private;
    assert_eq!(1, candidate.num_additions());

    let candidate = LinearCombination::one()
        + LinearCombination::zero()
        + one_public
        + one_private
        + one_public
        + one_private
        + &two_private;
    assert_eq!(2, candidate.num_additions());

    let candidate = LinearCombination::one()
        + LinearCombination::zero()
        + LinearCombination::one()
        + one_public
        + one_private
        + one_public
        + one_private
        + &two_private;
    assert_eq!(2, candidate.num_additions());

    // Now check with subtractions.

    let candidate = LinearCombination::zero() - one_public;
    assert_eq!(0, candidate.num_additions());

    let candidate = LinearCombination::one() - one_public;
    assert_eq!(1, candidate.num_additions());

    let candidate = LinearCombination::zero() + one_public - one_public;
    assert_eq!(0, candidate.num_additions());

    let candidate = LinearCombination::one() + one_public - one_public;
    assert_eq!(0, candidate.num_additions());

    let candidate = LinearCombination::zero() + one_public - one_private;
    assert_eq!(1, candidate.num_additions());

    let candidate = LinearCombination::one() + one_public - one_private;
    assert_eq!(2, candidate.num_additions());

    let candidate = LinearCombination::zero() + one_public + one_private - one_public;
    assert_eq!(0, candidate.num_additions());

    let candidate = LinearCombination::one() + one_public + one_private - one_public;
    assert_eq!(1, candidate.num_additions());

    let candidate = LinearCombination::zero() + one_public + one_private + one_public - one_private;
    assert_eq!(0, candidate.num_additions());

    let candidate = LinearCombination::one() + one_public + one_private + one_public - one_private;
    assert_eq!(1, candidate.num_additions());

    let candidate = LinearCombination::zero()
        + LinearCombination::zero()
        + one_public
        + one_private
        + one_public
        - one_private;
    assert_eq!(0, candidate.num_additions());

    let candidate = LinearCombination::one()
        + LinearCombination::zero()
        + one_public
        + one_private
        + one_public
        - one_private;
    assert_eq!(1, candidate.num_additions());

    let candidate = LinearCombination::one()
        + LinearCombination::zero()
        + LinearCombination::one()
        + one_public
        + one_private
        + one_public
        - one_private;
    assert_eq!(1, candidate.num_additions());

    let candidate = LinearCombination::zero()
        + LinearCombination::zero()
        + one_public
        + one_private
        + one_public
        + one_private
        - &two_private;
    assert_eq!(0, candidate.num_additions());

    let candidate = LinearCombination::one()
        + LinearCombination::zero()
        + one_public
        + one_private
        + one_public
        + one_private
        - &two_private;
    assert_eq!(1, candidate.num_additions());

    let candidate = LinearCombination::one()
        + LinearCombination::zero()
        + LinearCombination::one()
        + one_public
        + one_private
        + one_public
        + one_private
        - &two_private;
    assert_eq!(1, candidate.num_additions());
}
