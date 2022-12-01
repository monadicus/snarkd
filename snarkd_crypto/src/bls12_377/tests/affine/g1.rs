use test_runner::{Namespace, Test, TestResult};

use super::*;
use crate::bls12_377::G1Affine;

pub struct G1AffineNs;

impl G1AffineNs {
    fn neg(a: G1Affine) -> TestResult {
        neg(a)
    }

    fn mul(a: G1Affine, s: Scalar) -> TestResult {
        mul(a, s)
    }
}

impl Namespace for G1AffineNs {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "neg" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                Self::neg(a)
            }
            "mul" => {
                let (a, s) = serde_json::from_value(test.input).expect("failed to get input");
                Self::mul(a, s)
            }
            e => panic!("unknown method for G1ProjectiveNs: {e}"),
        }
    }
}