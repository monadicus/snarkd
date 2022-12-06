
use super::*;
use snarkvm_circuit_environment::Circuit;

const ITERATIONS: u64 = 100;

fn check_is_less_than(
    name: &str,
    expected: bool,
    a: &Field<Circuit>,
    b: &Field<Circuit>,
    num_constants: u64,
    num_public: u64,
    num_private: u64,
    num_constraints: u64,
) {
    // Perform the less than comparison.
    Circuit::scope(name, || {
        let candidate = a.is_less_than(b);
        assert_eq!(expected, candidate.eject_value());
        match (a.eject_mode(), b.eject_mode()) {
            (Mode::Constant, Mode::Constant) => {
                assert_scope!(num_constants, num_public, num_private, num_constraints)
            }
            (_, Mode::Constant) | (Mode::Constant, _) => {
                assert_scope!(<=num_constants, <=num_public, <=num_private, <=num_constraints)
            }
            _ => assert_scope!(num_constants, num_public, num_private, num_constraints),
        }
    });
    Circuit::reset();
}

fn run_test(
    mode_a: Mode,
    mode_b: Mode,
    num_constants: u64,
    num_public: u64,
    num_private: u64,
    num_constraints: u64,
) {
    let mut rng = TestRng::default();

    for i in 0..ITERATIONS {
        let first = Uniform::rand(&mut rng);
        let second = Uniform::rand(&mut rng);

        let a = Field::<Circuit>::new(mode_a, first);
        let b = Field::<Circuit>::new(mode_b, second);
        let expected = first < second;
        let name = format!("{} {} {}", mode_a, mode_b, i);
        check_is_less_than(
            &name,
            expected,
            &a,
            &b,
            num_constants,
            num_public,
            num_private,
            num_constraints,
        );

        // Check `first` is not less than `first`.
        let a = Field::<Circuit>::new(mode_a, first);
        let b = Field::<Circuit>::new(mode_b, first);
        check_is_less_than(
            "first !< first",
            false,
            &a,
            &b,
            num_constants,
            num_public,
            num_private,
            num_constraints,
        );
    }
}

#[test]
fn test_constant_is_less_than_constant() {
    run_test(Mode::Constant, Mode::Constant, 0, 0, 0, 0);
}

#[test]
fn test_constant_is_less_than_public() {
    run_test(Mode::Constant, Mode::Public, 253, 0, 506, 507);
}

#[test]
fn test_constant_is_less_than_private() {
    run_test(Mode::Constant, Mode::Private, 253, 0, 506, 507);
}

#[test]
fn test_public_is_less_than_constant() {
    run_test(Mode::Public, Mode::Constant, 253, 0, 506, 507);
}

#[test]
fn test_public_is_less_than_public() {
    run_test(Mode::Public, Mode::Public, 0, 0, 1012, 1014);
}

#[test]
fn test_public_is_less_than_private() {
    run_test(Mode::Public, Mode::Private, 0, 0, 1012, 1014);
}

#[test]
fn test_private_is_less_than_constant() {
    run_test(Mode::Private, Mode::Constant, 253, 0, 506, 507);
}

#[test]
fn test_private_is_less_than_public() {
    run_test(Mode::Private, Mode::Public, 0, 0, 1012, 1014);
}

#[test]
fn test_private_is_less_than_private() {
    run_test(Mode::Private, Mode::Private, 0, 0, 1012, 1014);
}
