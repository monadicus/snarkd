use std::ops::MulAssign;

use test_runner::{Namespace, Test, TestResult};

use crate::{
    bls12_377::{Fp12, Fp2, Fp6},
    frobenius,
};

use super::*;

type Fp12Tuple = [Fp6Tuple; 2];

impl From<Fp12Tuple> for Fp12 {
    fn from(value: Fp12Tuple) -> Self {
        Self {
            c0: value[0].into(),
            c1: value[1].into(),
        }
    }
}

pub struct Fp12Ns;

impl Fp12Ns {
    pub fn mul_by_014(c0: Fp2, c1: Fp2, c5: Fp2, mut a: Fp12) -> TestResult {
        let mut outputs = Vec::new();
        let mut b = a;

        a.mul_by_014(&c0, &c1, &c5);
        outputs.push(a);
        b.mul_assign(&Fp12::new(
            Fp6::new(c0, c1, Fp2::ZERO),
            Fp6::new(Fp2::ZERO, c5, Fp2::ZERO),
        ));
        outputs.push(b);
        assert_eq!(a, b);

        Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
    }

    pub fn mul_by_034(c0: Fp2, c3: Fp2, c4: Fp2, mut a: Fp12) -> TestResult {
        let mut outputs = Vec::new();
        let mut b = a;

        a.mul_by_034(&c0, &c3, &c4);
        outputs.push(a);
        b.mul_assign(&Fp12::new(
            Fp6::new(c0, Fp2::ZERO, Fp2::ZERO),
            Fp6::new(c3, c4, Fp2::ZERO),
        ));
        outputs.push(b);
        assert_eq!(a, b);

        Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
    }
}

impl Namespace for Fp12Ns {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "neg" => {
                let a: Fp12Tuple = Self::get(test.input);
                neg::<Fp12>(a.into())
            }
            "add" => {
                let (a, b, c): (Fp12Tuple, Fp12Tuple, Fp12Tuple) = Self::get(test.input);
                add::<Fp12>(a.into(), b.into(), c.into())
            }
            "sub" => {
                let (a, b): (Fp12Tuple, Fp12Tuple) = Self::get(test.input);
                sub::<Fp12>(a.into(), b.into())
            }
            "mul" => {
                let (a, b, c): (Fp12Tuple, Fp12Tuple, Fp12Tuple) = Self::get(test.input);
                mul::<Fp12>(a.into(), b.into(), c.into())
            }
            "inversion" => {
                let a: Fp12Tuple = Self::get(test.input);
                inversion::<Fp12>(a.into())
            }
            "double" => {
                let a: Fp12Tuple = Self::get(test.input);
                double::<Fp12>(a.into())
            }
            "square" => {
                let a: Fp12Tuple = Self::get(test.input);
                square::<Fp12>(a.into())
            }
            "expansion" => {
                let (a, b, c, d): (Fp12Tuple, Fp12Tuple, Fp12Tuple, Fp12Tuple) =
                    Self::get(test.input);
                expansion::<Fp12>(a.into(), b.into(), c.into(), d.into())
            }
            "frobenius" => {
                let a: Fp12Tuple = Self::get(test.input);
                frobenius!(Fp12::from(a), Fp12)
            }
            "sqrt" => {
                let a: Fp12Tuple = Self::get(test.input);
                sqrt::<Fp12>(a.into())
            }
            "pow" => {
                let a: Fp12Tuple = Self::get(test.input);
                pow::<Fp12>(a.into())
            }
            "sum_of_products" => {
                let (a, b): (Vec<Fp12Tuple>, Vec<Fp12Tuple>) = Self::get(test.input);
                let a = a.into_iter().map(|f| f.into()).collect();
                let b = b.into_iter().map(|f| f.into()).collect();
                sum_of_products::<Fp12>(a, b)
            }
            "math_properties" => {
                let (a, b): (Fp12Tuple, Fp12Tuple) = Self::get(test.input);
                math_properties::<Fp12>(a.into(), b.into())
            }
            "mul_by_014" => {
                let (c0, c1, c5, a): (Fp2Tuple, Fp2Tuple, Fp2Tuple, Fp12Tuple) =
                    Self::get(test.input);
                Self::mul_by_014(c0.into(), c1.into(), c5.into(), a.into())
            }
            "mul_by_034" => {
                let (c0, c3, c4, a): (Fp2Tuple, Fp2Tuple, Fp2Tuple, Fp12Tuple) =
                    Self::get(test.input);
                Self::mul_by_034(c0.into(), c3.into(), c4.into(), a.into())
            }
            e => panic!("unknown method for Fp12Ns: {e}"),
        }
    }
}
