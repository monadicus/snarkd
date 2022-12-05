use crate::bls12_377::Scalar;

use super::*;

pub struct ScalarNs;

impl Namespace for ScalarNs {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "neg" => {
                let a = Self::get(test.input);
                neg::<Scalar>(a)
            }
            "add" => {
                let (a, b, c) = Self::get(test.input);
                add::<Scalar>(a, b, c)
            }
            "sub" => {
                let (a, b) = Self::get(test.input);
                sub::<Scalar>(a, b)
            }
            "mul" => {
                let (a, b, c) = Self::get(test.input);
                mul::<Scalar>(a, b, c)
            }
            "inversion" => {
                let a = Self::get(test.input);
                inversion::<Scalar>(a)
            }
            "double" => {
                let a = Self::get(test.input);
                double::<Scalar>(a)
            }
            "square" => {
                let a = Self::get(test.input);
                square::<Scalar>(a)
            }
            "expansion" => {
                let (a, b, c, d) = Self::get(test.input);
                expansion::<Scalar>(a, b, c, d)
            }
            "sqrt" => {
                let a = Self::get(test.input);
                sqrt::<Scalar>(a)
            }
            "pow" => {
                let a = Self::get(test.input);
                pow::<Scalar>(a)
            }
            "sum_of_products" => {
                let (a, b) = Self::get(test.input);
                sum_of_products::<Scalar>(a, b)
            }
            "math_properties" => {
                let (a, b) = Self::get(test.input);
                math_properties::<Scalar>(a, b)
            }
            e => panic!("unknown method for ScalarNs: {e}"),
        }
    }
}
