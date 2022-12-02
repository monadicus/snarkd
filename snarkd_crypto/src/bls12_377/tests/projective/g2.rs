use test_runner::{Namespace, Test, TestResult};

use crate::bls12_377::G2Projective;

use super::*;

pub struct G2ProjectiveNs;

impl Namespace for G2ProjectiveNs {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "add" => {
                let (a, b, c) = serde_json::from_value(test.input).expect("failed to get input");
                add::<G2Projective>(a, b, c)
            }
            "mul" => {
                let (a, b, s) = serde_json::from_value(test.input).expect("failed to get input");
                mul::<G2Projective>(a, b, s)
            }
            "double" => {
                let (a, b) = serde_json::from_value(test.input).expect("failed to get input");
                double::<G2Projective>(a, b)
            }
            "neg" => {
                let (a, s) = serde_json::from_value(test.input).expect("failed to get input");
                neg::<G2Projective>(a, s)
            }
            "transform" => {
                let g = serde_json::from_value(test.input).expect("failed to get input");
                transform::<G2Projective>(g)
            }
            "batch_normalization" => {
                let batch = serde_json::from_value(test.input).expect("failed to get input");
                batch_normalization::<G2Projective>(batch)
            }
            e => panic!("unknown method for G2ProjectiveNs: {e}"),
        }
    }
}
