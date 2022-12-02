use test_runner::{Namespace, Test, TestResult};

use crate::bls12_377::Scalar;

use super::*;

pub struct ScalarNs;

impl Namespace for ScalarNs {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "neg" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                neg::<Scalar>(a)
            }
            "add" => {
                let (a, b, c) = serde_json::from_value(test.input).expect("failed to get input");
                add::<Scalar>(a, b, c)
            }
            "sub" => {
                let (a, b) = serde_json::from_value(test.input).expect("failed to get input");
                sub::<Scalar>(a, b)
            }
            "mul" => {
                let (a, b, c) = serde_json::from_value(test.input).expect("failed to get input");
                mul::<Scalar>(a, b, c)
            }
            "inversion" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                inversion::<Scalar>(a)
            }
            "double" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                double::<Scalar>(a)
            }
            "square" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                square::<Scalar>(a)
            }
            "expansion" => {
                let (a, b, c, d) = serde_json::from_value(test.input).expect("failed to get input");
                expansion::<Scalar>(a, b, c, d)
            }
            "sqrt" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                sqrt::<Scalar>(a)
            }
            "pow" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                pow::<Scalar>(a)
            }
            "sum_of_products" => {
                let (a, b) = serde_json::from_value(test.input).expect("failed to get input");
                sum_of_products::<Scalar>(a, b)
            }
            "math_properties" => {
                let (a, b) = serde_json::from_value(test.input).expect("failed to get input");
                math_properties::<Scalar>(a, b)
            }
            e => panic!("unknown method for ScalarNs: {e}"),
        }
    }
}
