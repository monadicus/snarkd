use serde_json::Value;
use test_runner::{Namespace, Test};

use crate::bls12_377::{G1Projective, Scalar};

use super::{add, batch_normalization, double, mul, neg, transform};

pub struct G1ProjectiveNs;

impl G1ProjectiveNs {
    fn add(a: G1Projective, b: G1Projective, c: G1Projective) -> Result<Value, String> {
        add(a, b, c)
    }

    fn mul(a: G1Projective, b: G1Projective, s: Scalar) -> Result<Value, String> {
        mul(a, b, s)
    }

    fn double(a: G1Projective, b: G1Projective) -> Result<Value, String> {
        double(a, b)
    }

    fn neg(a: G1Projective, s: Scalar) -> Result<Value, String> {
        neg(a, s)
    }

    fn transform(g: G1Projective) -> Result<Value, String> {
        transform(g)
    }

    fn batch_normalization(batch: Vec<G1Projective>) -> Result<Value, String> {
        batch_normalization(batch)
    }
}

impl Namespace for G1ProjectiveNs {
    fn run_test(&self, test: Test) -> Result<Value, String> {
        match test.method.as_str() {
            "add" => {
                let (a, b, c): (G1Projective, G1Projective, G1Projective) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::add(a, b, c)
            }
            "mul" => {
                let (a, b, s): (G1Projective, G1Projective, Scalar) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::mul(a, b, s)
            }
            "double" => {
                let (a, b): (G1Projective, G1Projective) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::double(a, b)
            }
            "neg" => {
                let (a, s): (G1Projective, Scalar) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::neg(a, s)
            }
            "transform" => {
                let g: G1Projective =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::transform(g)
            }
            "batch_normalization" => {
                let batch: Vec<G1Projective> =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::batch_normalization(batch)
            }
            e => panic!("unknown method for G1ProjectiveNs: {e}"),
        }
    }
}