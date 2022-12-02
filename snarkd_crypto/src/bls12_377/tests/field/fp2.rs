use test_runner::{Namespace, Test, TestResult};

use crate::{
    bls12_377::{Fp, Fp2},
    frobenius,
};

use super::*;

pub type Fp2Tuple = [Fp; 2];

impl From<Fp2Tuple> for Fp2 {
    fn from(value: Fp2Tuple) -> Self {
        Self {
            c0: value[0],
            c1: value[1],
        }
    }
}

impl From<Fp2> for Fp2Tuple {
    fn from(v: Fp2) -> Self {
        [v.c0, v.c1]
    }
}

pub struct Fp2Ns;

impl Namespace for Fp2Ns {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "neg" => {
                let a: Fp2Tuple = Self::get(test.input);
                neg::<Fp2>(a.into())
            }
            "add" => {
                let (a, b, c): (Fp2Tuple, Fp2Tuple, Fp2Tuple) = Self::get(test.input);
                add::<Fp2>(a.into(), b.into(), c.into())
            }
            "sub" => {
                let (a, b): (Fp2Tuple, Fp2Tuple) = Self::get(test.input);
                sub::<Fp2>(a.into(), b.into())
            }
            "mul" => {
                let (a, b, c): (Fp2Tuple, Fp2Tuple, Fp2Tuple) = Self::get(test.input);
                mul::<Fp2>(a.into(), b.into(), c.into())
            }
            "inversion" => {
                let a: Fp2Tuple = Self::get(test.input);
                inversion::<Fp2>(a.into())
            }
            "double" => {
                let a: Fp2Tuple = Self::get(test.input);
                double::<Fp2>(a.into())
            }
            "square" => {
                let a: Fp2Tuple = Self::get(test.input);
                square::<Fp2>(a.into())
            }
            "expansion" => {
                let (a, b, c, d): (Fp2Tuple, Fp2Tuple, Fp2Tuple, Fp2Tuple) = Self::get(test.input);
                expansion::<Fp2>(a.into(), b.into(), c.into(), d.into())
            }
            "frobenius" => {
                let a: Fp2Tuple = Self::get(test.input);
                frobenius!(Fp2::from(a), Fp2)
            }
            "sqrt" => {
                let a: Fp2Tuple = Self::get(test.input);
                sqrt::<Fp2>(a.into())
            }
            "pow" => {
                let a: Fp2Tuple = Self::get(test.input);
                pow::<Fp2>(a.into())
            }
            "sum_of_products" => {
                let (a, b): (Vec<Fp2Tuple>, Vec<Fp2Tuple>) = Self::get(test.input);
                let a = a.into_iter().map(|f| f.into()).collect();
                let b = b.into_iter().map(|f| f.into()).collect();
                sum_of_products::<Fp2>(a, b)
            }
            "math_properties" => {
                let (a, b): (Fp2Tuple, Fp2Tuple) = Self::get(test.input);
                math_properties::<Fp2>(a.into(), b.into())
            }
            e => panic!("unknown method for Fp2Ns: {e}"),
        }
    }
}
