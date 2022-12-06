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
fn check_xor(
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
        let candidate = &a ^ &b;
        assert_eq!(
            expected,
            candidate.eject_value(),
            "({} != {})",
            a.eject_value(),
            b.eject_value()
        );
        assert_scope!(num_constants, num_public, num_private, num_constraints);
    });
}

#[test]
fn test_constant_xor_constant() {
    // false != false
    let expected = false;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Constant, false);
    check_xor("false != false", expected, a, b, 0, 0, 0, 0);

    // false != true
    let expected = true;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Constant, true);
    check_xor("false != true", expected, a, b, 0, 0, 0, 0);

    // true != false
    let expected = true;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Constant, false);
    check_xor("true != false", expected, a, b, 0, 0, 0, 0);

    // true != true
    let expected = false;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Constant, true);
    check_xor("true != true", expected, a, b, 0, 0, 0, 0);
}

#[test]
fn test_constant_xor_public() {
    // false != false
    let expected = false;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Public, false);
    check_xor("false != false", expected, a, b, 0, 0, 0, 0);

    // false != true
    let expected = true;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Public, true);
    check_xor("false != true", expected, a, b, 0, 0, 0, 0);

    // true != false
    let expected = true;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Public, false);
    check_xor("true != false", expected, a, b, 0, 0, 0, 0);

    // true != true
    let expected = false;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Public, true);
    check_xor("true != true", expected, a, b, 0, 0, 0, 0);
}

#[test]
fn test_constant_xor_private() {
    // false != false
    let expected = false;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Private, false);
    check_xor("false != false", expected, a, b, 0, 0, 0, 0);

    // false != true
    let expected = true;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Private, true);
    check_xor("false != true", expected, a, b, 0, 0, 0, 0);

    // true != false
    let expected = true;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Private, false);
    check_xor("true != false", expected, a, b, 0, 0, 0, 0);

    // true != true
    let expected = false;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Private, true);
    check_xor("true != true", expected, a, b, 0, 0, 0, 0);
}

#[test]
fn test_public_xor_constant() {
    // false != false
    let expected = false;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Constant, false);
    check_xor("false != false", expected, a, b, 0, 0, 0, 0);

    // false != true
    let expected = true;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Constant, true);
    check_xor("false != true", expected, a, b, 0, 0, 0, 0);

    // true != false
    let expected = true;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Constant, false);
    check_xor("true != false", expected, a, b, 0, 0, 0, 0);

    // true != true
    let expected = false;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Constant, true);
    check_xor("true != true", expected, a, b, 0, 0, 0, 0);
}

#[test]
fn test_public_xor_public() {
    // false != false
    let expected = false;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Public, false);
    check_xor("false != false", expected, a, b, 0, 0, 1, 1);

    // false != true
    let expected = true;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Public, true);
    check_xor("false != true", expected, a, b, 0, 0, 1, 1);

    // true != false
    let expected = true;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Public, false);
    check_xor("true != false", expected, a, b, 0, 0, 1, 1);

    // true != true
    let expected = false;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Public, true);
    check_xor("true != true", expected, a, b, 0, 0, 1, 1);
}

#[test]
fn test_public_xor_private() {
    // false != false
    let expected = false;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Private, false);
    check_xor("false != false", expected, a, b, 0, 0, 1, 1);

    // false != true
    let expected = true;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Private, true);
    check_xor("false != true", expected, a, b, 0, 0, 1, 1);

    // true != false
    let expected = true;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Private, false);
    check_xor("true != false", expected, a, b, 0, 0, 1, 1);

    // true != true
    let expected = false;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Private, true);
    check_xor("true != true", expected, a, b, 0, 0, 1, 1);
}

#[test]
fn test_private_xor_constant() {
    // false != false
    let expected = false;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Constant, false);
    check_xor("false != false", expected, a, b, 0, 0, 0, 0);

    // false != true
    let expected = true;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Constant, true);
    check_xor("false != true", expected, a, b, 0, 0, 0, 0);

    // true != false
    let expected = true;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Constant, false);
    check_xor("true != false", expected, a, b, 0, 0, 0, 0);

    // true != true
    let expected = false;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Constant, true);
    check_xor("true != true", expected, a, b, 0, 0, 0, 0);
}

#[test]
fn test_private_xor_public() {
    // false != false
    let expected = false;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Public, false);
    check_xor("false != false", expected, a, b, 0, 0, 1, 1);

    // false != true
    let expected = true;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Public, true);
    check_xor("false != true", expected, a, b, 0, 0, 1, 1);

    // true != false
    let expected = true;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Public, false);
    check_xor("true != false", expected, a, b, 0, 0, 1, 1);

    // true != true
    let expected = false;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Public, true);
    check_xor("true != true", expected, a, b, 0, 0, 1, 1);
}

#[test]
fn test_private_xor_private() {
    // false != false
    let expected = false;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Private, false);
    check_xor("false != false", expected, a, b, 0, 0, 1, 1);

    // false != true
    let expected = true;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Private, true);
    check_xor("false != true", expected, a, b, 0, 0, 1, 1);

    // true != false
    let expected = true;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Private, false);
    check_xor("true != false", expected, a, b, 0, 0, 1, 1);

    // true != true
    let expected = false;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Private, true);
    check_xor("true != true", expected, a, b, 0, 0, 1, 1);
}
