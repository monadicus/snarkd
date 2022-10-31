use crate::bls12_377::{field::Field, Fq};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use ruint::uint;

#[derive(Copy, Clone)]
pub struct Fq2 {
    pub c0: Fq,
    pub c1: Fq,
}

/// Coefficients for the Frobenius automorphism.
const FROBENIUS_COEFF_FP2_C1: [Fq; 2] = [
    // NONRESIDUE**(((q^0) - 1) / 2)
    uint!(1_U384),
    // NONRESIDUE**(((q^1) - 1) / 2)
    uint!(258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458176_U384),
];

/// NONRESIDUE = -5
const NONRESIDUE: Fq = uint!(258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458172_U384);

/// QUADRATIC_NONRESIDUE = U
const QUADRATIC_NONRESIDUE: (Fq, Fq) = (uint!(0_U384), uint!(1_U384));

impl Fq2 {
    #[inline(always)]
    fn mul_fp_by_nonresidue(fe: &Fq) -> Fq {
        let original = fe;
        let mut fe = -fe.double();
        fe.double_in_place();
        fe - original
    }
}

impl Field for Fq2 {
    const PHI: Fq2 = Fq2 {
        c0: uint!(0_U384),
        c1: uint!(258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047231_U384),
    };

    fn zero() -> Self {
        Fq2 {
            c0: uint!(0_U256),
            c1: uint!(0_U256),
        }
    }

    fn is_zero(&self) -> bool {
        self.c0.is_zero() && self.c1.is_zero()
    }

    fn one() -> Self {
        Fq2 {
            c0: uint!(1_U256),
            c1: uint!(0_U256),
        }
    }

    fn is_one(&self) -> bool {
        self.c0.is_one() && self.c1.is_zero()
    }

    fn characteristic() -> Self {
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

    // NOTE: check this
    fn square_in_place(&mut self) {
        let c0 = self.c0;
        let c1 = self.c1;
        self.c0 = c0.square() - (c1.square() * NONRESIDUE);
        self.c1 = (c0 + c1).square() - c0.square() - c1.square();
    }

    fn inverse(&self) -> Option<Self> {
        let mut t0 = self.c0.square();
        let mut t1 = self.c1.square();
        t1 *= NONRESIDUE;
        t0 -= t1;
        t0.inverse().map(|t0| {
            let mut tmp = *self;
            tmp.c0 *= t0;
            tmp.c1 *= t0;
            tmp.c1 = -tmp.c1;
            tmp
        })
    }

    fn inverse_in_place(&mut self) -> Option<&mut Self> {
        if let Some(inv) = self.inverse() {
            *self = inv;
            Some(self)
        } else {
            None
        }
    }

    fn frobenius_map(&mut self, power: usize) {
        self.c1 *= FROBENIUS_COEFF_FP2_C1[power % 2];
    }

    fn glv_endomorphism(&self) -> Self {
        let p = self.mul_by_fp(&Self::PHI.c1);
        p
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
        *self = self + other;
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
        *self = self - other;
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
        let c0 = self.c0;
        let c1 = self.c1;
        let c2 = other.c0;
        let c3 = other.c1;

        self.c0 = c0 * c2 - (c1 * c3 * NONRESIDUE);
        self.c1 = (c0 + c1) * (c2 + c3) - c0 * c2 - c1 * c3;
    }
}

impl Mul<Fq> for Fq2 {
    type Output = Self;

    fn mul(self, other: Fq) -> Self {
        Self {
            c0: self.c0 * other,
            c1: self.c1 * other,
        }
    }
}

impl MulAssign<Fq> for Fq2 {
    fn mul_assign(&mut self, other: Fq) {
        self.c0 *= other;
        self.c1 *= other;
    }
}

impl Div for Fq2 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self * other.inverse().unwrap()
    }
}

impl DivAssign for Fq2 {
    fn div_assign(&mut self, other: Self) {
        *self *= other.inverse().unwrap();
    }
}
