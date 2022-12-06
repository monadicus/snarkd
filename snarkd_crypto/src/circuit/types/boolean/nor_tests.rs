use crate::{
    assert_count, assert_output_mode,
    circuit::{
        circuit::Circuit,
        helpers::{CircuitType, Mode},
        traits::{Eject, Inject, Nor},
        types::Boolean,
        Environment,
    },
};

fn check_nor(name: &str, expected: bool, a: Boolean, b: Boolean) {
    Circuit::scope(name, || {
        let candidate = a.nor(&b);
        assert_eq!(
            expected,
            candidate.eject_value(),
            "({} NOR {})",
            a.eject_value(),
            b.eject_value()
        );
        assert_count!(Nor(Boolean, Boolean) => Boolean, &(a.eject_mode(), b.eject_mode()));
        assert_output_mode!(Nor(Boolean, Boolean) => Boolean, &(CircuitType::from(&a), CircuitType::from(&b)), candidate);
    });
    Circuit::reset();
}

#[test]
fn test_constant_nor_constant() {
    // false NOR false
    let expected = true;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Constant, false);
    check_nor("false NOR false", expected, a, b);

    // false NOR true
    let expected = false;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Constant, true);
    check_nor("false NOR true", expected, a, b);

    // true NOR false
    let expected = false;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Constant, false);
    check_nor("true NOR false", expected, a, b);

    // true NOR true
    let expected = false;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Constant, true);
    check_nor("true NOR true", expected, a, b);
}

#[test]
fn test_constant_nor_public() {
    // false NOR false
    let expected = true;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Public, false);
    check_nor("false NOR false", expected, a, b);

    // false NOR true
    let expected = false;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Public, true);
    check_nor("false NOR true", expected, a, b);

    // true NOR false
    let expected = false;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Public, false);
    check_nor("true NOR false", expected, a, b);

    // true NOR true
    let expected = false;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Public, true);
    check_nor("true NOR true", expected, a, b);
}

#[test]
fn test_constant_nor_private() {
    // false NOR false
    let expected = true;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Private, false);
    check_nor("false NOR false", expected, a, b);

    // false NOR true
    let expected = false;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Private, true);
    check_nor("false NOR true", expected, a, b);

    // true NOR false
    let expected = false;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Private, false);
    check_nor("true NOR false", expected, a, b);

    // true NOR true
    let expected = false;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Private, true);
    check_nor("true NOR true", expected, a, b);
}

#[test]
fn test_public_nor_constant() {
    // false NOR false
    let expected = true;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Constant, false);
    check_nor("false NOR false", expected, a, b);

    // false NOR true
    let expected = false;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Constant, true);
    check_nor("false NOR true", expected, a, b);

    // true NOR false
    let expected = false;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Constant, false);
    check_nor("true NOR false", expected, a, b);

    // true NOR true
    let expected = false;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Constant, true);
    check_nor("true NOR true", expected, a, b);
}

#[test]
fn test_public_nor_public() {
    // false NOR false
    let expected = true;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Public, false);
    check_nor("false NOR false", expected, a, b);

    // false NOR true
    let expected = false;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Public, true);
    check_nor("false NOR true", expected, a, b);

    // true NOR false
    let expected = false;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Public, false);
    check_nor("true NOR false", expected, a, b);

    // true NOR true
    let expected = false;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Public, true);
    check_nor("true NOR true", expected, a, b);
}

#[test]
fn test_public_nor_private() {
    // false NOR false
    let expected = true;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Private, false);
    check_nor("false NOR false", expected, a, b);

    // false NOR true
    let expected = false;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Private, true);
    check_nor("false NOR true", expected, a, b);

    // true NOR false
    let expected = false;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Private, false);
    check_nor("true NOR false", expected, a, b);

    // true NOR true
    let expected = false;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Private, true);
    check_nor("true NOR true", expected, a, b);
}

#[test]
fn test_private_nor_constant() {
    // false NOR false
    let expected = true;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Constant, false);
    check_nor("false NOR false", expected, a, b);

    // false NOR true
    let expected = false;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Constant, true);
    check_nor("false NOR true", expected, a, b);

    // true NOR false
    let expected = false;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Constant, false);
    check_nor("true NOR false", expected, a, b);

    // true NOR true
    let expected = false;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Constant, true);
    check_nor("true NOR true", expected, a, b);
}

#[test]
fn test_private_nor_public() {
    // false NOR false
    let expected = true;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Public, false);
    check_nor("false NOR false", expected, a, b);

    // false NOR true
    let expected = false;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Public, true);
    check_nor("false NOR true", expected, a, b);

    // true NOR false
    let expected = false;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Public, false);
    check_nor("true NOR false", expected, a, b);

    // true NOR true
    let expected = false;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Public, true);
    check_nor("true NOR true", expected, a, b);
}

#[test]
fn test_private_nor_private() {
    // false NOR false
    let expected = true;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Private, false);
    check_nor("false NOR false", expected, a, b);

    // false NOR true
    let expected = false;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Private, true);
    check_nor("false NOR true", expected, a, b);

    // true NOR false
    let expected = false;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Private, false);
    check_nor("true NOR false", expected, a, b);

    // true NOR true
    let expected = false;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Private, true);
    check_nor("true NOR true", expected, a, b);
}
