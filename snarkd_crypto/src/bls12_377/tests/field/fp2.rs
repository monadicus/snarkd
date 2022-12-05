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

#[test]
fn test_ordering() {
    let mut a = Fp2::new(Fp::ZERO, Fp::ZERO);
    let mut b = a;

    use std::cmp::Ordering;
    assert!(a.cmp(&b) == Ordering::Equal);
    b.c0 += Fp::ONE;
    assert!(a.cmp(&b) == Ordering::Less);
    a.c0 += Fp::ONE;
    assert!(a.cmp(&b) == Ordering::Equal);
    b.c1 += Fp::ONE;
    assert!(a.cmp(&b) == Ordering::Less);
    a.c0 += Fp::ONE;
    assert!(a.cmp(&b) == Ordering::Less);
    a.c1 += Fp::ONE;
    assert!(a.cmp(&b) == Ordering::Greater);
    b.c0 += Fp::ONE;
    assert!(a.cmp(&b) == Ordering::Equal);
}

#[test]
fn test_legendre() {
    use crate::bls12_377::{Fp6, LegendreSymbol};
    assert_eq!(LegendreSymbol::Zero, Fp2::ZERO.legendre());
    // i^2 = -1
    let mut m1 = -Fp2::ONE;
    assert_eq!(LegendreSymbol::QuadraticResidue, m1.legendre());
    m1 = Fp6::mul_fp2_by_nonresidue(&m1);
    assert_eq!(LegendreSymbol::QuadraticNonResidue, m1.legendre());
}

// TODO: remove rand
#[test]
fn test_mul_nonresidue() {
    let nqr = Fp2::new(Fp::ZERO, Fp::ONE);

    use crate::bls12_377::fp2;
    let quadratic_non_residue = Fp2::new(fp2::QUADRATIC_NONRESIDUE.0, fp2::QUADRATIC_NONRESIDUE.1);
    use rayon::prelude::{IntoParallelIterator, ParallelIterator};
    (0..100).into_par_iter().for_each(|_| {
        let mut a = Fp2::rand();
        let mut b = a;
        a = quadratic_non_residue * a;
        b *= &nqr;

        assert_eq!(a, b);
    });
}
