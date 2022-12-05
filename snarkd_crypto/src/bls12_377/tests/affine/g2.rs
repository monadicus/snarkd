use serde::{Deserialize, Serialize};
use test_runner::{Namespace, Test, TestResult};

use super::*;
use crate::bls12_377::{test::tests::field::Fp2Tuple, G2Affine};

#[derive(Serialize, Deserialize)]
struct G2AffineTuple {
    x: Fp2Tuple,
    y: Fp2Tuple,
    infinity: bool,
}

impl From<G2AffineTuple> for G2Affine {
    fn from(value: G2AffineTuple) -> Self {
        Self {
            x: value.x.into(),
            y: value.y.into(),
            infinity: value.infinity,
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
