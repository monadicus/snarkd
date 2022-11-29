use crate::bls12_377::{
    field::Field, fp, fp2, pairing, Affine, Fp, Fp12, Fp2, Fp6, G1Affine, G1Projective, G2Affine,
    G2Projective, LegendreSymbol, Projective, Scalar,
};
use bitvec::prelude::*;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use ruint::uint;
use std::{
    cmp::Ordering,
    ops::{AddAssign, Mul, MulAssign, SubAssign},
};

pub(crate) const ITERATIONS: usize = 10;

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
