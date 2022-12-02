use test_runner::{Namespace, Test, TestResult};

use super::*;
use crate::bls12_377::G1Affine;

pub struct G1AffineNs;

impl Namespace for G1AffineNs {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "neg" => {
                let a = Self::get(test.input);
                neg::<G1Affine>(a)
            }
            "mul" => {
                let (a, s) = Self::get(test.input);
                mul::<G1Affine>(a, s)
            }
            e => panic!("unknown method for G1ProjectiveNs: {e}"),
        }
    }
}
