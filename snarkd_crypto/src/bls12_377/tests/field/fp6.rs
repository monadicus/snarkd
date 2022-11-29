use std::ops::MulAssign;

use test_runner::{Namespace, Test, TestResult};

use crate::{
    bls12_377::{Fp2, Fp6},
    frobenius,
};

use super::*;

pub struct Fp6Ns;

impl Fp6Ns {
    pub fn neg(a: Fp6) -> TestResult {
        neg(a)
    }

    pub fn add(a: Fp6, b: Fp6, c: Fp6) -> TestResult {
        add(a, b, c)
    }

    pub fn sub(a: Fp6, b: Fp6) -> TestResult {
        sub(a, b)
    }

    pub fn mul(a: Fp6, b: Fp6, c: Fp6) -> TestResult {
        mul(a, b, c)
    }

    pub fn inversion(a: Fp6) -> TestResult {
        inversion(a)
    }

    pub fn double(a: Fp6) -> TestResult {
        double(a)
    }

    pub fn square(a: Fp6) -> TestResult {
        square(a)
    }

    pub fn expansion(a: Fp6, b: Fp6, c: Fp6, d: Fp6) -> TestResult {
        expansion(a, b, c, d)
    }

    pub fn sqrt(a: Fp6) -> TestResult {
        sqrt(a)
    }

    pub fn frobenius(a: Fp6) -> TestResult {
        frobenius!(a, Fp6)
    }

    pub fn pow(a: Fp6) -> TestResult {
        pow(a)
    }

    pub fn sum_of_products(a: Vec<Fp6>, b: Vec<Fp6>) -> TestResult {
        sum_of_products(a, b)
    }

    pub fn math_properties(a: Fp6, b: Fp6) -> TestResult {
        math_properties(a, b)
    }

    pub fn mul_by_1(c1: Fp2, mut a: Fp6) -> TestResult {
        let mut outputs = Vec::new();
        let mut b = a;

        a.mul_by_1(&c1);
        outputs.push(a);
        b.mul_assign(&Fp6::new(Fp2::ZERO, c1, Fp2::ZERO));
        outputs.push(b);
        assert_eq!(a, b);

        Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
    }

    pub fn mul_by_01(c0: Fp2, c1: Fp2, mut a: Fp6) -> TestResult {
        let mut outputs = Vec::new();
        let mut b = a;

        a.mul_by_01(&c0, &c1);
        outputs.push(a);
        b.mul_assign(&Fp6::new(c0, c1, Fp2::ZERO));
        outputs.push(b);
        assert_eq!(a, b);

        Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
    }
}

impl Namespace for Fp6Ns {
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
            "mul_by_1" => {
                let (c1, a) = serde_json::from_value(test.input).expect("failed to get input");
                Self::mul_by_1(c1, a)
            }
            "mul_by_01" => {
                let (c0, c1, a) = serde_json::from_value(test.input).expect("failed to get input");
                Self::mul_by_01(c0, c1, a)
            }
            e => panic!("unknown method for Fp6Ns: {e}"),
        }
    }
}
