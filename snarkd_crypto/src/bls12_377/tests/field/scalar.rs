use test_runner::{Namespace, Test, TestResult};

use crate::bls12_377::Scalar;

use super::*;

pub struct ScalarNs;

impl ScalarNs {
    pub fn neg(a: Scalar) -> TestResult {
        neg(a)
    }

    pub fn add(a: Scalar, b: Scalar, c: Scalar) -> TestResult {
        add(a, b, c)
    }

    pub fn sub(a: Scalar, b: Scalar) -> TestResult {
        sub(a, b)
    }

    pub fn mul(a: Scalar, b: Scalar, c: Scalar) -> TestResult {
        mul(a, b, c)
    }

    pub fn inversion(a: Scalar) -> TestResult {
        inversion(a)
    }

    pub fn double(a: Scalar) -> TestResult {
        double(a)
    }

    pub fn square(a: Scalar) -> TestResult {
        square(a)
    }

    pub fn expansion(a: Scalar, b: Scalar, c: Scalar, d: Scalar) -> TestResult {
        expansion(a, b, c, d)
    }

    pub fn sqrt(a: Scalar) -> TestResult {
        sqrt(a)
    }

    pub fn pow(a: Scalar) -> TestResult {
        pow(a)
    }

    pub fn sum_of_products(a: Vec<Scalar>, b: Vec<Scalar>) -> TestResult {
        sum_of_products(a, b)
    }

    pub fn math_properties(a: Scalar, b: Scalar) -> TestResult {
        math_properties(a, b)
    }
}

impl Namespace for ScalarNs {
    fn run_test(&self, test: Test) -> TestResult {
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
            "pow" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                Self::pow(a)
            }
            "sum_of_products" => {
                let (a, b) = serde_json::from_value(test.input).expect("failed to get input");
                Self::sum_of_products(a, b)
            }
            "math_properties" => {
                let (a, b) = serde_json::from_value(test.input).expect("failed to get input");
                Self::math_properties(a, b)
            }
            e => panic!("unknown method for ScalarNs: {e}"),
        }
    }
}
