use crate::{
    assert_scope,
    bls12_377::{Field, Fp},
    circuit::{
        circuit::Circuit,
        helpers::Mode,
        traits::{Eject, Inject},
        types::Boolean,
        Environment,
    },
};

#[test]
fn test_new_constant() {
    Circuit::scope("test_new_constant", || {
        let candidate = Boolean::new(Mode::Constant, false);
        assert!(!candidate.eject_value()); // false
        assert_scope!(1, 0, 0, 0);

        let candidate = Boolean::new(Mode::Constant, true);
        assert!(candidate.eject_value()); // true
        assert_scope!(2, 0, 0, 0);
    });
}

#[test]
fn test_new_public() {
    Circuit::scope("test_new_public", || {
        let candidate = Boolean::new(Mode::Public, false);
        assert!(!candidate.eject_value()); // false
        assert_scope!(0, 1, 0, 1);

        let candidate = Boolean::new(Mode::Public, true);
        assert!(candidate.eject_value()); // true
        assert_scope!(0, 2, 0, 2);
    });
}

#[test]
fn test_new_private() {
    Circuit::scope("test_new_private", || {
        let candidate = Boolean::new(Mode::Private, false);
        assert!(!candidate.eject_value()); // false
        assert_scope!(0, 0, 1, 1);

        let candidate = Boolean::new(Mode::Private, true);
        assert!(candidate.eject_value()); // true
        assert_scope!(0, 0, 2, 2);
    });
}

#[test]
fn test_new_fail() {
    let one = Fp::ONE;
    let two = one + one;
    {
        let candidate = Circuit::new_variable(Mode::Constant, two);

        // Ensure `a` is either 0 or 1:
        // (1 - a) * a = 0
        assert!(std::panic::catch_unwind(|| Circuit::enforce(|| (
            Circuit::one() - &candidate,
            candidate,
            Circuit::zero()
        )))
        .is_err());
        assert_eq!(0, Circuit::num_constraints());

        Circuit::reset();
    }
    {
        let candidate = Circuit::new_variable(Mode::Public, two);

        // Ensure `a` is either 0 or 1:
        // (1 - a) * a = 0
        Circuit::enforce(|| (Circuit::one() - &candidate, candidate, Circuit::zero()));
        assert!(!Circuit::is_satisfied());

        Circuit::reset();
    }
    {
        let candidate = Circuit::new_variable(Mode::Private, two);

        // Ensure `a` is either 0 or 1:
        // (1 - a) * a = 0
        Circuit::enforce(|| (Circuit::one() - &candidate, candidate, Circuit::zero()));
        assert!(!Circuit::is_satisfied());

        Circuit::reset();
    }
}

#[test]
fn test_display() {
    let candidate = Boolean::new(Mode::Constant, false);
    assert_eq!("false.constant", format!("{}", candidate));

    let candidate = Boolean::new(Mode::Constant, true);
    assert_eq!("true.constant", format!("{}", candidate));

    let candidate = Boolean::new(Mode::Public, false);
    assert_eq!("false.public", format!("{}", candidate));

    let candidate = Boolean::new(Mode::Public, true);
    assert_eq!("true.public", format!("{}", candidate));

    let candidate = Boolean::new(Mode::Private, false);
    assert_eq!("false.private", format!("{}", candidate));

    let candidate = Boolean::new(Mode::Private, true);
    assert_eq!("true.private", format!("{}", candidate));
}
