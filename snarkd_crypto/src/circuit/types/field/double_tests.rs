
use super::*;
use snarkvm_circuit_environment::Circuit;

const ITERATIONS: u64 = 10_000;

fn check_double(name: &str, mode: Mode, rng: &mut TestRng) {
    for _ in 0..ITERATIONS {
        // Sample a random element.
        let given = Uniform::rand(rng);
        let candidate = Field::<Circuit>::new(mode, given);

        Circuit::scope(name, || {
            let result = candidate.double();
            assert_eq!(given.double(), result.eject_value());
            assert_count!(Double(Field) => Field, &mode);
            assert_output_mode!(Double(Field) => Field, &mode, result);
        });
    }
}

#[test]
fn test_double() {
    let mut rng = TestRng::default();

    check_double("Constant", Mode::Constant, &mut rng);
    check_double("Public", Mode::Public, &mut rng);
    check_double("Private", Mode::Private, &mut rng);
}

#[test]
fn test_0_double() {
    let zero = console::Field::<<Circuit as Environment>::Network>::zero();

    let candidate = Field::<Circuit>::new(Mode::Public, zero).double();
    assert_eq!(zero, candidate.eject_value());
}

#[test]
fn test_1_double() {
    let one = console::Field::<<Circuit as Environment>::Network>::one();
    let two = one + one;

    let candidate = Field::<Circuit>::new(Mode::Public, one).double();
    assert_eq!(two, candidate.eject_value());
}
