use crate::{
    assert_scope,
    circuit::{
        circuit::Circuit,
        helpers::Mode,
        traits::{Eject, Inject},
        types::Boolean,
        Environment,
    },
};

#[allow(clippy::too_many_arguments)]
fn check_or(
    name: &str,
    expected: bool,
    a: Boolean,
    b: Boolean,
    num_constants: u64,
    num_public: u64,
    num_private: u64,
    num_constraints: u64,
) {
    Circuit::scope(name, || {
        let candidate = &a | &b;
        assert_eq!(
            expected,
            candidate.eject_value(),
            "({} OR {})",
            a.eject_value(),
            b.eject_value()
        );
        assert_scope!(num_constants, num_public, num_private, num_constraints);
    });
}

#[test]
fn test_constant_or_constant() {
    // false OR false
    let expected = false;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Constant, false);
    check_or("false OR false", expected, a, b, 0, 0, 0, 0);

    // false OR true
    let expected = true;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Constant, true);
    check_or("false OR true", expected, a, b, 0, 0, 0, 0);

    // true OR false
    let expected = true;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Constant, false);
    check_or("true OR false", expected, a, b, 0, 0, 0, 0);

    // true OR true
    let expected = true;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Constant, true);
    check_or("true OR true", expected, a, b, 0, 0, 0, 0);
}

#[test]
fn test_constant_or_public() {
    // false OR false
    let expected = false;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Public, false);
    check_or("false OR false", expected, a, b, 0, 0, 0, 0);

    // false OR true
    let expected = true;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Public, true);
    check_or("false OR true", expected, a, b, 0, 0, 0, 0);

    // true OR false
    let expected = true;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Public, false);
    check_or("true OR false", expected, a, b, 0, 0, 0, 0);

    // true OR true
    let expected = true;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Public, true);
    check_or("true OR true", expected, a, b, 0, 0, 0, 0);
}

#[test]
fn test_constant_or_private() {
    // false OR false
    let expected = false;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Private, false);
    check_or("false OR false", expected, a, b, 0, 0, 0, 0);

    // false OR true
    let expected = true;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Private, true);
    check_or("false OR true", expected, a, b, 0, 0, 0, 0);

    // true OR false
    let expected = true;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Private, false);
    check_or("true OR false", expected, a, b, 0, 0, 0, 0);

    // true OR true
    let expected = true;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Private, true);
    check_or("true OR true", expected, a, b, 0, 0, 0, 0);
}

#[test]
fn test_public_or_constant() {
    // false OR false
    let expected = false;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Constant, false);
    check_or("false OR false", expected, a, b, 0, 0, 0, 0);

    // false OR true
    let expected = true;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Constant, true);
    check_or("false OR true", expected, a, b, 0, 0, 0, 0);

    // true OR false
    let expected = true;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Constant, false);
    check_or("true OR false", expected, a, b, 0, 0, 0, 0);

    // true OR true
    let expected = true;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Constant, true);
    check_or("true OR true", expected, a, b, 0, 0, 0, 0);
}

#[test]
fn test_public_or_public() {
    // false OR false
    let expected = false;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Public, false);
    check_or("false OR false", expected, a, b, 0, 0, 1, 1);

    // false OR true
    let expected = true;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Public, true);
    check_or("false OR true", expected, a, b, 0, 0, 1, 1);

    // true OR false
    let expected = true;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Public, false);
    check_or("true OR false", expected, a, b, 0, 0, 1, 1);

    // true OR true
    let expected = true;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Public, true);
    check_or("true OR true", expected, a, b, 0, 0, 1, 1);
}

#[test]
fn test_public_or_private() {
    // false OR false
    let expected = false;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Private, false);
    check_or("false OR false", expected, a, b, 0, 0, 1, 1);

    // false OR true
    let expected = true;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Private, true);
    check_or("false OR true", expected, a, b, 0, 0, 1, 1);

    // true OR false
    let expected = true;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Private, false);
    check_or("true OR false", expected, a, b, 0, 0, 1, 1);

    // true OR true
    let expected = true;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Private, true);
    check_or("true OR true", expected, a, b, 0, 0, 1, 1);
}

#[test]
fn test_private_or_constant() {
    // false OR false
    let expected = false;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Constant, false);
    check_or("false OR false", expected, a, b, 0, 0, 0, 0);

    // false OR true
    let expected = true;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Constant, true);
    check_or("false OR true", expected, a, b, 0, 0, 0, 0);

    // true OR false
    let expected = true;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Constant, false);
    check_or("true OR false", expected, a, b, 0, 0, 0, 0);

    // true OR true
    let expected = true;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Constant, true);
    check_or("true OR true", expected, a, b, 0, 0, 0, 0);
}

#[test]
fn test_private_or_public() {
    // false OR false
    let expected = false;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Public, false);
    check_or("false OR false", expected, a, b, 0, 0, 1, 1);

    // false OR true
    let expected = true;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Public, true);
    check_or("false OR true", expected, a, b, 0, 0, 1, 1);

    // true OR false
    let expected = true;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Public, false);
    check_or("true OR false", expected, a, b, 0, 0, 1, 1);

    // true OR true
    let expected = true;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Public, true);
    check_or("true OR true", expected, a, b, 0, 0, 1, 1);
}

#[test]
fn test_private_or_private() {
    // false OR false
    let expected = false;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Private, false);
    check_or("false OR false", expected, a, b, 0, 0, 1, 1);

    // false OR true
    let expected = true;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Private, true);
    check_or("false OR true", expected, a, b, 0, 0, 1, 1);

    // true OR false
    let expected = true;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Private, false);
    check_or("true OR false", expected, a, b, 0, 0, 1, 1);

    // true OR true
    let expected = true;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Private, true);
    check_or("true OR true", expected, a, b, 0, 0, 1, 1);
}
