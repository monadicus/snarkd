use test_runner::{Namespace, Test, TestResult};

use crate::bls12_377::Fp;

use super::*;

pub struct FpNs;

impl FpNs {
    pub fn neg(a: Fp) -> TestResult {
        neg(a)
    }

    pub fn add(a: Fp, b: Fp, c: Fp) -> TestResult {
        add(a, b, c)
    }

    pub fn sub(a: Fp, b: Fp) -> TestResult {
        sub(a, b)
    }

    pub fn mul(a: Fp, b: Fp, c: Fp) -> TestResult {
        mul(a, b, c)
    }

    pub fn inversion(a: Fp) -> TestResult {
        inversion(a)
    }

    pub fn double(a: Fp) -> TestResult {
        double(a)
    }

    pub fn square(a: Fp) -> TestResult {
        square(a)
    }

    pub fn expansion(a: Fp, b: Fp, c: Fp, d: Fp) -> TestResult {
        expansion(a, b, c, d)
    }

    pub fn sqrt(a: Fp) -> TestResult {
        sqrt(a)
    }

    pub fn pow(a: Fp) -> TestResult {
        pow(a)
    }

    pub fn sum_of_products(a: Vec<Fp>, b: Vec<Fp>) -> TestResult {
        sum_of_products(a, b)
    }

    pub fn math_properties(a: Fp, b: Fp) -> TestResult {
        math_properties(a, b)
    }
}

impl Namespace for FpNs {
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
            e => panic!("unknown method for FpNs: {e}"),
        }
    }
}
