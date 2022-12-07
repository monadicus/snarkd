use crate::{assert_scope, circuit::traits::Adder};

use super::*;

#[allow(clippy::too_many_arguments)]
fn check_adder(
    name: &str,
    expected_sum: bool,
    expected_carry: bool,
    a: Boolean,
    b: Boolean,
    c: Boolean,
    num_constants: u64,
    num_public: u64,
    num_private: u64,
    num_constraints: u64,
) {
    Circuit::scope(name, || {
        let case = format!(
            "({} ADD {} WITH {})",
            a.eject_value(),
            b.eject_value(),
            c.eject_value()
        );
        let (candidate_sum, candidate_carry) = a.adder(&b, &c);
        assert_eq!(expected_sum, candidate_sum.eject_value(), "SUM {}", case);
        assert_eq!(
            expected_carry,
            candidate_carry.eject_value(),
            "CARRY {}",
            case
        );
        assert_scope!(
            case,
            num_constants,
            num_public,
            num_private,
            num_constraints
        );
    });
}

fn check_false_add_false_with_false(
    mode_a: Mode,
    mode_b: Mode,
    mode_c: Mode,
    num_constants: u64,
    num_public: u64,
    num_private: u64,
    num_constraints: u64,
) {
    // false ADD false WITH false => (false, false)
    let expected_sum = false;
    let expected_carry = false;
    let a = Boolean::new(mode_a, false);
    let b = Boolean::new(mode_b, false);
    let c = Boolean::new(mode_c, false);
    check_adder(
        "false ADD false WITH false",
        expected_sum,
        expected_carry,
        a,
        b,
        c,
        num_constants,
        num_public,
        num_private,
        num_constraints,
    );
}

fn check_false_add_false_with_true(
    mode_a: Mode,
    mode_b: Mode,
    mode_c: Mode,
    num_constants: u64,
    num_public: u64,
    num_private: u64,
    num_constraints: u64,
) {
    // false ADD false WITH true => (true, false)
    let expected_sum = true;
    let expected_carry = false;
    let a = Boolean::new(mode_a, false);
    let b = Boolean::new(mode_b, false);
    let c = Boolean::new(mode_c, true);
    check_adder(
        "false ADD false WITH true",
        expected_sum,
        expected_carry,
        a,
        b,
        c,
        num_constants,
        num_public,
        num_private,
        num_constraints,
    );
}

fn check_false_add_true_with_false(
    mode_a: Mode,
    mode_b: Mode,
    mode_c: Mode,
    num_constants: u64,
    num_public: u64,
    num_private: u64,
    num_constraints: u64,
) {
    // false ADD true WITH false => (true, false)
    let expected_sum = true;
    let expected_carry = false;
    let a = Boolean::new(mode_a, false);
    let b = Boolean::new(mode_b, true);
    let c = Boolean::new(mode_c, false);
    check_adder(
        "false ADD true WITH false",
        expected_sum,
        expected_carry,
        a,
        b,
        c,
        num_constants,
        num_public,
        num_private,
        num_constraints,
    );
}

fn check_false_add_true_with_true(
    mode_a: Mode,
    mode_b: Mode,
    mode_c: Mode,
    num_constants: u64,
    num_public: u64,
    num_private: u64,
    num_constraints: u64,
) {
    // false ADD true WITH true => (false, true)
    let expected_sum = false;
    let expected_carry = true;
    let a = Boolean::new(mode_a, false);
    let b = Boolean::new(mode_b, true);
    let c = Boolean::new(mode_c, true);
    check_adder(
        "false ADD true WITH true",
        expected_sum,
        expected_carry,
        a,
        b,
        c,
        num_constants,
        num_public,
        num_private,
        num_constraints,
    );
}

fn check_true_add_false_with_false(
    mode_a: Mode,
    mode_b: Mode,
    mode_c: Mode,
    num_constants: u64,
    num_public: u64,
    num_private: u64,
    num_constraints: u64,
) {
    // true ADD false WITH false => (true, false)
    let expected_sum = true;
    let expected_carry = false;
    let a = Boolean::new(mode_a, true);
    let b = Boolean::new(mode_b, false);
    let c = Boolean::new(mode_c, false);
    check_adder(
        "true ADD false WITH false",
        expected_sum,
        expected_carry,
        a,
        b,
        c,
        num_constants,
        num_public,
        num_private,
        num_constraints,
    );
}

fn check_true_add_false_with_true(
    mode_a: Mode,
    mode_b: Mode,
    mode_c: Mode,
    num_constants: u64,
    num_public: u64,
    num_private: u64,
    num_constraints: u64,
) {
    // true ADD false WITH true => (false, true)
    let expected_sum = false;
    let expected_carry = true;
    let a = Boolean::new(mode_a, true);
    let b = Boolean::new(mode_b, false);
    let c = Boolean::new(mode_c, true);
    check_adder(
        "true ADD false WITH true",
        expected_sum,
        expected_carry,
        a,
        b,
        c,
        num_constants,
        num_public,
        num_private,
        num_constraints,
    );
}

fn check_true_add_true_with_false(
    mode_a: Mode,
    mode_b: Mode,
    mode_c: Mode,
    num_constants: u64,
    num_public: u64,
    num_private: u64,
    num_constraints: u64,
) {
    // true ADD true WITH false => (false, true)
    let expected_sum = false;
    let expected_carry = true;
    let a = Boolean::new(mode_a, true);
    let b = Boolean::new(mode_b, true);
    let c = Boolean::new(mode_c, false);
    check_adder(
        "true ADD true WITH false",
        expected_sum,
        expected_carry,
        a,
        b,
        c,
        num_constants,
        num_public,
        num_private,
        num_constraints,
    );
}

fn check_true_add_true_with_true(
    mode_a: Mode,
    mode_b: Mode,
    mode_c: Mode,
    num_constants: u64,
    num_public: u64,
    num_private: u64,
    num_constraints: u64,
) {
    // true ADD true WITH true => (true, true)
    let expected_sum = true;
    let expected_carry = true;
    let a = Boolean::new(mode_a, true);
    let b = Boolean::new(mode_b, true);
    let c = Boolean::new(mode_c, true);
    check_adder(
        "true ADD true WITH true",
        expected_sum,
        expected_carry,
        a,
        b,
        c,
        num_constants,
        num_public,
        num_private,
        num_constraints,
    );
}

#[test]
fn test_constant_add_constant_with_constant() {
    check_false_add_false_with_false(Mode::Constant, Mode::Constant, Mode::Constant, 0, 0, 0, 0);
    check_false_add_false_with_true(Mode::Constant, Mode::Constant, Mode::Constant, 0, 0, 0, 0);
    check_false_add_true_with_false(Mode::Constant, Mode::Constant, Mode::Constant, 0, 0, 0, 0);
    check_false_add_true_with_true(Mode::Constant, Mode::Constant, Mode::Constant, 0, 0, 0, 0);
    check_true_add_false_with_false(Mode::Constant, Mode::Constant, Mode::Constant, 0, 0, 0, 0);
    check_true_add_false_with_true(Mode::Constant, Mode::Constant, Mode::Constant, 0, 0, 0, 0);
    check_true_add_true_with_false(Mode::Constant, Mode::Constant, Mode::Constant, 0, 0, 0, 0);
    check_true_add_true_with_true(Mode::Constant, Mode::Constant, Mode::Constant, 0, 0, 0, 0);
}

#[test]
fn test_constant_add_constant_with_public() {
    check_false_add_false_with_false(Mode::Constant, Mode::Constant, Mode::Public, 0, 0, 0, 0);
    check_false_add_false_with_true(Mode::Constant, Mode::Constant, Mode::Public, 0, 0, 0, 0);
    check_false_add_true_with_false(Mode::Constant, Mode::Constant, Mode::Public, 0, 0, 0, 0);
    check_false_add_true_with_true(Mode::Constant, Mode::Constant, Mode::Public, 0, 0, 0, 0);
    check_true_add_false_with_false(Mode::Constant, Mode::Constant, Mode::Public, 0, 0, 0, 0);
    check_true_add_false_with_true(Mode::Constant, Mode::Constant, Mode::Public, 0, 0, 0, 0);
    check_true_add_true_with_false(Mode::Constant, Mode::Constant, Mode::Public, 0, 0, 0, 0);
    check_true_add_true_with_true(Mode::Constant, Mode::Constant, Mode::Public, 0, 0, 0, 0);
}

#[test]
fn test_constant_add_constant_with_private() {
    check_false_add_false_with_false(Mode::Constant, Mode::Constant, Mode::Private, 0, 0, 0, 0);
    check_false_add_false_with_true(Mode::Constant, Mode::Constant, Mode::Private, 0, 0, 0, 0);
    check_false_add_true_with_false(Mode::Constant, Mode::Constant, Mode::Private, 0, 0, 0, 0);
    check_false_add_true_with_true(Mode::Constant, Mode::Constant, Mode::Private, 0, 0, 0, 0);
    check_true_add_false_with_false(Mode::Constant, Mode::Constant, Mode::Private, 0, 0, 0, 0);
    check_true_add_false_with_true(Mode::Constant, Mode::Constant, Mode::Private, 0, 0, 0, 0);
    check_true_add_true_with_false(Mode::Constant, Mode::Constant, Mode::Private, 0, 0, 0, 0);
    check_true_add_true_with_true(Mode::Constant, Mode::Constant, Mode::Private, 0, 0, 0, 0);
}

#[test]
fn test_constant_add_public_with_constant() {
    check_false_add_false_with_false(Mode::Constant, Mode::Public, Mode::Constant, 0, 0, 0, 0);
    check_false_add_false_with_true(Mode::Constant, Mode::Public, Mode::Constant, 0, 0, 0, 0);
    check_false_add_true_with_false(Mode::Constant, Mode::Public, Mode::Constant, 0, 0, 0, 0);
    check_false_add_true_with_true(Mode::Constant, Mode::Public, Mode::Constant, 0, 0, 0, 0);
    check_true_add_false_with_false(Mode::Constant, Mode::Public, Mode::Constant, 0, 0, 0, 0);
    check_true_add_false_with_true(Mode::Constant, Mode::Public, Mode::Constant, 0, 0, 1, 1); // <- Differs
    check_true_add_true_with_false(Mode::Constant, Mode::Public, Mode::Constant, 0, 0, 0, 0);
    check_true_add_true_with_true(Mode::Constant, Mode::Public, Mode::Constant, 0, 0, 1, 1);
    // <- Differs
}

#[test]
fn test_constant_add_public_with_public() {
    check_false_add_false_with_false(Mode::Constant, Mode::Public, Mode::Public, 0, 0, 2, 2);
    check_false_add_false_with_true(Mode::Constant, Mode::Public, Mode::Public, 0, 0, 2, 2);
    check_false_add_true_with_false(Mode::Constant, Mode::Public, Mode::Public, 0, 0, 2, 2);
    check_false_add_true_with_true(Mode::Constant, Mode::Public, Mode::Public, 0, 0, 2, 2);
    check_true_add_false_with_false(Mode::Constant, Mode::Public, Mode::Public, 0, 0, 3, 3); // <- Differs
    check_true_add_false_with_true(Mode::Constant, Mode::Public, Mode::Public, 0, 0, 3, 3); // <- Differs
    check_true_add_true_with_false(Mode::Constant, Mode::Public, Mode::Public, 0, 0, 3, 3); // <- Differs
    check_true_add_true_with_true(Mode::Constant, Mode::Public, Mode::Public, 0, 0, 3, 3);
    // <- Differs
}

#[test]
fn test_constant_add_public_with_private() {
    check_false_add_false_with_false(Mode::Constant, Mode::Public, Mode::Private, 0, 0, 2, 2);
    check_false_add_false_with_true(Mode::Constant, Mode::Public, Mode::Private, 0, 0, 2, 2);
    check_false_add_true_with_false(Mode::Constant, Mode::Public, Mode::Private, 0, 0, 2, 2);
    check_false_add_true_with_true(Mode::Constant, Mode::Public, Mode::Private, 0, 0, 2, 2);
    check_true_add_false_with_false(Mode::Constant, Mode::Public, Mode::Private, 0, 0, 3, 3); // <- Differs
    check_true_add_false_with_true(Mode::Constant, Mode::Public, Mode::Private, 0, 0, 3, 3); // <- Differs
    check_true_add_true_with_false(Mode::Constant, Mode::Public, Mode::Private, 0, 0, 3, 3); // <- Differs
    check_true_add_true_with_true(Mode::Constant, Mode::Public, Mode::Private, 0, 0, 3, 3);
    // <- Differs
}

#[test]
fn test_constant_add_private_with_constant() {
    check_false_add_false_with_false(Mode::Constant, Mode::Private, Mode::Constant, 0, 0, 0, 0);
    check_false_add_false_with_true(Mode::Constant, Mode::Private, Mode::Constant, 0, 0, 0, 0);
    check_false_add_true_with_false(Mode::Constant, Mode::Private, Mode::Constant, 0, 0, 0, 0);
    check_false_add_true_with_true(Mode::Constant, Mode::Private, Mode::Constant, 0, 0, 0, 0);
    check_true_add_false_with_false(Mode::Constant, Mode::Private, Mode::Constant, 0, 0, 0, 0);
    check_true_add_false_with_true(Mode::Constant, Mode::Private, Mode::Constant, 0, 0, 1, 1); // <- Differs
    check_true_add_true_with_false(Mode::Constant, Mode::Private, Mode::Constant, 0, 0, 0, 0);
    check_true_add_true_with_true(Mode::Constant, Mode::Private, Mode::Constant, 0, 0, 1, 1);
    // <- Differs
}

#[test]
fn test_constant_add_private_with_public() {
    check_false_add_false_with_false(Mode::Constant, Mode::Private, Mode::Public, 0, 0, 2, 2);
    check_false_add_false_with_true(Mode::Constant, Mode::Private, Mode::Public, 0, 0, 2, 2);
    check_false_add_true_with_false(Mode::Constant, Mode::Private, Mode::Public, 0, 0, 2, 2);
    check_false_add_true_with_true(Mode::Constant, Mode::Private, Mode::Public, 0, 0, 2, 2);
    check_true_add_false_with_false(Mode::Constant, Mode::Private, Mode::Public, 0, 0, 3, 3); // <- Differs
    check_true_add_false_with_true(Mode::Constant, Mode::Private, Mode::Public, 0, 0, 3, 3); // <- Differs
    check_true_add_true_with_false(Mode::Constant, Mode::Private, Mode::Public, 0, 0, 3, 3); // <- Differs
    check_true_add_true_with_true(Mode::Constant, Mode::Private, Mode::Public, 0, 0, 3, 3);
    // <- Differs
}

#[test]
fn test_constant_add_private_with_private() {
    check_false_add_false_with_false(Mode::Constant, Mode::Private, Mode::Private, 0, 0, 2, 2);
    check_false_add_false_with_true(Mode::Constant, Mode::Private, Mode::Private, 0, 0, 2, 2);
    check_false_add_true_with_false(Mode::Constant, Mode::Private, Mode::Private, 0, 0, 2, 2);
    check_false_add_true_with_true(Mode::Constant, Mode::Private, Mode::Private, 0, 0, 2, 2);
    check_true_add_false_with_false(Mode::Constant, Mode::Private, Mode::Private, 0, 0, 3, 3); // <- Differs
    check_true_add_false_with_true(Mode::Constant, Mode::Private, Mode::Private, 0, 0, 3, 3); // <- Differs
    check_true_add_true_with_false(Mode::Constant, Mode::Private, Mode::Private, 0, 0, 3, 3); // <- Differs
    check_true_add_true_with_true(Mode::Constant, Mode::Private, Mode::Private, 0, 0, 3, 3);
    // <- Differs
}

#[test]
fn test_public_add_constant_with_constant() {
    check_false_add_false_with_false(Mode::Public, Mode::Constant, Mode::Constant, 0, 0, 0, 0);
    check_false_add_false_with_true(Mode::Public, Mode::Constant, Mode::Constant, 0, 0, 0, 0);
    check_false_add_true_with_false(Mode::Public, Mode::Constant, Mode::Constant, 0, 0, 0, 0);
    check_false_add_true_with_true(Mode::Public, Mode::Constant, Mode::Constant, 0, 0, 1, 1); // <- Differs
    check_true_add_false_with_false(Mode::Public, Mode::Constant, Mode::Constant, 0, 0, 0, 0);
    check_true_add_false_with_true(Mode::Public, Mode::Constant, Mode::Constant, 0, 0, 0, 0);
    check_true_add_true_with_false(Mode::Public, Mode::Constant, Mode::Constant, 0, 0, 0, 0);
    check_true_add_true_with_true(Mode::Public, Mode::Constant, Mode::Constant, 0, 0, 1, 1);
    // <- Differs
}

#[test]
fn test_public_add_constant_with_public() {
    check_false_add_false_with_false(Mode::Public, Mode::Constant, Mode::Public, 0, 0, 2, 2);
    check_false_add_false_with_true(Mode::Public, Mode::Constant, Mode::Public, 0, 0, 2, 2);
    check_false_add_true_with_false(Mode::Public, Mode::Constant, Mode::Public, 0, 0, 3, 3); // <- Differs
    check_false_add_true_with_true(Mode::Public, Mode::Constant, Mode::Public, 0, 0, 3, 3); // <- Differs
    check_true_add_false_with_false(Mode::Public, Mode::Constant, Mode::Public, 0, 0, 2, 2);
    check_true_add_false_with_true(Mode::Public, Mode::Constant, Mode::Public, 0, 0, 2, 2);
    check_true_add_true_with_false(Mode::Public, Mode::Constant, Mode::Public, 0, 0, 3, 3); // <- Differs
    check_true_add_true_with_true(Mode::Public, Mode::Constant, Mode::Public, 0, 0, 3, 3);
    // <- Differs
}

#[test]
fn test_public_add_constant_with_private() {
    check_false_add_false_with_false(Mode::Public, Mode::Constant, Mode::Private, 0, 0, 2, 2);
    check_false_add_false_with_true(Mode::Public, Mode::Constant, Mode::Private, 0, 0, 2, 2);
    check_false_add_true_with_false(Mode::Public, Mode::Constant, Mode::Private, 0, 0, 3, 3); // <- Differs
    check_false_add_true_with_true(Mode::Public, Mode::Constant, Mode::Private, 0, 0, 3, 3); // <- Differs
    check_true_add_false_with_false(Mode::Public, Mode::Constant, Mode::Private, 0, 0, 2, 2);
    check_true_add_false_with_true(Mode::Public, Mode::Constant, Mode::Private, 0, 0, 2, 2);
    check_true_add_true_with_false(Mode::Public, Mode::Constant, Mode::Private, 0, 0, 3, 3); // <- Differs
    check_true_add_true_with_true(Mode::Public, Mode::Constant, Mode::Private, 0, 0, 3, 3);
    // <- Differs
}

#[test]
fn test_public_add_public_with_constant() {
    check_false_add_false_with_false(Mode::Public, Mode::Public, Mode::Constant, 0, 0, 2, 2);
    check_false_add_false_with_true(Mode::Public, Mode::Public, Mode::Constant, 0, 0, 3, 3); // <- Differs
    check_false_add_true_with_false(Mode::Public, Mode::Public, Mode::Constant, 0, 0, 2, 2);
    check_false_add_true_with_true(Mode::Public, Mode::Public, Mode::Constant, 0, 0, 3, 3); // <- Differs
    check_true_add_false_with_false(Mode::Public, Mode::Public, Mode::Constant, 0, 0, 2, 2);
    check_true_add_false_with_true(Mode::Public, Mode::Public, Mode::Constant, 0, 0, 3, 3); // <- Differs
    check_true_add_true_with_false(Mode::Public, Mode::Public, Mode::Constant, 0, 0, 2, 2);
    check_true_add_true_with_true(Mode::Public, Mode::Public, Mode::Constant, 0, 0, 3, 3);
    // <- Differs
}

#[test]
fn test_public_add_public_with_public() {
    check_false_add_false_with_false(Mode::Public, Mode::Public, Mode::Public, 0, 0, 5, 5);
    check_false_add_false_with_true(Mode::Public, Mode::Public, Mode::Public, 0, 0, 5, 5);
    check_false_add_true_with_false(Mode::Public, Mode::Public, Mode::Public, 0, 0, 5, 5);
    check_false_add_true_with_true(Mode::Public, Mode::Public, Mode::Public, 0, 0, 5, 5);
    check_true_add_false_with_false(Mode::Public, Mode::Public, Mode::Public, 0, 0, 5, 5);
    check_true_add_false_with_true(Mode::Public, Mode::Public, Mode::Public, 0, 0, 5, 5);
    check_true_add_true_with_false(Mode::Public, Mode::Public, Mode::Public, 0, 0, 5, 5);
    check_true_add_true_with_true(Mode::Public, Mode::Public, Mode::Public, 0, 0, 5, 5);
}

#[test]
fn test_public_add_public_with_private() {
    check_false_add_false_with_false(Mode::Public, Mode::Public, Mode::Private, 0, 0, 5, 5);
    check_false_add_false_with_true(Mode::Public, Mode::Public, Mode::Private, 0, 0, 5, 5);
    check_false_add_true_with_false(Mode::Public, Mode::Public, Mode::Private, 0, 0, 5, 5);
    check_false_add_true_with_true(Mode::Public, Mode::Public, Mode::Private, 0, 0, 5, 5);
    check_true_add_false_with_false(Mode::Public, Mode::Public, Mode::Private, 0, 0, 5, 5);
    check_true_add_false_with_true(Mode::Public, Mode::Public, Mode::Private, 0, 0, 5, 5);
    check_true_add_true_with_false(Mode::Public, Mode::Public, Mode::Private, 0, 0, 5, 5);
    check_true_add_true_with_true(Mode::Public, Mode::Public, Mode::Private, 0, 0, 5, 5);
}

#[test]
fn test_public_add_private_with_constant() {
    check_false_add_false_with_false(Mode::Public, Mode::Private, Mode::Constant, 0, 0, 2, 2);
    check_false_add_false_with_true(Mode::Public, Mode::Private, Mode::Constant, 0, 0, 3, 3); // <- Differs
    check_false_add_true_with_false(Mode::Public, Mode::Private, Mode::Constant, 0, 0, 2, 2);
    check_false_add_true_with_true(Mode::Public, Mode::Private, Mode::Constant, 0, 0, 3, 3); // <- Differs
    check_true_add_false_with_false(Mode::Public, Mode::Private, Mode::Constant, 0, 0, 2, 2);
    check_true_add_false_with_true(Mode::Public, Mode::Private, Mode::Constant, 0, 0, 3, 3); // <- Differs
    check_true_add_true_with_false(Mode::Public, Mode::Private, Mode::Constant, 0, 0, 2, 2);
    check_true_add_true_with_true(Mode::Public, Mode::Private, Mode::Constant, 0, 0, 3, 3);
    // <- Differs
}

#[test]
fn test_public_add_private_with_public() {
    check_false_add_false_with_false(Mode::Public, Mode::Private, Mode::Public, 0, 0, 5, 5);
    check_false_add_false_with_true(Mode::Public, Mode::Private, Mode::Public, 0, 0, 5, 5);
    check_false_add_true_with_false(Mode::Public, Mode::Private, Mode::Public, 0, 0, 5, 5);
    check_false_add_true_with_true(Mode::Public, Mode::Private, Mode::Public, 0, 0, 5, 5);
    check_true_add_false_with_false(Mode::Public, Mode::Private, Mode::Public, 0, 0, 5, 5);
    check_true_add_false_with_true(Mode::Public, Mode::Private, Mode::Public, 0, 0, 5, 5);
    check_true_add_true_with_false(Mode::Public, Mode::Private, Mode::Public, 0, 0, 5, 5);
    check_true_add_true_with_true(Mode::Public, Mode::Private, Mode::Public, 0, 0, 5, 5);
}

#[test]
fn test_public_add_private_with_private() {
    check_false_add_false_with_false(Mode::Public, Mode::Private, Mode::Private, 0, 0, 5, 5);
    check_false_add_false_with_true(Mode::Public, Mode::Private, Mode::Private, 0, 0, 5, 5);
    check_false_add_true_with_false(Mode::Public, Mode::Private, Mode::Private, 0, 0, 5, 5);
    check_false_add_true_with_true(Mode::Public, Mode::Private, Mode::Private, 0, 0, 5, 5);
    check_true_add_false_with_false(Mode::Public, Mode::Private, Mode::Private, 0, 0, 5, 5);
    check_true_add_false_with_true(Mode::Public, Mode::Private, Mode::Private, 0, 0, 5, 5);
    check_true_add_true_with_false(Mode::Public, Mode::Private, Mode::Private, 0, 0, 5, 5);
    check_true_add_true_with_true(Mode::Public, Mode::Private, Mode::Private, 0, 0, 5, 5);
}
