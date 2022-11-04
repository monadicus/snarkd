use crate::bls12_377::{field::Field, Fq, LegendreSymbol};
use core::{
    cmp::Ordering,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use ruint::uint;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Fq2 {
    pub c0: Fq,
    pub c1: Fq,
}

/// Coefficients for the Frobenius automorphism.
pub const FROBENIUS_COEFF_FP2_C1: [Fq; 2] = [
    // NONRESIDUE**(((q^0) - 1) / 2)
    Fq(uint!(1_U384)),
    // NONRESIDUE**(((q^1) - 1) / 2)
    Fq(
        uint!(258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458176_U384),
    ),
];

/// NONRESIDUE = -5
pub const NONRESIDUE: Fq = Fq(
    uint!(258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458172_U384),
);

/// QUADRATIC_NONRESIDUE = U
pub const QUADRATIC_NONRESIDUE: (Fq, Fq) = (Fq(uint!(0_U384)), Fq(uint!(1_U384)));

impl Fq2 {
    pub fn new(c0: Fq, c1: Fq) -> Self {
        Self { c0, c1 }
    }

    pub fn mul_fp_by_nonresidue(fe: &Fq) -> Fq {
        NONRESIDUE * fe
    }

    pub fn mul_by_fp(&mut self, other: &Fq) {
        self.c0 *= other;
        self.c1 *= other;
    }

    /// Norm of Fp2 over Fp: Norm(a) = a.x^2 - beta * a.y^2
    pub fn norm(&self) -> Fq {
        let t0 = self.c0.square();
        let mut t1 = self.c1.square();
        t1 = -Self::mul_fp_by_nonresidue(&t1);
        t1.add_assign(t0);
        t1
    }

    pub fn legendre(&self) -> LegendreSymbol {
        self.norm().legendre()
    }
}

impl Field for Fq2 {
    const PHI: Fq2 = Fq2 {
        c0: Fq(uint!(0_U384)),
        c1: Fq(
            uint!(258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047231_U384),
        ),
    };

    fn zero() -> Self {
        Fq2 {
            c0: Fq(uint!(0_U384)),
            c1: Fq(uint!(0_U384)),
        }
    }

    fn is_zero(&self) -> bool {
        self.c0.is_zero() && self.c1.is_zero()
    }

    fn one() -> Self {
        Fq2 {
            c0: Fq(uint!(1_U384)),
            c1: Fq(uint!(0_U384)),
        }
    }

    fn is_one(&self) -> bool {
        self.c0.is_one() && self.c1.is_zero()
    }

    fn rand() -> Self {
        Self {
            c0: Fq::rand(),
            c1: Fq::rand(),
        }
    }

    fn characteristic() -> Self {
        unreachable!("unhit");
        Fq2 {
            c0: Fq::characteristic(),
            c1: Fq::zero(),
        }
    }

    fn double(&self) -> Self {
        let mut tmp = *self;
        tmp.double_in_place();
        tmp
    }

    fn double_in_place(&mut self) {
        self.c0.double_in_place();
        self.c1.double_in_place();
    }

    fn square(&self) -> Self {
        let mut tmp = *self;
        tmp.square_in_place();
        tmp
    }

    fn square_in_place(&mut self) {
        // v0 = c0 - c1
        let mut v0 = self.c0 - self.c1;
        // v3 = c0 - beta * c1
        let v3 = self.c0 - Self::mul_fp_by_nonresidue(&self.c1);
        // v2 = c0 * c1
        let v2 = self.c0 * self.c1;

        // v0 = (v0 * v3) + v2
        v0 *= &v3;
        v0 += &v2;

        self.c1 = v2.double();
        self.c0 = v0 + Self::mul_fp_by_nonresidue(&v2);
    }

    fn inverse(&self) -> Option<Self> {
        if self.is_zero() {
            None
        } else {
            // Guide to Pairing-based Cryptography, Algorithm 5.19.
            // v0 = c0.square()
            let mut v0 = self.c0.square();
            // v1 = c1.square()
            let v1 = self.c1.square();
            // v0 = v0 - beta * v1
            v0 -= Self::mul_fp_by_nonresidue(&v1);
            v0.inverse().map(|v1| {
                let c0 = self.c0 * v1;
                let c1 = -(self.c1 * v1);
                Self::new(c0, c1)
            })
        }
    }

    fn inverse_in_place(&mut self) -> Option<&mut Self> {
        if let Some(inv) = self.inverse() {
            unreachable!("unhit");
            *self = inv;
            Some(self)
        } else {
            unreachable!("unhit");
            None
        }
    }

    fn sqrt(&self) -> Option<Self> {
        if self.c1.is_zero() {
            return self.c0.sqrt().map(|c0| Self::new(c0, Fq::zero()));
        }
        match self.legendre() {
            // Square root based on the complex method. See
            // https://eprint.iacr.org/2012/685.pdf (page 15, algorithm 8)
            LegendreSymbol::Zero => unreachable!("unhit"), // Some(*self),
            LegendreSymbol::QuadraticNonResidue => None,
            LegendreSymbol::QuadraticResidue => {
                let two_inv = Fq::half();
                let alpha = self
                    .norm()
                    .sqrt()
                    .expect("We are in the QR case, the norm should have a square root");
                let mut delta = (alpha + self.c0) * two_inv;
                if matches!(delta.legendre(), LegendreSymbol::QuadraticNonResidue) {
                    delta -= &alpha;
                }
                let c0 = delta.sqrt().expect("Delta must have a square root");
                let c0_inv = c0.inverse().expect("c0 must have an inverse");
                Some(Self::new(c0, self.c1 * two_inv * c0_inv))
            }
        }
    }

    fn frobenius_map(&mut self, power: usize) {
        self.c1 *= FROBENIUS_COEFF_FP2_C1[power % 2];
    }

    fn glv_endomorphism(&self) -> Self {
        let mut tmp = *self;
        tmp.mul_by_fp(&Self::PHI.c1);
        tmp
    }
}

impl Add for Fq2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            c0: self.c0 + other.c0,
            c1: self.c1 + other.c1,
        }
    }
}

impl AddAssign for Fq2 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Fq2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            c0: self.c0 - other.c0,
            c1: self.c1 - other.c1,
        }
    }
}

impl SubAssign for Fq2 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Neg for Fq2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            c0: -self.c0,
            c1: -self.c1,
        }
    }
}

impl Mul for Fq2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut tmp = self;
        tmp *= other;
        tmp
    }
}

impl MulAssign for Fq2 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self::new(
            Fq::sum_of_products(
                [self.c0, Self::mul_fp_by_nonresidue(&self.c1)].into_iter(),
                [other.c0, other.c1].into_iter(),
            ),
            Fq::sum_of_products(
                [self.c0, self.c1].into_iter(),
                [other.c1, other.c0].into_iter(),
            ),
        );
    }
}

impl Div for Fq2 {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, other: Self) -> Self {
        unreachable!("unhit");
        self * other.inverse().unwrap()
    }
}

impl DivAssign for Fq2 {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn div_assign(&mut self, other: Self) {
        unreachable!("unhit");
        *self *= other.inverse().unwrap();
    }
}

impl<'a> Add<&'a Self> for Fq2 {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        Self {
            c0: self.c0 + other.c0,
            c1: self.c1 + other.c1,
        }
    }
}

impl<'a> AddAssign<&'a Self> for Fq2 {
    fn add_assign(&mut self, other: &Self) {
        *self = *self + other;
    }
}

impl<'a> Sub<&'a Self> for Fq2 {
    type Output = Self;

    fn sub(self, other: &Self) -> Self {
        Self {
            c0: self.c0 - other.c0,
            c1: self.c1 - other.c1,
        }
    }
}

impl<'a> SubAssign<&'a Self> for Fq2 {
    fn sub_assign(&mut self, other: &Self) {
        *self = *self - other;
    }
}

impl<'a> Mul<&'a Self> for Fq2 {
    type Output = Self;

    fn mul(self, other: &Self) -> Self {
        let mut tmp = self;
        tmp *= other;
        tmp
    }
}

impl<'a> MulAssign<&'a Self> for Fq2 {
    fn mul_assign(&mut self, other: &Self) {
        *self = Self::new(
            Fq::sum_of_products(
                [self.c0, Self::mul_fp_by_nonresidue(&self.c1)].into_iter(),
                [other.c0, other.c1].into_iter(),
            ),
            Fq::sum_of_products(
                [self.c0, self.c1].into_iter(),
                [other.c1, other.c0].into_iter(),
            ),
        );
    }
}

impl<'a> Div<&'a Self> for Fq2 {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, other: &Self) -> Self {
        unreachable!("unhit");
        self * other.inverse().unwrap()
    }
}

impl<'a> DivAssign<&'a Self> for Fq2 {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn div_assign(&mut self, other: &Self) {
        unreachable!("unhit");
        *self *= other.inverse().unwrap();
    }
}

impl Sum<Fq2> for Fq2 {
    /// Returns the `sum` of `self` and `other`.
    fn sum<I: Iterator<Item = Fq2>>(iter: I) -> Self {
        iter.fold(Fq2::zero(), |a, b| a + b)
    }
}

impl std::fmt::Display for Fq2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unreachable!("unhit");
        write!(f, "Fp2({} + {} * u)", self.c0, self.c1)
    }
}

/// `Fp2` elements are ordered lexicographically.
impl Ord for Fq2 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.c1.cmp(&other.c1) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.c0.cmp(&other.c0),
        }
    }
}

impl PartialOrd for Fq2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
