use test_runner::{Namespace, Test, TestResult};

use crate::{
    bls12_377::{Fp, Fp2},
    frobenius,
};

use super::*;

pub struct Fp2Ns;

pub type Fp2Tuple = [Fp; 2];
impl From<Fp2Tuple> for Fp2 {
    fn from(value: Fp2Tuple) -> Self {
        Self {
            c0: value[0],
            c1: value[1],
        }
    }
}

impl Namespace for Fp2Ns {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "neg" => {
                let a: Fp2Tuple = serde_json::from_value(test.input).expect("failed to get input");
                neg::<Fp2>(a.into())
            }
            "add" => {
                let (a, b, c): (Fp2Tuple, Fp2Tuple, Fp2Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                add::<Fp2>(a.into(), b.into(), c.into())
            }
            "sub" => {
                let (a, b): (Fp2Tuple, Fp2Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                sub::<Fp2>(a.into(), b.into())
            }
            "mul" => {
                let (a, b, c): (Fp2Tuple, Fp2Tuple, Fp2Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                mul::<Fp2>(a.into(), b.into(), c.into())
            }
            "inversion" => {
                let a: Fp2Tuple = serde_json::from_value(test.input).expect("failed to get input");
                inversion::<Fp2>(a.into())
            }
            "double" => {
                let a: Fp2Tuple = serde_json::from_value(test.input).expect("failed to get input");
                double::<Fp2>(a.into())
            }
            "square" => {
                let a: Fp2Tuple = serde_json::from_value(test.input).expect("failed to get input");
                square::<Fp2>(a.into())
            }
            "expansion" => {
                let (a, b, c, d): (Fp2Tuple, Fp2Tuple, Fp2Tuple, Fp2Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                expansion::<Fp2>(a.into(), b.into(), c.into(), d.into())
            }
            "frobenius" => {
                let a: Fp2Tuple = serde_json::from_value(test.input).expect("failed to get input");
                frobenius!(Fp2::from(a), Fp2)
            }
            "sqrt" => {
                let a: Fp2Tuple = serde_json::from_value(test.input).expect("failed to get input");
                sqrt::<Fp2>(a.into())
            }
            "pow" => {
                let a: Fp2Tuple = serde_json::from_value(test.input).expect("failed to get input");
                pow::<Fp2>(a.into())
            }
            "sum_of_products" => {
                let (a, b): (Vec<Fp2Tuple>, Vec<Fp2Tuple>) =
                    serde_json::from_value(test.input).expect("failed to get input");
                let a = a.into_iter().map(|f| f.into()).collect();
                let b = b.into_iter().map(|f| f.into()).collect();
                sum_of_products::<Fp2>(a, b)
            }
            "math_properties" => {
                let (a, b): (Fp2Tuple, Fp2Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                math_properties::<Fp2>(a.into(), b.into())
            }
            e => panic!("unknown method for Fp2Ns: {e}"),
        }
    }
}
