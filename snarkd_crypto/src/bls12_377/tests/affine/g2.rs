use test_runner::{Namespace, Test, TestResult};

use super::*;
use crate::bls12_377::G2Affine;

pub struct G2AffineNs;

impl Namespace for G2AffineNs {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "neg" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                neg::<G2Affine>(a)
            }
            "mul" => {
                let (a, s) = serde_json::from_value(test.input).expect("failed to get input");
                mul::<G2Affine>(a, s)
            }
            e => panic!("unknown method for G1ProjectiveNs: {e}"),
        }
    }
}
