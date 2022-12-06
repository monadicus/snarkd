use crate::{
    assert_scope,
    circuit::{
        circuit::Circuit,
        helpers::Mode,
        traits::{Eject, Inject, Ternary},
        types::Boolean,
        Environment,
    },
};

#[allow(clippy::too_many_arguments)]
fn check_ternary(
    name: &str,
    expected: bool,
    condition: Boolean,
    a: Boolean,
    b: Boolean,
    num_constants: u64,
    num_public: u64,
    num_private: u64,
    num_constraints: u64,
) {
    Circuit::scope(name, || {
        let case = format!(
            "({} ? {} : {})",
            condition.eject_value(),
            a.eject_value(),
            b.eject_value()
        );
        let candidate = Boolean::ternary(&condition, &a, &b);
        assert_eq!(expected, candidate.eject_value(), "{case}");
        assert_scope!(num_constants, num_public, num_private, num_constraints);
    });
}

fn run_test(
    mode_condition: Mode,
    mode_a: Mode,
    mode_b: Mode,
    num_constants: u64,
    num_public: u64,
    num_private: u64,
    num_constraints: u64,
) {
    for flag in [true, false] {
        for first in [true, false] {
            for second in [true, false] {
                let condition = Boolean::new(mode_condition, flag);
                let a = Boolean::new(mode_a, first);
                let b = Boolean::new(mode_b, second);

                let name = format!("{} ? {} : {}", mode_condition, mode_a, mode_b);
                check_ternary(
                    &name,
                    if flag { first } else { second },
                    condition,
                    a,
                    b,
                    num_constants,
                    num_public,
                    num_private,
                    num_constraints,
                );
            }
        }
    }
}

#[test]
fn test_if_constant_then_constant_else_constant() {
    run_test(Mode::Constant, Mode::Constant, Mode::Constant, 0, 0, 0, 0);
}

#[test]
fn test_if_constant_then_constant_else_public() {
    run_test(Mode::Constant, Mode::Constant, Mode::Public, 0, 0, 0, 0);
}

#[test]
fn test_if_constant_then_constant_else_private() {
    run_test(Mode::Constant, Mode::Constant, Mode::Private, 0, 0, 0, 0);
}

#[test]
fn test_if_constant_then_public_else_constant() {
    run_test(Mode::Constant, Mode::Public, Mode::Constant, 0, 0, 0, 0);
}

#[test]
fn test_if_constant_then_public_else_public() {
    run_test(Mode::Constant, Mode::Public, Mode::Public, 0, 0, 0, 0);
}

#[test]
fn test_if_constant_then_public_else_private() {
    run_test(Mode::Constant, Mode::Public, Mode::Private, 0, 0, 0, 0);
}

#[test]
fn test_if_constant_then_private_else_constant() {
    run_test(Mode::Constant, Mode::Private, Mode::Constant, 0, 0, 0, 0);
}

#[test]
fn test_if_constant_then_private_else_public() {
    run_test(Mode::Constant, Mode::Private, Mode::Public, 0, 0, 0, 0);
}

#[test]
fn test_if_constant_then_private_else_private() {
    run_test(Mode::Constant, Mode::Private, Mode::Private, 0, 0, 0, 0);
}

#[test]
fn test_if_public_then_constant_else_constant() {
    run_test(Mode::Public, Mode::Constant, Mode::Constant, 0, 0, 0, 0);
}

#[test]
fn test_if_public_then_constant_else_public() {
    run_test(Mode::Public, Mode::Constant, Mode::Public, 0, 0, 1, 1);
}

#[test]
fn test_if_public_then_constant_else_private() {
    run_test(Mode::Public, Mode::Constant, Mode::Private, 0, 0, 1, 1);
}

#[test]
fn test_if_public_then_public_else_constant() {
    run_test(Mode::Public, Mode::Public, Mode::Constant, 0, 0, 1, 1);
}

#[test]
fn test_if_public_then_public_else_public() {
    run_test(Mode::Public, Mode::Public, Mode::Public, 0, 0, 1, 1);
}

#[test]
fn test_if_public_then_public_else_private() {
    run_test(Mode::Public, Mode::Public, Mode::Private, 0, 0, 1, 1);
}

#[test]
fn test_if_public_then_private_else_constant() {
    run_test(Mode::Public, Mode::Private, Mode::Constant, 0, 0, 1, 1);
}

#[test]
fn test_if_public_then_private_else_public() {
    run_test(Mode::Public, Mode::Private, Mode::Public, 0, 0, 1, 1);
}

#[test]
fn test_if_public_then_private_else_private() {
    run_test(Mode::Public, Mode::Private, Mode::Private, 0, 0, 1, 1);
}

#[test]
fn test_if_private_then_constant_else_constant() {
    run_test(Mode::Private, Mode::Constant, Mode::Constant, 0, 0, 0, 0);
}

#[test]
fn test_if_private_then_constant_else_public() {
    run_test(Mode::Private, Mode::Constant, Mode::Public, 0, 0, 1, 1);
}

#[test]
fn test_if_private_then_constant_else_private() {
    run_test(Mode::Private, Mode::Constant, Mode::Private, 0, 0, 1, 1);
}

#[test]
fn test_if_private_then_public_else_constant() {
    run_test(Mode::Private, Mode::Public, Mode::Constant, 0, 0, 1, 1);
}

#[test]
fn test_if_private_then_public_else_public() {
    run_test(Mode::Private, Mode::Public, Mode::Public, 0, 0, 1, 1);
}

#[test]
fn test_if_private_then_public_else_private() {
    run_test(Mode::Private, Mode::Public, Mode::Private, 0, 0, 1, 1);
}

#[test]
fn test_if_private_then_private_else_constant() {
    run_test(Mode::Private, Mode::Private, Mode::Constant, 0, 0, 1, 1);
}

#[test]
fn test_if_private_then_private_else_public() {
    run_test(Mode::Private, Mode::Private, Mode::Public, 0, 0, 1, 1);
}

#[test]
fn test_if_private_then_private_else_private() {
    run_test(Mode::Private, Mode::Private, Mode::Private, 0, 0, 1, 1);
}
