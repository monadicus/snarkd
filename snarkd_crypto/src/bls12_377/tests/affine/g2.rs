use super::*;
use crate::bls12_377::{test::tests::field::Fp2Tuple, G2Affine};

pub type G2AffineTuple = AffineTuple<Fp2Tuple>;

pub struct G2AffineNs;

impl Namespace for G2AffineNs {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "neg" => {
                let a: G2AffineTuple = Self::get(test.input);
                neg::<G2Affine>(a.into())
            }
            "mul" => {
                let (a, s): (G2AffineTuple, _) = Self::get(test.input);
                mul::<G2Affine>(a.into(), s)
            }
            e => panic!("unknown method for G1ProjectiveNs: {e}"),
        }
    }
}

#[test]
fn test_generator() {
    let generator = G2Affine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}
