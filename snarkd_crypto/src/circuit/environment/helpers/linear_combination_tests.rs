use std::rc::Rc;

#[test]
fn test_zero() {
    let zero = <Circuit as Environment>::BaseField::zero();

    let candidate = LinearCombination::zero();
    assert_eq!(zero, candidate.constant);
    assert!(candidate.terms.is_empty());
    assert_eq!(zero, candidate.value());
}

#[test]
fn test_one() {
    let one = <Circuit as Environment>::BaseField::one();

    let candidate = LinearCombination::one();
    assert_eq!(one, candidate.constant);
    assert!(candidate.terms.is_empty());
    assert_eq!(one, candidate.value());
}

#[test]
fn test_two() {
    let one = <Circuit as Environment>::BaseField::one();
    let two = one + one;

    let candidate = LinearCombination::one() + LinearCombination::one();
    assert_eq!(two, candidate.constant);
    assert!(candidate.terms.is_empty());
    assert_eq!(two, candidate.value());
}

#[test]
fn test_is_constant() {
    let zero = <Circuit as Environment>::BaseField::zero();
    let one = <Circuit as Environment>::BaseField::one();

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
    let zero = <Circuit as Environment>::BaseField::zero();
    let one = <Circuit as Environment>::BaseField::one();
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
fn test_debug() {
    let one_public =
        &Circuit::new_variable(Mode::Public, <Circuit as Environment>::BaseField::one());
    let one_private =
        &Circuit::new_variable(Mode::Private, <Circuit as Environment>::BaseField::one());
    {
        let expected = "Constant(1) + Public(1, 1) + Private(0, 1)";

        let candidate = LinearCombination::one() + one_public + one_private;
        assert_eq!(expected, format!("{:?}", candidate));

        let candidate = one_private + one_public + LinearCombination::one();
        assert_eq!(expected, format!("{:?}", candidate));

        let candidate = one_private + LinearCombination::one() + one_public;
        assert_eq!(expected, format!("{:?}", candidate));

        let candidate = one_public + LinearCombination::one() + one_private;
        assert_eq!(expected, format!("{:?}", candidate));
    }
    {
        let expected = "Constant(1) + 2 * Public(1, 1) + Private(0, 1)";

        let candidate = LinearCombination::one() + one_public + one_public + one_private;
        assert_eq!(expected, format!("{:?}", candidate));

        let candidate = one_private + one_public + LinearCombination::one() + one_public;
        assert_eq!(expected, format!("{:?}", candidate));

        let candidate = one_public + one_private + LinearCombination::one() + one_public;
        assert_eq!(expected, format!("{:?}", candidate));

        let candidate = one_public + LinearCombination::one() + one_private + one_public;
        assert_eq!(expected, format!("{:?}", candidate));
    }
    {
        let expected = "Constant(1) + Public(1, 1) + 2 * Private(0, 1)";

        let candidate = LinearCombination::one() + one_public + one_private + one_private;
        assert_eq!(expected, format!("{:?}", candidate));

        let candidate = one_private + one_public + LinearCombination::one() + one_private;
        assert_eq!(expected, format!("{:?}", candidate));

        let candidate = one_private + one_private + LinearCombination::one() + one_public;
        assert_eq!(expected, format!("{:?}", candidate));

        let candidate = one_public + LinearCombination::one() + one_private + one_private;
        assert_eq!(expected, format!("{:?}", candidate));
    }
    {
        let expected = "Constant(1) + Public(1, 1)";

        let candidate = LinearCombination::one() + one_public + one_private - one_private;
        assert_eq!(expected, format!("{:?}", candidate));

        let candidate = one_private + one_public + LinearCombination::one() - one_private;
        assert_eq!(expected, format!("{:?}", candidate));

        let candidate = one_private - one_private + LinearCombination::one() + one_public;
        assert_eq!(expected, format!("{:?}", candidate));

        let candidate = one_public + LinearCombination::one() + one_private - one_private;
        assert_eq!(expected, format!("{:?}", candidate));
    }
}

#[rustfmt::skip]
    #[test]
    fn test_num_additions() {
        let one_public = &Circuit::new_variable(Mode::Public, <Circuit as Environment>::BaseField::one());
        let one_private = &Circuit::new_variable(Mode::Private, <Circuit as Environment>::BaseField::one());
        let two_private = one_private + one_private;

        let candidate = LinearCombination::<<Circuit as Environment>::BaseField>::zero();
        assert_eq!(0, candidate.num_additions());

        let candidate = LinearCombination::<<Circuit as Environment>::BaseField>::one();
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

        let candidate = LinearCombination::zero() + LinearCombination::zero() + one_public + one_private + one_public + one_private;
        assert_eq!(1, candidate.num_additions());

        let candidate = LinearCombination::one() + LinearCombination::zero() + one_public + one_private + one_public + one_private;
        assert_eq!(2, candidate.num_additions());

        let candidate = LinearCombination::one() + LinearCombination::zero() + LinearCombination::one() + one_public + one_private + one_public + one_private;
        assert_eq!(2, candidate.num_additions());

        let candidate = LinearCombination::zero() + LinearCombination::zero() + one_public + one_private + one_public + one_private + &two_private;
        assert_eq!(1, candidate.num_additions());

        let candidate = LinearCombination::one() + LinearCombination::zero() + one_public + one_private + one_public + one_private + &two_private;
        assert_eq!(2, candidate.num_additions());

        let candidate = LinearCombination::one() + LinearCombination::zero() + LinearCombination::one() + one_public + one_private + one_public + one_private + &two_private;
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

        let candidate = LinearCombination::zero() + LinearCombination::zero() + one_public + one_private + one_public - one_private;
        assert_eq!(0, candidate.num_additions());

        let candidate = LinearCombination::one() + LinearCombination::zero() + one_public + one_private + one_public - one_private;
        assert_eq!(1, candidate.num_additions());

        let candidate = LinearCombination::one() + LinearCombination::zero() + LinearCombination::one() + one_public + one_private + one_public - one_private;
        assert_eq!(1, candidate.num_additions());

        let candidate = LinearCombination::zero() + LinearCombination::zero() + one_public + one_private + one_public + one_private - &two_private;
        assert_eq!(0, candidate.num_additions());

        let candidate = LinearCombination::one() + LinearCombination::zero() + one_public + one_private + one_public + one_private - &two_private;
        assert_eq!(1, candidate.num_additions());

        let candidate = LinearCombination::one() + LinearCombination::zero() + LinearCombination::one() + one_public + one_private + one_public + one_private - &two_private;
        assert_eq!(1, candidate.num_additions());
    }
