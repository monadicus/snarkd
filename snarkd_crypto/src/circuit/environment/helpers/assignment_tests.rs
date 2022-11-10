/// Compute 2^EXPONENT - 1, in a purposefully constraint-inefficient manner for testing.
fn create_example_circuit<E: Environment>() -> Field<E> {
    let one = snarkvm_console_types::Field::<E::Network>::one();
    let two = one + one;

    const EXPONENT: u64 = 64;

    // Compute 2^EXPONENT - 1, in a purposefully constraint-inefficient manner for testing.
    let mut candidate = Field::<E>::new(Mode::Public, one);
    let mut accumulator = Field::new(Mode::Private, two);
    for _ in 0..EXPONENT {
        candidate += &accumulator;
        accumulator *= Field::new(Mode::Private, two);
    }

    assert_eq!(
        (accumulator - Field::one()).eject_value(),
        candidate.eject_value()
    );
    assert_eq!(2, E::num_public());
    assert_eq!(2 * EXPONENT + 1, E::num_private());
    assert_eq!(EXPONENT, E::num_constraints());
    assert!(E::is_satisfied());

    candidate
}

#[test]
fn test_constraint_converter() {
    let _candidate_output = create_example_circuit::<Circuit>();
    let assignment = Circuit::eject_assignment_and_reset();
    assert_eq!(0, Circuit::num_constants());
    assert_eq!(1, Circuit::num_public());
    assert_eq!(0, Circuit::num_private());
    assert_eq!(0, Circuit::num_constraints());

    let mut cs = snarkvm_r1cs::TestConstraintSystem::new();
    assignment.generate_constraints(&mut cs).unwrap();
    {
        use snarkvm_r1cs::ConstraintSystem;
        assert_eq!(
            assignment.num_public() + 1,
            cs.num_public_variables() as u64
        );
        assert_eq!(assignment.num_private(), cs.num_private_variables() as u64);
        assert_eq!(assignment.num_constraints(), cs.num_constraints() as u64);
        assert!(cs.is_satisfied());
    }
}

#[test]
fn test_marlin() {
    let _candidate_output = create_example_circuit::<Circuit>();
    let assignment = Circuit::eject_assignment_and_reset();
    assert_eq!(0, Circuit::num_constants());
    assert_eq!(1, Circuit::num_public());
    assert_eq!(0, Circuit::num_private());
    assert_eq!(0, Circuit::num_constraints());

    // Marlin setup, prove, and verify.

    use snarkvm_algorithms::{
        crypto_hash::PoseidonSponge,
        snark::marlin::{ahp::AHPForR1CS, MarlinHidingMode, MarlinSNARK},
    };
    use snarkvm_curves::bls12_377::{Bls12_377, Fq};
    use snarkvm_utilities::rand::TestRng;

    type FS = PoseidonSponge<Fq, 2, 1>;
    type MarlinInst = MarlinSNARK<Bls12_377, FS, MarlinHidingMode>;

    let rng = &mut TestRng::default();

    let max_degree = AHPForR1CS::<Fr, MarlinHidingMode>::max_degree(200, 200, 300).unwrap();
    let universal_srs = MarlinInst::universal_setup(&max_degree).unwrap();
    let fs_pp = FS::sample_parameters();

    let (index_pk, index_vk) = MarlinInst::circuit_setup(&universal_srs, &assignment).unwrap();
    println!("Called circuit setup");

    let proof = MarlinInst::prove(&fs_pp, &index_pk, &assignment, rng).unwrap();
    println!("Called prover");

    let one = <Circuit as Environment>::BaseField::one();
    assert!(MarlinInst::verify(&fs_pp, &index_vk, [one, one], &proof).unwrap());
    println!("Called verifier");
    println!("\nShould not verify (i.e. verifier messages should print below):");
    assert!(!MarlinInst::verify(&fs_pp, &index_vk, [one, one + one], &proof).unwrap());
}
