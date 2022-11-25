use serde_json::Value;
use test_runner::{Namespace, Test};

use crate::bls12_377::Scalar;

use super::{add, double, expansion, inversion, mul, neg, sqrt, square, sub};

pub struct ScalarNs;

impl ScalarNs {
    pub fn neg(a: Scalar) -> Result<Value, String> {
        neg(a)
    }

    pub fn add(a: Scalar, b: Scalar, c: Scalar) -> Result<Value, String> {
        add(a, b, c)
    }

    pub fn sub(a: Scalar, b: Scalar) -> Result<Value, String> {
        sub(a, b)
    }

    pub fn mul(a: Scalar, b: Scalar, c: Scalar) -> Result<Value, String> {
        mul(a, b, c)
    }

    pub fn inversion(a: Scalar) -> Result<Value, String> {
        inversion(a)
    }

    pub fn double(a: Scalar) -> Result<Value, String> {
        double(a)
    }

    pub fn square(a: Scalar) -> Result<Value, String> {
        square(a)
    }

    pub fn expansion(a: Scalar, b: Scalar, c: Scalar, d: Scalar) -> Result<Value, String> {
        expansion(a, b, c, d)
    }

    pub fn sqrt(a: Scalar) -> Result<Value, String> {
        sqrt(a)
    }
}

impl Namespace for ScalarNs {
    fn run_test(&self, test: Test) -> Result<Value, String> {
        match test.method.as_str() {
            "neg" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                Self::neg(a)
            }
            "add" => {
                let (a, b, c) = serde_json::from_value(test.input).expect("failed to get input");
                Self::add(a, b, c)
            }
            "sub" => {
                let (a, b) = serde_json::from_value(test.input).expect("failed to get input");
                Self::sub(a, b)
            }
            "mul" => {
                let (a, b, c) = serde_json::from_value(test.input).expect("failed to get input");
                Self::mul(a, b, c)
            }
            "inversion" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                Self::inversion(a)
            }
            "double" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                Self::double(a)
            }
            "square" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                Self::square(a)
            }
            "expansion" => {
                let (a, b, c, d) = serde_json::from_value(test.input).expect("failed to get input");
                Self::expansion(a, b, c, d)
            }
            "sqrt" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                Self::sqrt(a)
            }
            e => panic!("unknown method for ScalarNs: {e}"),
        }
    }
}
