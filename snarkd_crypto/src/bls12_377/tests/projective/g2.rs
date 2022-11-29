use test_runner::{Namespace, Test, TestResult};

use crate::bls12_377::{G2Projective, Scalar};

use super::*;

pub struct G2ProjectiveNs;

impl G2ProjectiveNs {
    fn add(a: G2Projective, b: G2Projective, c: G2Projective) -> TestResult {
        add(a, b, c)
    }

    fn mul(a: G2Projective, b: G2Projective, s: Scalar) -> TestResult {
        mul(a, b, s)
    }

    fn double(a: G2Projective, b: G2Projective) -> TestResult {
        double(a, b)
    }

    fn neg(a: G2Projective, s: Scalar) -> TestResult {
        neg(a, s)
    }

    fn transform(g: G2Projective) -> TestResult {
        transform(g)
    }

    fn batch_normalization(batch: Vec<G2Projective>) -> TestResult {
        batch_normalization(batch)
    }
}

impl Namespace for G2ProjectiveNs {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "add" => {
                let (a, b, c) = serde_json::from_value(test.input).expect("failed to get input");
                Self::add(a, b, c)
            }
            "mul" => {
                let (a, b, s) = serde_json::from_value(test.input).expect("failed to get input");
                Self::mul(a, b, s)
            }
            "double" => {
                let (a, b) = serde_json::from_value(test.input).expect("failed to get input");
                Self::double(a, b)
            }
            "neg" => {
                let (a, s) = serde_json::from_value(test.input).expect("failed to get input");
                Self::neg(a, s)
            }
            "transform" => {
                let g = serde_json::from_value(test.input).expect("failed to get input");
                Self::transform(g)
            }
            "batch_normalization" => {
                let batch = serde_json::from_value(test.input).expect("failed to get input");
                Self::batch_normalization(batch)
            }
            e => panic!("unknown method for G2ProjectiveNs: {e}"),
        }
    }
}
