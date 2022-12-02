use test_runner::{Namespace, Test, TestResult};

use crate::bls12_377::Fp;

use super::*;

pub struct FpNs;

impl Namespace for FpNs {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "neg" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                neg::<Fp>(a)
            }
            "add" => {
                let (a, b, c) = serde_json::from_value(test.input).expect("failed to get input");
                add::<Fp>(a, b, c)
            }
            "sub" => {
                let (a, b) = serde_json::from_value(test.input).expect("failed to get input");
                sub::<Fp>(a, b)
            }
            "mul" => {
                let (a, b, c) = serde_json::from_value(test.input).expect("failed to get input");
                mul::<Fp>(a, b, c)
            }
            "inversion" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                inversion::<Fp>(a)
            }
            "double" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                double::<Fp>(a)
            }
            "square" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                square::<Fp>(a)
            }
            "expansion" => {
                let (a, b, c, d) = serde_json::from_value(test.input).expect("failed to get input");
                expansion::<Fp>(a, b, c, d)
            }
            "sqrt" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                sqrt::<Fp>(a)
            }
            "pow" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                pow::<Fp>(a)
            }
            "sum_of_products" => {
                let (a, b) = serde_json::from_value(test.input).expect("failed to get input");
                sum_of_products::<Fp>(a, b)
            }
            "math_properties" => {
                let (a, b) = serde_json::from_value(test.input).expect("failed to get input");
                math_properties::<Fp>(a, b)
            }
            e => panic!("unknown method for FpNs: {e}"),
        }
    }
}
