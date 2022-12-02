use test_runner::{Namespace, Test, TestResult};

use crate::bls12_377::Fp;

use super::*;

pub struct FpNs;

impl Namespace for FpNs {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "neg" => {
                let a = Self::get(test.input);
                neg::<Fp>(a)
            }
            "add" => {
                let (a, b, c) = Self::get(test.input);
                add::<Fp>(a, b, c)
            }
            "sub" => {
                let (a, b) = Self::get(test.input);
                sub::<Fp>(a, b)
            }
            "mul" => {
                let (a, b, c) = Self::get(test.input);
                mul::<Fp>(a, b, c)
            }
            "inversion" => {
                let a = Self::get(test.input);
                inversion::<Fp>(a)
            }
            "double" => {
                let a = Self::get(test.input);
                double::<Fp>(a)
            }
            "square" => {
                let a = Self::get(test.input);
                square::<Fp>(a)
            }
            "expansion" => {
                let (a, b, c, d) = Self::get(test.input);
                expansion::<Fp>(a, b, c, d)
            }
            "sqrt" => {
                let a = Self::get(test.input);
                sqrt::<Fp>(a)
            }
            "pow" => {
                let a = Self::get(test.input);
                pow::<Fp>(a)
            }
            "sum_of_products" => {
                let (a, b) = Self::get(test.input);
                sum_of_products::<Fp>(a, b)
            }
            "math_properties" => {
                let (a, b) = Self::get(test.input);
                math_properties::<Fp>(a, b)
            }
            e => panic!("unknown method for FpNs: {e}"),
        }
    }
}
