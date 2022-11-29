use test_runner::{Namespace, Test, TestResult};

use crate::{bls12_377::Fp2, frobenius};

use super::*;

pub struct Fp2Ns;

impl Fp2Ns {
    pub fn neg(a: Fp2) -> TestResult {
        neg(a)
    }

    pub fn add(a: Fp2, b: Fp2, c: Fp2) -> TestResult {
        add(a, b, c)
    }

    pub fn sub(a: Fp2, b: Fp2) -> TestResult {
        sub(a, b)
    }

    pub fn mul(a: Fp2, b: Fp2, c: Fp2) -> TestResult {
        mul(a, b, c)
    }

    pub fn inversion(a: Fp2) -> TestResult {
        inversion(a)
    }

    pub fn double(a: Fp2) -> TestResult {
        double(a)
    }

    pub fn square(a: Fp2) -> TestResult {
        square(a)
    }

    pub fn expansion(a: Fp2, b: Fp2, c: Fp2, d: Fp2) -> TestResult {
        expansion(a, b, c, d)
    }

    pub fn sqrt(a: Fp2) -> TestResult {
        sqrt(a)
    }

    pub fn frobenius(a: Fp2) -> TestResult {
        frobenius!(a, Fp2)
    }

    pub fn pow(a: Fp2) -> TestResult {
        pow(a)
    }

    pub fn sum_of_products(a: Vec<Fp2>, b: Vec<Fp2>) -> TestResult {
        sum_of_products(a, b)
    }

    pub fn math_properties(a: Fp2, b: Fp2) -> TestResult {
        math_properties(a, b)
    }
}

impl Namespace for Fp2Ns {
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
            "frobenius" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                Self::frobenius(a)
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
            e => panic!("unknown method for Fp2Ns: {e}"),
        }
    }
}
