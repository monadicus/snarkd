use serde_json::Value;
use test_runner::{Namespace, Test};

use crate::{bls12_377::Fp12, frobenius};

use super::{add, double, expansion, inversion, mul, neg, sqrt, square, sub};

pub struct Fp12Ns;

impl Fp12Ns {
    pub fn neg(a: Fp12) -> Result<Value, String> {
        neg(a)
    }

    pub fn add(a: Fp12, b: Fp12, c: Fp12) -> Result<Value, String> {
        add(a, b, c)
    }

    pub fn sub(a: Fp12, b: Fp12) -> Result<Value, String> {
        sub(a, b)
    }

    pub fn mul(a: Fp12, b: Fp12, c: Fp12) -> Result<Value, String> {
        mul(a, b, c)
    }

    pub fn inversion(a: Fp12) -> Result<Value, String> {
        inversion(a)
    }

    pub fn double(a: Fp12) -> Result<Value, String> {
        double(a)
    }

    pub fn square(a: Fp12) -> Result<Value, String> {
        square(a)
    }

    pub fn expansion(a: Fp12, b: Fp12, c: Fp12, d: Fp12) -> Result<Value, String> {
        expansion(a, b, c, d)
    }

    pub fn sqrt(a: Fp12) -> Result<Value, String> {
        sqrt(a)
    }

    pub fn frobenius(a: Fp12) -> Result<Value, String> {
        frobenius!(a, Fp12)
    }
}

impl Namespace for Fp12Ns {
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
            "frobenius" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                Self::frobenius(a)
            }
            "sqrt" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                Self::sqrt(a)
            }
            e => panic!("unknown method for Fp12Ns: {e}"),
        }
    }
}