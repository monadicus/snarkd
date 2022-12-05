use serde::{Deserialize, Serialize};

use super::*;
use crate::bls12_377::{test::tests::field::Fp2Tuple, G2Affine};

#[derive(Serialize, Deserialize)]
pub struct G2AffineTuple(Fp2Tuple, Fp2Tuple, bool);

impl From<G2AffineTuple> for G2Affine {
    fn from(value: G2AffineTuple) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
            infinity: value.2,
        }
    }
}

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
