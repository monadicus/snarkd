pub mod affine;
use affine::*;

pub mod field;
use field::*;

pub mod projective;
use projective::*;

#[cfg(test)]
use crate::bls12_377::{fp, Field, Fp, Fp12, Fp2, Fp6, Scalar};

use test_runner::{Namespace, Runner};

struct TestRunner;

impl Runner for TestRunner {
    fn resolve_namespace(&self, name: &str) -> Option<Box<dyn Namespace>> {
        Some(match name {
            "FpNs" => Box::new(FpNs),
            "Fp2Ns" => Box::new(Fp2Ns),
            "Fp6Ns" => Box::new(Fp6Ns),
            "Fp12Ns" => Box::new(Fp12Ns),
            "G1ProjectiveNs" => Box::new(G1ProjectiveNs),
            "G2ProjectiveNs" => Box::new(G2ProjectiveNs),
            "BilinearNs" => Box::new(BilinearNs),
            "ScalarNs" => Box::new(ScalarNs),
            "AffineG1Ns" => Box::new(G1AffineNs),
            "AffineG2Ns" => Box::new(G2AffineNs),
            _ => return None,
        })
    }
}

#[test]
fn curve_tests() {
    test_runner::run_tests(&TestRunner, "crypto");
}

#[test]
fn test_zero_one_two() {
    zero_one_two::<Fp>();
    zero_one_two::<Fp2>();
    zero_one_two::<Fp6>();
    zero_one_two::<Fp12>();
    zero_one_two::<Scalar>();
}

#[test]
fn test_sqrt_1_to_100() {
    sqrt_1_to_100::<Fp>();
    sqrt_1_to_100::<Fp2>();
    sqrt_1_to_100::<Scalar>();
}

#[test]
fn test_fp_num_bits() {
    assert_eq!(fp::MODULUS_BITS, 377);
    assert_eq!(fp::CAPACITY, 376);
}

#[test]
fn test_fp_root_of_unity() {
    assert_eq!(fp::TWO_ADICITY, 46);
    assert_eq!(
        Fp::MULTIPLICATIVE_GENERATOR.pow(&[
            0x7510c00000021423,
            0x88bee82520005c2d,
            0x67cc03d44e3c7bcd,
            0x1701b28524ec688b,
            0xe9185f1443ab18ec,
            0x6b8
        ]),
        fp::TWO_ADIC_ROOT_OF_UNITY_AS_FIELD
    );
    assert_eq!(
        fp::TWO_ADIC_ROOT_OF_UNITY_AS_FIELD.pow(&[1 << fp::TWO_ADICITY]),
        Fp::ONE
    );
    assert!(Fp::MULTIPLICATIVE_GENERATOR.sqrt().is_none());
}
