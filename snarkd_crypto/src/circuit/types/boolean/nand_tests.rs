use crate::{
    assert_scope,
    circuit::{
        circuit::Circuit,
        helpers::Mode,
        traits::{Eject, Inject, Nand},
        types::Boolean,
        Environment,
    },
};

#[allow(clippy::too_many_arguments)]
fn check_nand(
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
        let candidate = a.nand(&b);
        assert_eq!(
            expected,
            candidate.eject_value(),
            "({} NAND {})",
            a.eject_value(),
            b.eject_value()
        );
        assert_scope!(num_constants, num_public, num_private, num_constraints);
    });
}

#[test]
fn test_constant_nand_constant() {
    // false NAND false
    let expected = true;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Constant, false);
    check_nand("false NAND false", expected, a, b, 0, 0, 0, 0);

    // false NAND true
    let expected = true;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Constant, true);
    check_nand("false NAND true", expected, a, b, 0, 0, 0, 0);

    // true NAND false
    let expected = true;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Constant, false);
    check_nand("true NAND false", expected, a, b, 0, 0, 0, 0);

    // true NAND true
    let expected = false;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Constant, true);
    check_nand("true NAND true", expected, a, b, 0, 0, 0, 0);
}

#[test]
fn test_constant_nand_public() {
    // false NAND false
    let expected = true;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Public, false);
    check_nand("false NAND false", expected, a, b, 0, 0, 0, 0);

    // false NAND true
    let expected = true;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Public, true);
    check_nand("false NAND true", expected, a, b, 0, 0, 0, 0);

    // true NAND false
    let expected = true;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Public, false);
    check_nand("true NAND false", expected, a, b, 0, 0, 0, 0);

    // true NAND true
    let expected = false;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Public, true);
    check_nand("true NAND true", expected, a, b, 0, 0, 0, 0);
}

#[test]
fn test_constant_nand_private() {
    // false NAND false
    let expected = true;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Private, false);
    check_nand("false NAND false", expected, a, b, 0, 0, 0, 0);

    // false NAND true
    let expected = true;
    let a = Boolean::new(Mode::Constant, false);
    let b = Boolean::new(Mode::Private, true);
    check_nand("false NAND true", expected, a, b, 0, 0, 0, 0);

    // true NAND false
    let expected = true;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Private, false);
    check_nand("true NAND false", expected, a, b, 0, 0, 0, 0);

    // true NAND true
    let expected = false;
    let a = Boolean::new(Mode::Constant, true);
    let b = Boolean::new(Mode::Private, true);
    check_nand("true NAND true", expected, a, b, 0, 0, 0, 0);
}

#[test]
fn test_public_nand_constant() {
    // false NAND false
    let expected = true;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Constant, false);
    check_nand("false NAND false", expected, a, b, 0, 0, 0, 0);

    // false NAND true
    let expected = true;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Constant, true);
    check_nand("false NAND true", expected, a, b, 0, 0, 0, 0);

    // true NAND false
    let expected = true;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Constant, false);
    check_nand("true NAND false", expected, a, b, 0, 0, 0, 0);

    // true NAND true
    let expected = false;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Constant, true);
    check_nand("true NAND true", expected, a, b, 0, 0, 0, 0);
}

#[test]
fn test_public_nand_public() {
    // false NAND false
    let expected = true;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Public, false);
    check_nand("false NAND false", expected, a, b, 0, 0, 1, 1);

    // false NAND true
    let expected = true;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Public, true);
    check_nand("false NAND true", expected, a, b, 0, 0, 1, 1);

    // true NAND false
    let expected = true;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Public, false);
    check_nand("true NAND false", expected, a, b, 0, 0, 1, 1);

    // true NAND true
    let expected = false;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Public, true);
    check_nand("true NAND true", expected, a, b, 0, 0, 1, 1);
}

#[test]
fn test_public_nand_private() {
    // false NAND false
    let expected = true;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Private, false);
    check_nand("false NAND false", expected, a, b, 0, 0, 1, 1);

    // false NAND true
    let expected = true;
    let a = Boolean::new(Mode::Public, false);
    let b = Boolean::new(Mode::Private, true);
    check_nand("false NAND true", expected, a, b, 0, 0, 1, 1);

    // true NAND false
    let expected = true;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Private, false);
    check_nand("true NAND false", expected, a, b, 0, 0, 1, 1);

    // true NAND true
    let expected = false;
    let a = Boolean::new(Mode::Public, true);
    let b = Boolean::new(Mode::Private, true);
    check_nand("true NAND true", expected, a, b, 0, 0, 1, 1);
}

#[test]
fn test_private_nand_constant() {
    // false NAND false
    let expected = true;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Constant, false);
    check_nand("false NAND false", expected, a, b, 0, 0, 0, 0);

    // false NAND true
    let expected = true;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Constant, true);
    check_nand("false NAND true", expected, a, b, 0, 0, 0, 0);

    // true NAND false
    let expected = true;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Constant, false);
    check_nand("true NAND false", expected, a, b, 0, 0, 0, 0);

    // true NAND true
    let expected = false;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Constant, true);
    check_nand("true NAND true", expected, a, b, 0, 0, 0, 0);
}

#[test]
fn test_private_nand_public() {
    // false NAND false
    let expected = true;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Public, false);
    check_nand("false NAND false", expected, a, b, 0, 0, 1, 1);

    // false NAND true
    let expected = true;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Public, true);
    check_nand("false NAND true", expected, a, b, 0, 0, 1, 1);

    // true NAND false
    let expected = true;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Public, false);
    check_nand("true NAND false", expected, a, b, 0, 0, 1, 1);

    // true NAND true
    let expected = false;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Public, true);
    check_nand("true NAND true", expected, a, b, 0, 0, 1, 1);
}

#[test]
fn test_private_nand_private() {
    // false NAND false
    let expected = true;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Private, false);
    check_nand("false NAND false", expected, a, b, 0, 0, 1, 1);

    // false NAND true
    let expected = true;
    let a = Boolean::new(Mode::Private, false);
    let b = Boolean::new(Mode::Private, true);
    check_nand("false NAND true", expected, a, b, 0, 0, 1, 1);

    // true NAND false
    let expected = true;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Private, false);
    check_nand("true NAND false", expected, a, b, 0, 0, 1, 1);

    // true NAND true
    let expected = false;
    let a = Boolean::new(Mode::Private, true);
    let b = Boolean::new(Mode::Private, true);
    check_nand("true NAND true", expected, a, b, 0, 0, 1, 1);
}
