use crate::{
    bls12_377::{Field, Scalar},
    marlin::SNARK,
    r1cs::ConstraintSystem,
    utils::{PoseidonParameters, PoseidonSponge},
    ConstraintSynthesizer,
};
use anyhow::{anyhow, Result};
use rayon::prelude::*;
use std::ops::MulAssign;

#[derive(Copy, Clone)]
pub struct Circuit {
    pub a: Option<Scalar>,
    pub b: Option<Scalar>,
    pub num_constraints: usize,
    pub num_variables: usize,
}

impl ConstraintSynthesizer for Circuit {
    fn generate_constraints<CS: ConstraintSystem<Field = Scalar>>(
        &self,
        cs: &mut CS,
    ) -> Result<()> {
        let a = cs.alloc(|| "a", || self.a.ok_or(anyhow!("assignment missing")))?;
        let b = cs.alloc(|| "b", || self.b.ok_or(anyhow!("assignment missing")))?;
        let c = cs.alloc_input(
            || "c",
            || {
                let mut a = self.a.ok_or(anyhow!("assignment missing"))?;
                let b = self.b.ok_or(anyhow!("assignment missing"))?;

                a.mul_assign(&b);
                Ok(a)
            },
        )?;
        let d = cs.alloc_input(
            || "d",
            || {
                let mut a = self.a.ok_or(anyhow!("assignment missing"))?;
                let b = self.b.ok_or(anyhow!("assignment missing"))?;

                a.mul_assign(&b);
                a.mul_assign(&b);
                Ok(a)
            },
        )?;

        for i in 0..(self.num_variables - 3) {
            let _ = cs.alloc(
                || format!("var {}", i),
                || self.a.ok_or(anyhow!("assignment missing")),
            )?;
        }

        for i in 0..(self.num_constraints - 1) {
            cs.enforce(
                || format!("constraint {}", i),
                |lc| lc + a,
                |lc| lc + b,
                |lc| lc + c,
            );
        }
        cs.enforce(|| "constraint_final", |lc| lc + c, |lc| lc + b, |lc| lc + d);

        Ok(())
    }
}

mod marlin {
    use super::*;
    use crate::{
        bls12_377::{Field, Scalar},
        marlin::{
            AHPForR1CS, CircuitVerifyingKey, MarlinHidingMode, MarlinNonHidingMode, MarlinSNARK,
        },
    };

    use core::ops::MulAssign;

    type MarlinSonicInst = MarlinSNARK;

    type MarlinSonicPoswInst = MarlinSNARK;

    type FS = crate::utils::PoseidonSponge;

    macro_rules! impl_marlin_test {
        ($test_struct: ident, $marlin_inst: tt, $marlin_mode: expr) => {
            struct $test_struct {}
            impl $test_struct {
                pub(crate) fn test_circuit(num_constraints: usize, num_variables: usize) {
                    let rng = &mut rand::thread_rng();
                    let ahp = AHPForR1CS { mode: $marlin_mode };

                    let max_degree = ahp.max_degree(100, 25, 300).unwrap();
                    let universal_srs = $marlin_inst::universal_setup(max_degree, rng).unwrap();
                    let fs_parameters = PoseidonParameters::default();

                    let rng = &mut rand::thread_rng();
                    let a = Scalar::rand();
                    let b = Scalar::rand();
                    let mut c = a;
                    c.mul_assign(&b);
                    let mut d = c;
                    d.mul_assign(&b);

                    let circ = Circuit {
                        a: Some(a),
                        b: Some(b),
                        num_constraints,
                        num_variables,
                    };

                    let (index_pk, index_vk) =
                        $marlin_inst::circuit_setup(&universal_srs, &circ, $marlin_mode).unwrap();
                    println!("Called circuit setup");

                    let certificate =
                        $marlin_inst::prove_vk(&fs_parameters, &index_vk, &index_pk).unwrap();
                    assert!($marlin_inst::verify_vk(
                        &fs_parameters,
                        &circ,
                        &index_vk,
                        &certificate
                    )
                    .unwrap());

                    let snark = $marlin_inst { mode: $marlin_mode };

                    let proof = snark.prove(&fs_parameters, &index_pk, &circ, rng).unwrap();
                    println!("Called prover");

                    assert!(snark
                        .verify::<[Scalar], _>(&fs_parameters, &index_vk, [c, d], &proof)
                        .unwrap());
                    println!("Called verifier");
                    println!("\nShould not verify (i.e. verifier messages should print below):");
                    assert!(!snark
                        .verify::<[Scalar], _>(&fs_parameters, &index_vk, [a, a], &proof)
                        .unwrap());

                    let rng = &mut rand::thread_rng();
                    for batch_size in (0..5).map(|i| 2usize.pow(i)) {
                        let (circuit_batch, input_batch): (Vec<_>, Vec<_>) = (0..batch_size)
                            .map(|_| {
                                let a = Scalar::rand();
                                let b = Scalar::rand();
                                let mut c = a;
                                c.mul_assign(&b);
                                let mut d = c;
                                d.mul_assign(&b);

                                let circ = Circuit {
                                    a: Some(a),
                                    b: Some(b),
                                    num_constraints,
                                    num_variables,
                                };
                                (circ, [c, d])
                            })
                            .unzip();
                        let (index_pk, index_vk) = $marlin_inst::circuit_setup(
                            &universal_srs,
                            &circuit_batch[0],
                            $marlin_mode,
                        )
                        .unwrap();
                        println!("Called circuit setup");

                        let snark = $marlin_inst { mode: $marlin_mode };
                        let proof = snark
                            .prove_batch(&fs_parameters, &index_pk, &circuit_batch, rng)
                            .unwrap();
                        println!("Called prover");

                        assert!(
                            snark
                                .verify_batch::<[Scalar], _>(
                                    &fs_parameters,
                                    &index_vk,
                                    &input_batch,
                                    &proof
                                )
                                .unwrap(),
                            "Batch verification failed with {batch_size} inputs"
                        );
                        println!("Called verifier");
                        println!(
                            "\nShould not verify (i.e. verifier messages should print below):"
                        );
                        assert!(!snark
                            .verify_batch::<[Scalar], _>(
                                &fs_parameters,
                                &index_vk,
                                &vec![[Scalar::rand(), Scalar::rand()]; batch_size],
                                &proof
                            )
                            .unwrap());
                    }
                }
            }
        };
    }

    impl_marlin_test!(SonicPCTest, MarlinSonicInst, true);
    impl_marlin_test!(SonicPCPoswTest, MarlinSonicPoswInst, false);

    #[test]
    fn prove_and_verify_with_tall_matrix_big() {
        let num_constraints = 100;
        let num_variables = 25;

        SonicPCTest::test_circuit(num_constraints, num_variables);
        SonicPCPoswTest::test_circuit(num_constraints, num_variables);
    }

    #[test]
    fn prove_and_verify_with_tall_matrix_small() {
        let num_constraints = 26;
        let num_variables = 25;

        SonicPCTest::test_circuit(num_constraints, num_variables);
        SonicPCPoswTest::test_circuit(num_constraints, num_variables);
    }

    #[test]
    fn prove_and_verify_with_squat_matrix_big() {
        let num_constraints = 25;
        let num_variables = 100;

        SonicPCTest::test_circuit(num_constraints, num_variables);
        SonicPCPoswTest::test_circuit(num_constraints, num_variables);
    }

    #[test]
    fn prove_and_verify_with_squat_matrix_small() {
        let num_constraints = 25;
        let num_variables = 26;

        SonicPCTest::test_circuit(num_constraints, num_variables);
        SonicPCPoswTest::test_circuit(num_constraints, num_variables);
    }

    #[test]
    fn prove_and_verify_with_square_matrix() {
        let num_constraints = 25;
        let num_variables = 25;

        SonicPCTest::test_circuit(num_constraints, num_variables);
        SonicPCPoswTest::test_circuit(num_constraints, num_variables);
    }
}

mod marlin_recursion {
    use super::*;
    use crate::{
        bls12_377::Scalar,
        marlin::{ahp::AHPForR1CS, CircuitVerifyingKey, MarlinSNARK},
    };

    use core::ops::MulAssign;
    use std::str::FromStr;

    type MarlinInst = MarlinSNARK;
    type FS = PoseidonSponge;

    fn test_circuit(num_constraints: usize, num_variables: usize) {
        let rng = &mut rand::thread_rng();
        let ahp = AHPForR1CS { mode: true };

        let max_degree = ahp.max_degree(100, 25, 300).unwrap();
        let universal_srs = MarlinInst::universal_setup(max_degree, rng).unwrap();
        let fs_parameters = PoseidonParameters::default();

        let rng = &mut rand::thread_rng();
        let a = Scalar::rand();
        let b = Scalar::rand();
        let mut c = a;
        c.mul_assign(&b);
        let mut d = c;
        d.mul_assign(&b);

        let circuit = Circuit {
            a: Some(a),
            b: Some(b),
            num_constraints,
            num_variables,
        };

        let (index_pk, index_vk) =
            MarlinInst::circuit_setup(&universal_srs, &circuit, true).unwrap();
        println!("Called circuit setup");

        let snark = MarlinInst { mode: true };
        let proof = snark
            .prove(&fs_parameters, &index_pk, &circuit, rng)
            .unwrap();
        println!("Called prover");

        assert!(snark
            .verify::<[Scalar], _>(&fs_parameters, &index_vk, [c, d], &proof)
            .unwrap());
        println!("Called verifier");
        println!("\nShould not verify (i.e. verifier messages should print below):");
        assert!(!snark
            .verify::<[Scalar], _>(&fs_parameters, &index_vk, [a, a], &proof)
            .unwrap());
    }

    #[test]
    fn prove_and_verify_with_tall_matrix_big() {
        let num_constraints = 100;
        let num_variables = 25;

        test_circuit(num_constraints, num_variables);
    }

    #[test]
    fn prove_and_verify_with_tall_matrix_small() {
        let num_constraints = 26;
        let num_variables = 25;

        test_circuit(num_constraints, num_variables);
    }

    #[test]
    fn prove_and_verify_with_squat_matrix_big() {
        let num_constraints = 25;
        let num_variables = 100;

        test_circuit(num_constraints, num_variables);
    }

    #[test]
    fn prove_and_verify_with_squat_matrix_small() {
        let num_constraints = 25;
        let num_variables = 26;

        test_circuit(num_constraints, num_variables);
    }

    #[test]
    fn prove_and_verify_with_square_matrix() {
        let num_constraints = 25;
        let num_variables = 25;

        test_circuit(num_constraints, num_variables);
    }

    // #[test]
    // /// Test on a constraint system that will trigger outlining.
    // fn prove_and_test_outlining() {
    //     let rng = &mut rand::thread_rng();
    //
    //     let universal_srs = MarlinInst::universal_setup(150, 150, 150, rng).unwrap();
    //
    //     let circ = OutlineTestCircuit {
    //         field_phantom: PhantomData,
    //     };
    //
    //     let (index_pk, index_vk) = MarlinInst::index(&universal_srs, circ.clone()).unwrap();
    //     println!("Called index");
    //
    //     let proof = MarlinInst::prove(&index_pk, circ, rng).unwrap();
    //     println!("Called prover");
    //
    //     let mut inputs = Vec::new();
    //     for i in 0u128..5u128 {
    //         inputs.push(Fr::from(i));
    //     }
    //
    //     assert!(MarlinInst::verify(&index_vk, &inputs, &proof).unwrap());
    //     println!("Called verifier");
    // }
}
