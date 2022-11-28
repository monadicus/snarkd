pub mod affine;
pub mod field;
pub mod projective;

use std::{cmp::Ordering, ops::AddAssign};

use ruint::uint;
use test_runner::{run_tests, Namespace, Runner};

use crate::bls12_377::{
    fp, Affine, Field, Fp, Fp12, Fp2, Fp6, G1Affine, G2Affine, LegendreSymbol, Scalar,
};

use self::{
    field::{sqrt_deterministic, Fp12Ns, Fp2Ns, Fp6Ns, FpNs, ScalarNs},
    projective::{G1ProjectiveNs, G2ProjectiveNs},
};

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
            "ScalarNs" => Box::new(ScalarNs),
            _ => return None,
        })
    }
}

#[test]
fn curve_tests() {
    run_tests(&TestRunner, "crypto");
}

#[test]
fn square_deterministic() {
    sqrt_deterministic::<Fp>();
    sqrt_deterministic::<Fp2>();
    sqrt_deterministic::<Fp6>();
    sqrt_deterministic::<Fp12>();
    sqrt_deterministic::<Scalar>();
}

#[test]
fn test_fp_is_half() {
    assert_eq!(Fp::half(), Fp::ONE.double().inverse().unwrap());
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

#[test]
fn test_fp_legendre() {
    assert_eq!(LegendreSymbol::QuadraticResidue, Fp::ONE.legendre());
    assert_eq!(LegendreSymbol::Zero, Fp::ZERO.legendre());
    assert_eq!(
        LegendreSymbol::QuadraticResidue,
        Fp(uint!(4_U384)).legendre()
    );
    assert_eq!(
        LegendreSymbol::QuadraticNonResidue,
        Fp(uint!(5_U384)).legendre()
    );
}

#[test]
fn test_fp2_ordering() {
    let mut a = Fp2::new(Fp::ZERO, Fp::ZERO);
    let mut b = a;

    assert!(a.cmp(&b) == Ordering::Equal);
    b.c0.add_assign(Fp::ONE);
    assert!(a.cmp(&b) == Ordering::Less);
    a.c0.add_assign(Fp::ONE);
    assert!(a.cmp(&b) == Ordering::Equal);
    b.c1.add_assign(Fp::ONE);
    assert!(a.cmp(&b) == Ordering::Less);
    a.c0.add_assign(Fp::ONE);
    assert!(a.cmp(&b) == Ordering::Less);
    a.c1.add_assign(Fp::ONE);
    assert!(a.cmp(&b) == Ordering::Greater);
    b.c0.add_assign(Fp::ONE);
    assert!(a.cmp(&b) == Ordering::Equal);
}

#[test]
fn test_fp2_basics() {
    assert_eq!(Fp2::new(Fp::ZERO, Fp::ZERO,), Fp2::ZERO);
    assert_eq!(Fp2::new(Fp::ONE, Fp::ZERO,), Fp2::ONE);
    assert!(Fp2::ZERO.is_zero());
    assert!(!Fp2::ONE.is_zero());
    assert!(!Fp2::new(Fp::ZERO, Fp::ONE,).is_zero());
}

#[test]
fn test_fp2_legendre() {
    assert_eq!(LegendreSymbol::Zero, Fp2::ZERO.legendre());
    // i^2 = -1
    let mut m1 = -Fp2::ONE;
    assert_eq!(LegendreSymbol::QuadraticResidue, m1.legendre());
    m1 = Fp6::mul_fp2_by_nonresidue(&m1);
    assert_eq!(LegendreSymbol::QuadraticNonResidue, m1.legendre());
}

#[test]
fn test_g1_generator() {
    let generator = G1Affine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn test_g2_generator() {
    let generator = G2Affine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}
