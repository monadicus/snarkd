use test_runner::{Namespace, Test, TestResult};

use super::*;
use crate::bls12_377::G2Affine;

pub struct G2AffineNs;

impl Namespace for G2AffineNs {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "neg" => {
                let a = Self::get(test.input);
                neg::<G2Affine>(a)
            }
            "mul" => {
                let (a, s) = Self::get(test.input);
                mul::<G2Affine>(a, s)
            }
            e => panic!("unknown method for G1ProjectiveNs: {e}"),
        }
    }
}
