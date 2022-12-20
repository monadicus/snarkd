use super::*;
use snarkvm_circuit_environment::Circuit;

const ITERATIONS: u64 = 100;

fn check_to_bits_le(mode: Mode) {
    let expected_number_of_bits =
        console::Field::<<Circuit as Environment>::Network>::size_in_bits();

    let mut rng = TestRng::default();

    for i in 0..ITERATIONS {
        // Sample a random element.
        let expected = Uniform::rand(&mut rng);
        let candidate = Field::<Circuit>::new(mode, expected);

        Circuit::scope(&format!("{} {}", mode, i), || {
            let candidate_bits = candidate.to_bits_le();
            assert_eq!(expected_number_of_bits, candidate_bits.len());
            for (expected_bit, candidate_bit) in
                expected.to_bits_le().iter().zip_eq(&candidate_bits)
            {
                assert_eq!(*expected_bit, candidate_bit.eject_value());
            }
            assert_count!(ToBits<Boolean>() => Field, &mode);
            assert_output_mode!(ToBits<Boolean>() => Field, &mode, candidate_bits);

            // Ensure a second call to `to_bits_le` does not incur additional costs.
            let candidate_bits = candidate.to_bits_le();
            assert_eq!(expected_number_of_bits, candidate_bits.len());
            assert_count!(ToBits<Boolean>() => Field, &mode);
            assert_output_mode!(ToBits<Boolean>() => Field, &mode, candidate_bits);
        });
    }
}

fn check_to_bits_be(mode: Mode) {
    let expected_number_of_bits =
        console::Field::<<Circuit as Environment>::Network>::size_in_bits();

    let mut rng = TestRng::default();

    for i in 0..ITERATIONS {
        // Sample a random element.
        let expected = Uniform::rand(&mut rng);
        let candidate = Field::<Circuit>::new(mode, expected);

        Circuit::scope(&format!("{} {}", mode, i), || {
            let candidate_bits = candidate.to_bits_be();
            assert_eq!(expected_number_of_bits, candidate_bits.len());
            for (expected_bit, candidate_bit) in
                expected.to_bits_be().iter().zip_eq(&candidate_bits)
            {
                assert_eq!(*expected_bit, candidate_bit.eject_value());
            }
            assert_count!(ToBits<Boolean>() => Field, &mode);
            assert_output_mode!(ToBits<Boolean>() => Field, &mode, candidate_bits);

            // Ensure a second call to `to_bits_be` does not incur additional costs.
            let candidate_bits = candidate.to_bits_be();
            assert_eq!(expected_number_of_bits, candidate_bits.len());
            assert_count!(ToBits<Boolean>() => Field, &mode);
            assert_output_mode!(ToBits<Boolean>() => Field, &mode, candidate_bits);
        });
    }
}

#[test]
fn test_to_bits_le_constant() {
    check_to_bits_le(Mode::Constant);
}

#[test]
fn test_to_bits_le_public() {
    check_to_bits_le(Mode::Public);
}

#[test]
fn test_to_bits_le_private() {
    check_to_bits_le(Mode::Private);
}

#[test]
fn test_to_bits_be_constant() {
    check_to_bits_be(Mode::Constant);
}

#[test]
fn test_to_bits_be_public() {
    check_to_bits_be(Mode::Public);
}

#[test]
fn test_to_bits_be_private() {
    check_to_bits_be(Mode::Private);
}

#[test]
fn test_one() {
    /// Checks that the field element, when converted to little-endian bits, is well-formed.
    fn check_bits_le(candidate: Field<Circuit>) {
        for (i, bit) in candidate.to_bits_le().iter().enumerate() {
            match i == 0 {
                true => assert!(bit.eject_value()),
                false => assert!(!bit.eject_value()),
            }
        }
    }

    /// Checks that the field element, when converted to big-endian bits, is well-formed.
    fn check_bits_be(candidate: Field<Circuit>) {
        for (i, bit) in candidate.to_bits_be().iter().rev().enumerate() {
            match i == 0 {
                true => assert!(bit.eject_value()),
                false => assert!(!bit.eject_value()),
            }
        }
    }

    let one = console::Field::<<Circuit as Environment>::Network>::one();

    // Constant
    check_bits_le(Field::<Circuit>::new(Mode::Constant, one));
    check_bits_be(Field::<Circuit>::new(Mode::Constant, one));
    // Public
    check_bits_le(Field::<Circuit>::new(Mode::Public, one));
    check_bits_be(Field::<Circuit>::new(Mode::Public, one));
    // Private
    check_bits_le(Field::<Circuit>::new(Mode::Private, one));
    check_bits_be(Field::<Circuit>::new(Mode::Private, one));
}
