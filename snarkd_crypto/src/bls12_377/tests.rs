use crate::bls12_377::{
    field::Field, fp, fp2, pairing, Affine, Fp, Fp12, Fp2, Fp6, G1Affine, G1Projective, G2Affine,
    G2Projective, LegendreSymbol, Projective, Scalar,
};
use bitvec::prelude::*;
use ruint::uint;
use std::{
    cmp::Ordering,
    ops::{AddAssign, Mul, MulAssign, SubAssign},
};

pub(crate) const ITERATIONS: usize = 10;

fn random_addition_test<G: Projective>() {
    for _ in 0..ITERATIONS {
        let a = G::rand();
        let b = G::rand();
        let c = G::rand();
        let a_affine = a.to_affine();
        let b_affine = b.to_affine();
        let c_affine = c.to_affine();

        // a + a should equal the doubling
        {
            let mut aplusa = a;
            aplusa.add_assign(a);

            let mut aplusamixed = a;
            aplusamixed.add_assign_mixed(&a.to_affine());

            let mut adouble = a;
            adouble.double_in_place();

            assert_eq!(aplusa, adouble);
            assert_eq!(aplusa, aplusamixed);
        }

        let mut tmp = vec![G::ZERO; 6];

        // (a + b) + c
        tmp[0] = (a + b) + c;

        // a + (b + c)
        tmp[1] = a + (b + c);

        // (a + c) + b
        tmp[2] = (a + c) + b;

        // Mixed addition

        // (a + b) + c
        tmp[3] = a_affine.to_projective();
        tmp[3].add_assign_mixed(&b_affine);
        tmp[3].add_assign_mixed(&c_affine);

        // a + (b + c)
        tmp[4] = b_affine.to_projective();
        tmp[4].add_assign_mixed(&c_affine);
        tmp[4].add_assign_mixed(&a_affine);

        // (a + c) + b
        tmp[5] = a_affine.to_projective();
        tmp[5].add_assign_mixed(&c_affine);
        tmp[5].add_assign_mixed(&b_affine);

        // Comparisons
        for i in 0..6 {
            for j in 0..6 {
                if tmp[i] != tmp[j] {
                    println!("{} \n{}", tmp[i], tmp[j]);
                }
                assert_eq!(tmp[i], tmp[j], "Associativity failed {} {}", i, j);
                assert_eq!(
                    tmp[i].to_affine(),
                    tmp[j].to_affine(),
                    "Associativity failed"
                );
            }

            assert!(tmp[i] != a);
            assert!(tmp[i] != b);
            assert!(tmp[i] != c);

            assert!(a != tmp[i]);
            assert!(b != tmp[i]);
            assert!(c != tmp[i]);
        }
    }
}

fn random_multiplication_test<G: Projective>() {
    for _ in 0..ITERATIONS {
        let mut a = G::rand();
        let mut b = G::rand();
        let a_affine = a.to_affine();
        let b_affine = b.to_affine();

        let s = Scalar::rand();

        // s ( a + b )
        let mut tmp1 = a;
        tmp1.add_assign(b);
        tmp1.mul_assign(s);

        // sa + sb
        a.mul_assign(s);
        b.mul_assign(s);

        let mut tmp2 = a;
        tmp2.add_assign(b);

        // Affine multiplication
        let mut tmp3 = a_affine.mul(s);
        tmp3.add_assign(b_affine.mul(s));

        assert_eq!(tmp1, tmp2);
        assert_eq!(tmp1, tmp3);
    }
}

fn random_doubling_test<G: Projective>() {
    for _ in 0..ITERATIONS {
        let mut a = G::rand();
        let mut b = G::rand();

        // 2(a + b)
        let mut tmp1 = a;
        tmp1.add_assign(b);
        tmp1.double_in_place();

        // 2a + 2b
        a.double_in_place();
        b.double_in_place();

        let mut tmp2 = a;
        tmp2.add_assign(b);

        let mut tmp3 = a;
        tmp3.add_assign_mixed(&b.to_affine());

        assert_eq!(tmp1, tmp2);
        assert_eq!(tmp1, tmp3);
    }
}

fn random_negation_test<G: Projective>() {
    for _ in 0..ITERATIONS {
        let r = G::rand();

        let s = Scalar::rand();
        let sneg = -s;
        assert!((s + sneg).is_zero());

        let mut t1 = r;
        t1.mul_assign(s);

        let mut t2 = r;
        t2.mul_assign(sneg);

        let mut t3 = t1;
        t3.add_assign(t2);
        println!("t3 = {}", t3);
        assert!(t3.is_zero());

        let mut t4 = t1;
        t4.add_assign_mixed(&t2.to_affine());
        assert!(t4.is_zero());

        t1 = -t1;
        assert_eq!(t1, t2);
    }
}

fn random_transformation_test<G: Projective>() {
    let mut rng = rand::thread_rng();

    for _ in 0..ITERATIONS {
        let g = G::rand();
        let g_affine = g.to_affine();
        let g_projective = g_affine.to_projective();
        assert_eq!(g, g_projective);
    }

    // Batch normalization
    for _ in 0..10 {
        let mut v = (0..ITERATIONS).map(|_| G::rand()).collect::<Vec<_>>();

        for i in &v {
            assert!(!i.is_normalized());
        }

        use rand::distributions::{Distribution, Uniform};
        let between = Uniform::from(0..ITERATIONS);
        // Sprinkle in some normalized points
        for _ in 0..5 {
            v[between.sample(&mut rng)] = G::ZERO;
        }
        for _ in 0..5 {
            let s = between.sample(&mut rng);
            v[s] = v[s].to_affine().to_projective();
        }

        let expected_v = v
            .iter()
            .map(|v| v.to_affine().to_projective())
            .collect::<Vec<_>>();
        G::batch_normalization(&mut v);

        for i in &v {
            assert!(i.is_normalized());
        }

        assert_eq!(v, expected_v);
    }
}

fn random_negation_tests<F: Field>() {
    for _ in 0..ITERATIONS {
        let a = F::rand();
        let mut b = -a;
        b += &a;

        assert!(b.is_zero());
    }
}

fn random_addition_tests<F: Field>() {
    for _ in 0..ITERATIONS {
        let a = F::rand();
        let b = F::rand();
        let c = F::rand();

        let t0 = (a + b) + c; // (a + b) + c

        let t1 = (a + c) + b; // (a + c) + b

        let t2 = (b + c) + a; // (b + c) + a

        assert_eq!(t0, t1);
        assert_eq!(t1, t2);
    }
}

fn random_subtraction_tests<F: Field>() {
    for _ in 0..ITERATIONS {
        let a = F::rand();
        let b = F::rand();

        let t0 = a - b; // (a - b)

        let mut t1 = b; // (b - a)
        t1 -= &a;

        let mut t2 = t0; // (a - b) + (b - a) = 0
        t2 += &t1;

        assert!(t2.is_zero());
    }
}

fn random_multiplication_tests<F: Field>() {
    for _ in 0..ITERATIONS {
        let a = F::rand();
        let b = F::rand();
        let c = F::rand();

        let mut t0 = a; // (a * b) * c
        t0 *= &b;
        t0 *= &c;

        let mut t1 = a; // (a * c) * b
        t1 *= &c;
        t1 *= &b;

        let mut t2 = b; // (b * c) * a
        t2 *= &c;
        t2 *= &a;

        assert_eq!(t0, t1);
        assert_eq!(t1, t2);
    }
}

fn random_inversion_tests<F: Field>() {
    assert!(F::ZERO.inverse().is_none());

    for _ in 0..ITERATIONS {
        let mut a = F::rand();
        let b = a.inverse().unwrap(); // probablistically nonzero
        a *= &b;

        assert_eq!(a, F::ONE);
    }
}

fn random_doubling_tests<F: Field>() {
    for _ in 0..ITERATIONS {
        let mut a = F::rand();
        let mut b = a;
        a += &b;
        b.double_in_place();

        assert_eq!(a, b);
    }
}

fn random_squaring_tests<F: Field>() {
    for _ in 0..ITERATIONS {
        let mut a = F::rand();
        let mut b = a;
        a *= &b;
        b.square_in_place();

        assert_eq!(a, b);
    }
}

fn random_expansion_tests<F: Field>() {
    for _ in 0..ITERATIONS {
        // Compare (a + b)(c + d) and (a*c + b*c + a*d + b*d)

        let a = F::rand();
        let b = F::rand();
        let c = F::rand();
        let d = F::rand();

        let mut t0 = a;
        t0 += &b;
        let mut t1 = c;
        t1 += &d;
        t0 *= &t1;

        let mut t2 = a;
        t2 *= &c;
        let mut t3 = b;
        t3 *= &c;
        let mut t4 = a;
        t4 *= &d;
        let mut t5 = b;
        t5 *= &d;

        t2 += &t3;
        t2 += &t4;
        t2 += &t5;

        assert_eq!(t0, t2);
    }

    for _ in 0..ITERATIONS {
        // Compare (a + b)c and (a*c + b*c)

        let a = F::rand();
        let b = F::rand();
        let c = F::rand();

        let t0 = (a + b) * c;
        let t2 = a * c + (b * c);

        assert_eq!(t0, t2);
    }
}

macro_rules! frobenius_test {
    ($name:ident, $field:ty) => {
        #[test]
        fn $name() {
            for _ in 0..ITERATIONS {
                let a = <$field>::rand();

                let mut a_0 = a;
                a_0.frobenius_map(0);
                assert_eq!(a, a_0);

                let mut a_q = a.pow(&<$field>::characteristic());
                for power in 1..13 {
                    let mut a_qi = a;
                    a_qi.frobenius_map(power);
                    assert_eq!(a_qi, a_q);

                    a_q = a_q.pow(&<$field>::characteristic());
                }
            }
        }
    };
}

pub fn sqrt_field_test<F: Field>(elem: F) {
    let square = elem.square();
    let sqrt = square.sqrt().unwrap();
    assert!(sqrt == elem || sqrt == -elem);
    if let Some(sqrt) = elem.sqrt() {
        assert!(sqrt.square() == elem || sqrt.square() == -elem);
    }
    random_sqrt_tests::<F>();
}

#[allow(clippy::eq_op)]
pub fn field_test<F: Field>(a: F, b: F) {
    let zero = F::ZERO;
    assert!(zero == zero);
    assert!(zero.is_zero()); // true
    assert!(!zero.is_one()); // false

    let one = F::ONE;
    assert!(one == one);
    assert!(!one.is_zero()); // false
    assert!(one.is_one()); // true
    assert_eq!(zero + one, one);

    let two = one + one;
    assert!(two == two);
    assert_ne!(zero, two);
    assert_ne!(one, two);

    // a == a
    assert!(a == a);
    // a + 0 = a
    assert_eq!(a + zero, a);
    // a - 0 = a
    assert_eq!(a - zero, a);
    // a - a = 0
    assert_eq!(a - a, zero);
    // 0 - a = -a
    assert_eq!(zero - a, -a);
    // a.double() = a + a
    assert_eq!(a.double(), a + a);
    // b.double() = b + b
    assert_eq!(b.double(), b + b);
    // a + b = b + a
    assert_eq!(a + b, b + a);
    // a - b = -(b - a)
    assert_eq!(a - b, -(b - a));
    // (a + b) + a = a + (b + a)
    assert_eq!((a + b) + a, a + (b + a));
    // (a + b).double() = (a + b) + (b + a)
    assert_eq!((a + b).double(), (a + b) + (b + a));
    // assert_eq!(F::half(), F::ONE.double().inverse().unwrap());

    // a * 0 = 0
    assert_eq!(a * zero, zero);
    // a * 1 = a
    assert_eq!(a * one, a);
    // a * 2 = a.double()
    assert_eq!(a * two, a.double());
    // a * a^-1 = 1
    assert_eq!(a * a.inverse().unwrap(), one);
    // a * a = a^2
    assert_eq!(a * a, a.square());
    // a * a * a = a^3
    assert_eq!(a * (a * a), a.pow(&[0x3, 0x0, 0x0, 0x0]));
    // a * b = b * a
    assert_eq!(a * b, b * a);
    // (a * b) * a = a * (b * a)
    assert_eq!((a * b) * a, a * (b * a));
    // (a + b)^2 = a^2 + 2ab + b^2
    assert_eq!(
        (a + b).square(),
        a.square() + ((a * b) + (a * b)) + b.square()
    );
    // (a - b)^2 = (-(b - a))^2
    assert_eq!((a - b).square(), (-(b - a)).square());

    let mut c = a.clone();
    c.inverse_in_place();
    assert_eq!(a * c, one);

    assert_eq!(a / a, one);

    for len in 0..10 {
        let mut a = Vec::new();
        let mut b = Vec::new();
        for _ in 0..len {
            a.push(F::rand());
            b.push(F::rand());
            assert_eq!(
                F::sum_of_products(a.clone().into_iter(), b.clone().into_iter()),
                a.iter().zip(b.iter()).map(|(x, y)| *x * y).sum()
            );
        }
    }

    random_negation_tests::<F>();
    random_addition_tests::<F>();
    random_subtraction_tests::<F>();
    random_multiplication_tests::<F>();
    random_inversion_tests::<F>();
    random_doubling_tests::<F>();
    random_squaring_tests::<F>();
    random_expansion_tests::<F>();

    assert!(F::ZERO.is_zero());
    {
        let z = -F::ZERO;
        assert!(z.is_zero());
    }

    assert!(F::ZERO.inverse().is_none());

    // Multiplication by zero
    {
        let a = F::rand() * F::ZERO;
        assert!(a.is_zero());
    }

    // Addition by zero
    {
        let mut a = F::rand();
        let copy = a;
        a += &F::ZERO;
        assert_eq!(a, copy);
    }
}

fn random_sqrt_tests<F: Field>() {
    for _ in 0..ITERATIONS {
        let a = F::rand();
        let b = a.square();
        // assert_eq!(b.legendre(), LegendreSymbol::QuadraticResidue);

        let b = b.sqrt().unwrap();
        assert!(a == b || a == -b);
    }

    let mut c = F::ONE;
    for _ in 0..ITERATIONS {
        let mut b = c.square();
        // assert_eq!(b.legendre(), LegendreSymbol::QuadraticResidue);

        b = b.sqrt().unwrap();

        if b != c {
            b = -b;
        }

        assert_eq!(b, c);

        c += &F::ONE;
    }
}

pub fn curve_tests<G: Projective>() {
    // Negation edge case with zero.
    {
        let z = -G::ZERO;
        assert!(z.is_zero());
    }

    // Doubling edge case with zero.
    {
        let mut z = -G::ZERO;
        z.double_in_place();
        assert!(z.is_zero());
    }

    // Addition edge cases with zero
    {
        let mut r = G::rand();
        let rcopy = r;
        r.add_assign(G::ZERO);
        assert_eq!(r, rcopy);
        r.add_assign_mixed(&G::Affine::ZERO);
        assert_eq!(r, rcopy);

        let mut z = G::ZERO;
        z.add_assign(G::ZERO);
        assert!(z.is_zero());
        z.add_assign_mixed(&G::Affine::ZERO);
        assert!(z.is_zero());

        let mut z2 = z;
        z2.add_assign(r);

        z.add_assign_mixed(&r.to_affine());

        assert_eq!(z, z2);
        assert_eq!(z, r);
    }

    // Transformations
    {
        let a = G::rand();
        let b = a.to_affine().to_projective();
        let c = a.to_affine().to_projective().to_affine().to_projective();
        assert_eq!(a, b);
        assert_eq!(b, c);
    }

    random_addition_test::<G>();
    random_multiplication_test::<G>();
    random_doubling_test::<G>();
    random_negation_test::<G>();
    random_transformation_test::<G>();
}

#[allow(clippy::eq_op)]
pub fn projective_test<G: Projective>(a: G, mut b: G) {
    let zero = G::ZERO;
    let fr_zero = Scalar::ZERO;
    let fr_one = Scalar::ONE;
    let fr_two = fr_one + fr_one;
    assert!(zero == zero);
    assert!(zero.is_zero()); // true
    assert_eq!(a.mul(fr_one), a);
    assert_eq!(a.mul(fr_two), a + a);
    assert_eq!(a.mul(fr_zero), zero);
    assert_eq!(a.mul(fr_zero) - a, -a);
    assert_eq!(a.mul(fr_one) - a, zero);
    assert_eq!(a.mul(fr_two) - a, a);

    // a == a
    assert!(a == a);
    // a + 0 = a
    assert_eq!(a + zero, a);
    // a - 0 = a
    assert_eq!(a - zero, a);
    // a - a = 0
    assert_eq!(a - a, zero);
    // 0 - a = -a
    assert_eq!(zero - a, -a);
    // a.double() = a + a
    assert_eq!(a.double(), a + a);
    // b.double() = b + b
    assert_eq!(b.double(), b + b);
    // a + b = b + a
    assert_eq!(a + b, b + a);
    // a - b = -(b - a)
    assert_eq!(a - b, -(b - a));
    // (a + b) + a = a + (b + a)
    assert_eq!((a + b) + a, a + (b + a));
    // (a + b).double() = (a + b) + (b + a)
    assert_eq!((a + b).double(), (a + b) + (b + a));

    // Check that double_in_place and double give the same result
    let original_b = b;
    b.double_in_place();
    assert_eq!(original_b.double(), b);

    let fr_rand1 = Scalar::rand();
    let fr_rand2 = Scalar::rand();
    let a_rand1 = a.mul(fr_rand1);
    let a_rand2 = a.mul(fr_rand2);
    let fr_three = fr_two + fr_rand1;
    let a_two = a.mul(fr_two);
    assert_eq!(a_two, a.double(), "(a * 2)  != a.double()");
    let a_six = a.mul(fr_three * fr_two);
    assert_eq!(a_two.mul(fr_three), a_six, "(a * 2) * 3 != a * (2 * 3)");

    assert_eq!(
        a_rand1.mul(fr_rand2),
        a_rand2.mul(fr_rand1),
        "(a * r1) * r2 != (a * r2) * r1"
    );
    assert_eq!(
        a_rand2.mul(fr_rand1),
        a.mul(fr_rand1 * fr_rand2),
        "(a * r2) * r1 != a * (r1 * r2)"
    );
    assert_eq!(
        a_rand1.mul(fr_rand2),
        a.mul(fr_rand1 * fr_rand2),
        "(a * r1) * r2 != a * (r1 * r2)"
    );
}

#[test]
fn test_bls12_377_fr() {
    for _ in 0..ITERATIONS {
        let a = Scalar::rand();
        let b = Scalar::rand();
        field_test(a, b);
        sqrt_field_test(b);
    }
}

#[test]
fn test_bls12_377_fp() {
    for _ in 0..ITERATIONS {
        let a = Fp::rand();
        let b = Fp::rand();
        field_test(a, b);
        sqrt_field_test(a);
    }
}

#[test]
fn test_bls12_377_fp2() {
    for _ in 0..ITERATIONS {
        let a = Fp2::rand();
        let b = Fp2::rand();
        field_test(a, b);
        sqrt_field_test(a);
    }
}

frobenius_test!(fp2_frobenius_test, Fp2);

#[test]
fn test_bls12_377_fp6() {
    for _ in 0..ITERATIONS {
        let g = Fp6::rand();
        let h = Fp6::rand();
        field_test(g, h);
    }
}

frobenius_test!(fp6_frobenius_test, Fp6);

#[test]
fn test_bls12_377_fp12() {
    for _ in 0..ITERATIONS {
        let g = Fp12::rand();
        let h = Fp12::rand();
        field_test(g, h);
    }
}

frobenius_test!(fp12_frobenius_test, Fp12);

#[test]
fn test_fp_is_half() {
    assert_eq!(Fp::half(), Fp::ONE.double().inverse().unwrap());
}

#[test]
fn test_fr_sum_of_products() {
    for i in [2, 4, 8, 16, 32] {
        let a = (0..i).map(|_| Scalar::rand()).collect::<Vec<_>>();
        let b = (0..i).map(|_| Scalar::rand()).collect::<Vec<_>>();
        assert_eq!(
            Scalar::sum_of_products(a.clone().into_iter(), b.clone().into_iter()),
            a.into_iter().zip(b).map(|(a, b)| a * b).sum()
        );
    }
}

#[test]
fn test_fp_sum_of_products() {
    for i in [2, 4, 8, 16, 32] {
        let a = (0..i).map(|_| Fp::rand()).collect::<Vec<_>>();
        let b = (0..i).map(|_| Fp::rand()).collect::<Vec<_>>();
        assert_eq!(
            Fp::sum_of_products(a.clone().into_iter(), b.clone().into_iter()),
            a.into_iter().zip(b).map(|(a, b)| a * b).sum()
        );
    }
}

#[test]
fn test_fp_add_assign() {
    // Test associativity

    for _ in 0..1000 {
        // Generate a, b, c and ensure (a + b) + c == a + (b + c).
        let a = Fp::rand();
        let b = Fp::rand();
        let c = Fp::rand();

        let mut tmp1 = a;
        tmp1.add_assign(b);
        tmp1.add_assign(c);

        let mut tmp2 = b;
        tmp2.add_assign(c);
        tmp2.add_assign(a);

        assert!(tmp1.is_valid());
        assert!(tmp2.is_valid());
        assert_eq!(tmp1, tmp2);
    }
}

#[test]
fn test_fp_sub_assign() {
    for _ in 0..1000 {
        // Ensure that (a - b) + (b - a) = 0.
        let a = Fp::rand();
        let b = Fp::rand();

        let mut tmp1 = a;
        tmp1.sub_assign(&b);

        let mut tmp2 = b;
        tmp2.sub_assign(&a);

        tmp1.add_assign(tmp2);
        assert!(tmp1.is_zero());
    }
}

#[test]
fn test_fp_mul_assign() {
    for _ in 0..1000000 {
        // Ensure that (a * b) * c = a * (b * c)
        let a = Fp::rand();
        let b = Fp::rand();
        let c = Fp::rand();

        let mut tmp1 = a;
        tmp1.mul_assign(&b);
        tmp1.mul_assign(&c);

        let mut tmp2 = b;
        tmp2.mul_assign(&c);
        tmp2.mul_assign(&a);

        assert_eq!(tmp1, tmp2);
    }

    for _ in 0..1000000 {
        // Ensure that r * (a + b + c) = r*a + r*b + r*c

        let r = Fp::rand();
        let mut a = Fp::rand();
        let mut b = Fp::rand();
        let mut c = Fp::rand();

        let mut tmp1 = a;
        tmp1.add_assign(b);
        tmp1.add_assign(c);
        tmp1.mul_assign(&r);

        a.mul_assign(&r);
        b.mul_assign(&r);
        c.mul_assign(&r);

        a.add_assign(b);
        a.add_assign(c);

        assert_eq!(tmp1, a);
    }
}

#[test]
fn test_fp_squaring() {
    for _ in 0..1000000 {
        // Ensure that (a * a) = a^2
        let a = Fp::rand();

        let mut tmp = a;
        tmp.square_in_place();

        let mut tmp2 = a;
        tmp2.mul_assign(&a);

        assert_eq!(tmp, tmp2);
    }
}

#[test]
fn test_fp_inverse() {
    assert!(Fp::ZERO.inverse().is_none());

    let one = Fp::ONE;

    for _ in 0..1000 {
        // Ensure that a * a^-1 = 1
        let mut a = Fp::rand();
        let ainv = a.inverse().unwrap();
        a.mul_assign(&ainv);
        assert_eq!(a, one);
    }
}

#[test]
fn test_fp_double_in_place() {
    for _ in 0..1000 {
        // Ensure doubling a is equivalent to adding a to itself.
        let mut a = Fp::rand();
        let mut b = a;
        b.add_assign(a);
        a.double_in_place();
        assert_eq!(a, b);
    }
}

#[test]
fn test_fp_negate() {
    {
        let a = -Fp::ZERO;

        assert!(a.is_zero());
    }

    for _ in 0..1000 {
        // Ensure (a - (-a)) = 0.
        let mut a = Fp::rand();
        let b = -a;
        a.add_assign(b);

        assert!(a.is_zero());
    }
}

#[test]
fn test_fp_pow() {
    for i in 0..1000 {
        // Exponentiate by various small numbers and ensure it consists with repeated
        // multiplication.
        let a = Fp::rand();
        let target = a.pow(&[i]);
        let mut c = Fp::ONE;
        for _ in 0..i {
            c.mul_assign(&a);
        }
        assert_eq!(c, target);
    }

    for _ in 0..1000 {
        // Exponentiating by the modulus should have no effect in a prime field.
        let a = Fp::rand();

        assert_eq!(a, a.pow(&Fp::characteristic()));
    }
}

#[test]
fn test_fp_sqrt() {
    assert_eq!(Fp::ZERO.sqrt().unwrap(), Fp::ZERO);

    for _ in 0..1000 {
        // Ensure sqrt(a^2) = a or -a
        let a = Fp::rand();
        let nega = -a;
        let mut b = a;
        b.square_in_place();

        let b = b.sqrt().unwrap();

        assert!(a == b || nega == b);
    }

    for _ in 0..1000 {
        // Ensure sqrt(a)^2 = a for random a
        let a = Fp::rand();

        if let Some(mut tmp) = a.sqrt() {
            tmp.square_in_place();

            assert_eq!(a, tmp);
        }
    }
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
fn test_fp2_mul_nonresidue() {
    let nqr = Fp2::new(Fp::ZERO, Fp::ONE);

    let quadratic_non_residue = Fp2::new(fp2::QUADRATIC_NONRESIDUE.0, fp2::QUADRATIC_NONRESIDUE.1);
    for _ in 0..1000 {
        let mut a = Fp2::rand();
        let mut b = a;
        a = quadratic_non_residue * a;
        b.mul_assign(&nqr);

        assert_eq!(a, b);
    }
}

#[test]
fn test_fp6_mul_by_1() {
    for _ in 0..1000 {
        let c1 = Fp2::rand();
        let mut a = Fp6::rand();
        let mut b = a;

        a.mul_by_1(&c1);
        b.mul_assign(&Fp6::new(Fp2::ZERO, c1, Fp2::ZERO));

        assert_eq!(a, b);
    }
}

#[test]
fn test_fp6_mul_by_01() {
    for _ in 0..1000 {
        let c0 = Fp2::rand();
        let c1 = Fp2::rand();
        let mut a = Fp6::rand();
        let mut b = a;

        a.mul_by_01(&c0, &c1);
        b.mul_assign(&Fp6::new(c0, c1, Fp2::ZERO));

        assert_eq!(a, b);
    }
}

#[test]
fn test_fp12_mul_by_014() {
    for _ in 0..1000 {
        let c0 = Fp2::rand();
        let c1 = Fp2::rand();
        let c5 = Fp2::rand();
        let mut a = Fp12::rand();
        let mut b = a;

        a.mul_by_014(&c0, &c1, &c5);
        b.mul_assign(&Fp12::new(
            Fp6::new(c0, c1, Fp2::ZERO),
            Fp6::new(Fp2::ZERO, c5, Fp2::ZERO),
        ));

        assert_eq!(a, b);
    }
}

#[test]
fn test_fp12_mul_by_034() {
    for _ in 0..1000 {
        let c0 = Fp2::rand();
        let c3 = Fp2::rand();
        let c4 = Fp2::rand();
        let mut a = Fp12::rand();
        let mut b = a;

        a.mul_by_034(&c0, &c3, &c4);
        b.mul_assign(&Fp12::new(
            Fp6::new(c0, Fp2::ZERO, Fp2::ZERO),
            Fp6::new(c3, c4, Fp2::ZERO),
        ));

        assert_eq!(a, b);
    }
}

#[test]
fn test_g1_projective_glv() {
    let point = G1Projective::rand();
    let scalar = Scalar::rand();
    let affine = point.to_affine();
    assert_eq!(point.mul(scalar), affine.mul(scalar));
    assert_eq!(
        affine.mul(scalar),
        affine.mul_bits(
            scalar
                .0
                .as_limbs()
                .iter()
                .flat_map(|limb| limb.view_bits::<Lsb0>())
                .map(|bit| *bit)
                .rev()
                .collect::<Vec<_>>()
        )
    );
}

#[test]
fn test_g1_projective_curve() {
    curve_tests::<G1Projective>();
}

#[test]
fn test_g1_projective_group() {
    let a = G1Projective::rand();
    let b = G1Projective::rand();
    projective_test(a, b);
}

#[test]
fn test_g1_generator() {
    let generator = G1Affine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn test_g2_projective_curve() {
    curve_tests::<G2Projective>();
}

#[test]
fn test_g2_projective_group() {
    let a = G2Projective::rand();
    let b = G2Projective::rand();
    projective_test(a, b);
}

#[test]
fn test_g2_generator() {
    let generator = G2Affine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn test_bilinearity() {
    let a = G1Projective::rand();
    let b = G2Projective::rand();
    let s = Scalar::rand();

    let sa = a * s;
    let sb = b * s;

    let ans1 = pairing(sa, b);
    let ans2 = pairing(a, sb);
    let ans3 = pairing(a, b).pow(s.0.as_limbs());

    assert_eq!(ans1, ans2);
    assert_eq!(ans2, ans3);

    assert_ne!(ans1, Fp12::ONE);
    assert_ne!(ans2, Fp12::ONE);
    assert_ne!(ans3, Fp12::ONE);

    assert_eq!(ans1.pow(&Scalar::characteristic()), Fp12::ONE);
    assert_eq!(ans2.pow(&Scalar::characteristic()), Fp12::ONE);
    assert_eq!(ans3.pow(&Scalar::characteristic()), Fp12::ONE);
}
