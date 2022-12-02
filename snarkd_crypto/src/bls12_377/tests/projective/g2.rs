use test_runner::{Namespace, Test, TestResult};

use crate::bls12_377::G2Projective;

use super::*;

pub struct G2ProjectiveNs;

impl Namespace for G2ProjectiveNs {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "add" => {
                let (a, b, c) = Self::get(test.input);
                add::<G2Projective>(a, b, c)
            }
            "mul" => {
                let (a, b, s) = Self::get(test.input);
                mul::<G2Projective>(a, b, s)
            }
            "double" => {
                let (a, b) = Self::get(test.input);
                double::<G2Projective>(a, b)
            }
            "neg" => {
                let (a, s) = Self::get(test.input);
                neg::<G2Projective>(a, s)
            }
            "transform" => {
                let g = Self::get(test.input);
                transform::<G2Projective>(g)
            }
            "batch_normalization" => {
                let batch = Self::get(test.input);
                batch_normalization::<G2Projective>(batch)
            }
            e => panic!("unknown method for G2ProjectiveNs: {e}"),
        }
    }
}
