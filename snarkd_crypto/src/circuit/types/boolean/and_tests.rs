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
fn check_and(
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
        let candidate = &a & &b;
        assert_eq!(
            expected,
            candidate.eject_value(),
            "({} AND {})",
            a.eject_value(),
            b.eject_value()
        );
        assert_scope!(num_constants, num_public, num_private, num_constraints);
    });
}

#[test]
fn test_constant_and_constant() {
    // false AND false
    let expected = false;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Constant, false);
    check_and("false AND false", expected, a, b, 0, 0, 0, 0);

    // false AND true
    let expected = false;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Constant, true);
    check_and("false AND true", expected, a, b, 0, 0, 0, 0);

    // true AND false
    let expected = false;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Constant, false);
    check_and("true AND false", expected, a, b, 0, 0, 0, 0);

    // true AND true
    let expected = true;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Constant, true);
    check_and("true AND true", expected, a, b, 0, 0, 0, 0);
}

#[test]
fn test_constant_and_public() {
    // false AND false
    let expected = false;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Public, false);
    check_and("false AND false", expected, a, b, 0, 0, 0, 0);

    // false AND true
    let expected = false;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Public, true);
    check_and("false AND true", expected, a, b, 0, 0, 0, 0);

    // true AND false
    let expected = false;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Public, false);
    check_and("true AND false", expected, a, b, 0, 0, 0, 0);

    // true AND true
    let expected = true;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Public, true);
    check_and("true AND true", expected, a, b, 0, 0, 0, 0);
}

#[test]
fn test_constant_and_private() {
    // false AND false
    let expected = false;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Private, false);
    check_and("false AND false", expected, a, b, 0, 0, 0, 0);

    // false AND true
    let expected = false;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Private, true);
    check_and("false AND true", expected, a, b, 0, 0, 0, 0);

    // true AND false
    let expected = false;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Private, false);
    check_and("true AND false", expected, a, b, 0, 0, 0, 0);

    // true AND true
    let expected = true;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Private, true);
    check_and("true AND true", expected, a, b, 0, 0, 0, 0);
}

#[test]
fn test_public_and_constant() {
    // false AND false
    let expected = false;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Constant, false);
    check_and("false AND false", expected, a, b, 0, 0, 0, 0);

    // false AND true
    let expected = false;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Constant, true);
    check_and("false AND true", expected, a, b, 0, 0, 0, 0);

    // true AND false
    let expected = false;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Constant, false);
    check_and("true AND false", expected, a, b, 0, 0, 0, 0);

    // true AND true
    let expected = true;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Constant, true);
    check_and("true AND true", expected, a, b, 0, 0, 0, 0);
}

#[test]
fn test_public_and_public() {
    // false AND false
    let expected = false;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Public, false);
    check_and("false AND false", expected, a, b, 0, 0, 1, 1);

    // false AND true
    let expected = false;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Public, true);
    check_and("false AND true", expected, a, b, 0, 0, 1, 1);

    // true AND false
    let expected = false;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Public, false);
    check_and("true AND false", expected, a, b, 0, 0, 1, 1);

    // true AND true
    let expected = true;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Public, true);
    check_and("true AND true", expected, a, b, 0, 0, 1, 1);
}

#[test]
fn test_public_and_private() {
    // false AND false
    let expected = false;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Private, false);
    check_and("false AND false", expected, a, b, 0, 0, 1, 1);

    // false AND true
    let expected = false;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Private, true);
    check_and("false AND true", expected, a, b, 0, 0, 1, 1);

    // true AND false
    let expected = false;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Private, false);
    check_and("true AND false", expected, a, b, 0, 0, 1, 1);

    // true AND true
    let expected = true;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Private, true);
    check_and("true AND true", expected, a, b, 0, 0, 1, 1);
}

#[test]
fn test_private_and_constant() {
    // false AND false
    let expected = false;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Constant, false);
    check_and("false AND false", expected, a, b, 0, 0, 0, 0);

    // false AND true
    let expected = false;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Constant, true);
    check_and("false AND true", expected, a, b, 0, 0, 0, 0);

    // true AND false
    let expected = false;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Constant, false);
    check_and("true AND false", expected, a, b, 0, 0, 0, 0);

    // true AND true
    let expected = true;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Constant, true);
    check_and("true AND true", expected, a, b, 0, 0, 0, 0);
}

#[test]
fn test_private_and_public() {
    // false AND false
    let expected = false;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Private, false);
    check_and("false AND false", expected, a, b, 0, 0, 1, 1);

    // false AND true
    let expected = false;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Private, true);
    check_and("false AND true", expected, a, b, 0, 0, 1, 1);

    // true AND false
    let expected = false;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Private, false);
    check_and("true AND false", expected, a, b, 0, 0, 1, 1);

    // true AND true
    let expected = true;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Private, true);
    check_and("true AND true", expected, a, b, 0, 0, 1, 1);
}

#[test]
fn test_private_and_private() {
    // false AND false
    let expected = false;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Private, false);
    check_and("false AND false", expected, a, b, 0, 0, 1, 1);

    // false AND true
    let expected = false;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Private, true);
    check_and("false AND true", expected, a, b, 0, 0, 1, 1);

    // true AND false
    let expected = false;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Private, false);
    check_and("true AND false", expected, a, b, 0, 0, 1, 1);

    // true AND true
    let expected = true;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Private, true);
    check_and("true AND true", expected, a, b, 0, 0, 1, 1);
}
