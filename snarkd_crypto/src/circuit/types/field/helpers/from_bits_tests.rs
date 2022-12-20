
use super::*;
use snarkvm_circuit_environment::Circuit;

const ITERATIONS: u64 = 100;

fn check_from_bits_le(
    mode: Mode,
    num_constants: u64,
    num_public: u64,
    num_private: u64,
    num_constraints: u64,
) {
    let mut rng = TestRng::default();

    for i in 0..ITERATIONS {
        // Sample a random element.
        let expected = Uniform::rand(&mut rng);
        let given_bits = Field::<Circuit>::new(mode, expected).to_bits_le();
        let expected_size_in_bits = given_bits.len();

        Circuit::scope(format!("{mode} {i}"), || {
            let candidate = Field::<Circuit>::from_bits_le(&given_bits);
            assert_eq!(expected, candidate.eject_value());
            assert_eq!(
                expected_size_in_bits,
                candidate.bits_le.get().expect("Caching failed").len()
            );
            assert_scope!(num_constants, num_public, num_private, num_constraints);

            // Ensure a subsequent call to `to_bits_le` does not incur additional costs.
            let candidate_bits = candidate.to_bits_le();
            assert_eq!(expected_size_in_bits, candidate_bits.len());
            assert_scope!(num_constants, num_public, num_private, num_constraints);
        });

        // Add excess zero bits.
        let candidate = vec![given_bits, vec![Boolean::new(mode, false); i as usize]].concat();

        Circuit::scope(&format!("Excess {} {}", mode, i), || {
            let candidate = Field::<Circuit>::from_bits_le(&candidate);
            assert_eq!(expected, candidate.eject_value());
            assert_eq!(
                expected_size_in_bits,
                candidate.bits_le.get().expect("Caching failed").len()
            );
            match mode.is_constant() {
                true => assert_scope!(num_constants, num_public, num_private, num_constraints),
                // `num_private` gets 1 free excess bit, then is incremented by one for each excess bit.
                // `num_constraints` is incremented by one for each excess bit.
                false => {
                    assert_scope!(
                        num_constants,
                        num_public,
                        num_private + i.saturating_sub(1),
                        num_constraints + i
                    )
                }
            };
        });
    }
}

fn check_from_bits_be(
    mode: Mode,
    num_constants: u64,
    num_public: u64,
    num_private: u64,
    num_constraints: u64,
) {
    let mut rng = TestRng::default();

    for i in 0..ITERATIONS {
        // Sample a random element.
        let expected = Uniform::rand(&mut rng);
        let given_bits = Field::<Circuit>::new(mode, expected).to_bits_be();
        let expected_size_in_bits = given_bits.len();

        Circuit::scope(format!("{mode} {i}"), || {
            let candidate = Field::<Circuit>::from_bits_be(&given_bits);
            assert_eq!(expected, candidate.eject_value());
            assert_eq!(
                expected_size_in_bits,
                candidate.bits_le.get().expect("Caching failed").len()
            );
            assert_scope!(num_constants, num_public, num_private, num_constraints);

            // Ensure a subsequent call to `to_bits_be` does not incur additional costs.
            let candidate_bits = candidate.to_bits_be();
            assert_eq!(expected_size_in_bits, candidate_bits.len());
            assert_scope!(num_constants, num_public, num_private, num_constraints);
        });

        // Add excess zero bits.
        let candidate = vec![vec![Boolean::new(mode, false); i as usize], given_bits].concat();

        Circuit::scope(&format!("Excess {} {}", mode, i), || {
            let candidate = Field::<Circuit>::from_bits_be(&candidate);
            assert_eq!(expected, candidate.eject_value());
            assert_eq!(
                expected_size_in_bits,
                candidate.bits_le.get().expect("Caching failed").len()
            );
            match mode.is_constant() {
                true => assert_scope!(num_constants, num_public, num_private, num_constraints),
                // `num_private` gets 1 free excess bit, then is incremented by one for each excess bit.
                // `num_constraints` is incremented by one for each excess bit.
                false => {
                    assert_scope!(
                        num_constants,
                        num_public,
                        num_private + i.saturating_sub(1),
                        num_constraints + i
                    )
                }
            };
        });
    }
}

#[test]
fn test_from_bits_le_constant() {
    check_from_bits_le(Mode::Constant, 0, 0, 0, 0);
}

#[test]
fn test_from_bits_le_public() {
    check_from_bits_le(Mode::Public, 0, 0, 252, 253);
}

#[test]
fn test_from_bits_le_private() {
    check_from_bits_le(Mode::Private, 0, 0, 252, 253);
}

#[test]
fn test_from_bits_be_constant() {
    check_from_bits_be(Mode::Constant, 0, 0, 0, 0);
}

#[test]
fn test_from_bits_be_public() {
    check_from_bits_be(Mode::Public, 0, 0, 252, 253);
}

#[test]
fn test_from_bits_be_private() {
    check_from_bits_be(Mode::Private, 0, 0, 252, 253);
}
