use crate::bls12_377::{field::Field, Fp, Fp2};
use core::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};
use ruint::uint;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Fp6 {
    pub c0: Fp2,
    pub c1: Fp2,
    pub c2: Fp2,
}

const FROBENIUS_COEFF_FP6_C1: [Fp2; 6] = [
    // Fp2::NONRESIDUE^(((q^0) - 1) / 3)
    Fp2 {
        c0: Fp(uint!(1_U384)),
        c1: Fp(uint!(0_U384)),
    },
    // Fp2::NONRESIDUE^(((q^1) - 1) / 3)
    Fp2 {
        c0: Fp(
            uint!(80949648264912719408558363140637477264845294720710499478137287262712535938301461879813459410946_U384),
        ),
        c1: Fp(uint!(0_U384)),
    },
    // Fp2::NONRESIDUE^(((q^2) - 1) / 3)
    Fp2 {
        c0: Fp(
            uint!(80949648264912719408558363140637477264845294720710499478137287262712535938301461879813459410945_U384),
        ),
        c1: Fp(uint!(0_U384)),
    },
    // Fp2::NONRESIDUE^(((q^3) - 1) / 3)
    Fp2 {
        c0: Fp(
            uint!(258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458176_U384),
        ),
        c1: Fp(uint!(0_U384)),
    },
    // Fp2::NONRESIDUE^(((q^4) - 1) / 3)
    Fp2 {
        c0: Fp(
            uint!(258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047231_U384),
        ),
        c1: Fp(uint!(0_U384)),
    },
    // Fp2::NONRESIDUE^(((q^5) - 1) / 3)
    Fp2 {
        c0: Fp(
            uint!(258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047232_U384),
        ),
        c1: Fp(uint!(0_U384)),
    },
];

const FROBENIUS_COEFF_FP6_C2: [Fp2; 6] = [
    // Fp2::NONRESIDUE^((2*(q^0) - 2) / 3)
    Fp2 {
        c0: Fp(uint!(1_U384)),
        c1: Fp(uint!(0_U384)),
    },
    // Fp2::NONRESIDUE^((2*(q^1) - 2) / 3)
    Fp2 {
        c0: Fp(
            uint!(80949648264912719408558363140637477264845294720710499478137287262712535938301461879813459410945_U384),
        ),
        c1: Fp(uint!(0_U384)),
    },
    // Fp2::NONRESIDUE^((2*(q^2) - 2) / 3)
    Fp2 {
        c0: Fp(
            uint!(258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047231_U384),
        ),
        c1: Fp(uint!(0_U384)),
    },
    // Fp2::NONRESIDUE^((2*(q^3) - 2) / 3)
    Fp2 {
        c0: Fp(uint!(1_U384)),
        c1: Fp(uint!(0_U384)),
    },
    // Fp2::NONRESIDUE^((2*(q^4) - 2) / 3)
    Fp2 {
        c0: Fp(
            uint!(80949648264912719408558363140637477264845294720710499478137287262712535938301461879813459410945_U384),
        ),
        c1: Fp(uint!(0_U384)),
    },
    // Fp2::NONRESIDUE^((2*(q^5) - 2) / 3)
    Fp2 {
        c0: Fp(
            uint!(258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047231_U384),
        ),
        c1: Fp(uint!(0_U384)),
    },
];

impl Fp6 {
    pub fn new(c0: Fp2, c1: Fp2, c2: Fp2) -> Self {
        Self { c0, c1, c2 }
    }

    pub fn frobenius_map(&mut self, power: usize) {
        self.c0.frobenius_map(power);
        self.c1.frobenius_map(power);
        self.c2.frobenius_map(power);

        self.c1.mul_assign(FROBENIUS_COEFF_FP6_C1[power % 6]);
        self.c2.mul_assign(FROBENIUS_COEFF_FP6_C2[power % 6]);
    }
}

impl Field for Fp6 {
    // We don't need to perform GLV endomorphisms in Fp6.
    const PHI: Fp6 = Fp6 {
        c0: Fp2 {
            c0: Fp(uint!(1_U384)),
            c1: Fp(uint!(0_U384)),
        },
        c1: Fp2 {
            c0: Fp(uint!(0_U384)),
            c1: Fp(uint!(0_U384)),
        },
        c2: Fp2 {
            c0: Fp(uint!(0_U384)),
            c1: Fp(uint!(0_U384)),
        },
    };

    const ZERO: Fp6 = Self {
        c0: Fp2::ZERO,
        c1: Fp2::ZERO,
        c2: Fp2::ZERO,
    };

    const ONE: Fp6 = Self {
        c0: Fp2::ONE,
        c1: Fp2::ZERO,
        c2: Fp2::ZERO,
    };

    fn is_zero(&self) -> bool {
        self.c0.is_zero() && self.c1.is_zero() && self.c2.is_zero()
    }

    fn is_one(&self) -> bool {
        self.c0.is_one() && self.c1.is_zero() && self.c2.is_zero()
    }

    fn half() -> Self {
        Self {
            c0: Fp2::half(),
            c1: Fp2::ZERO,
            c2: Fp2::ZERO,
        }
    }

    fn rand() -> Self {
        Self {
            c0: Fp2::rand(),
            c1: Fp2::rand(),
            c2: Fp2::rand(),
        }
    }

    fn characteristic() -> Vec<u64> {
        Fp::characteristic()
    }

    fn double(&self) -> Self {
        Self {
            c0: self.c0.double(),
            c1: self.c1.double(),
            c2: self.c2.double(),
        }
    }

    fn double_in_place(&mut self) {
        self.c0.double_in_place();
        self.c1.double_in_place();
        self.c2.double_in_place();
    }

    fn square(&self) -> Self {
        let mut tmp = *self;
        tmp.square_in_place();
        tmp
    }

    fn square_in_place(&mut self) {
        let s0 = self.c0.square();
        let s1 = (self.c0 * self.c1).double();
        let s2 = (self.c0 - self.c1 + self.c2).square();
        let s3 = (self.c1 * self.c2).double();
        let s4 = self.c2.square();

        self.c0 = s0 + Self::mul_fp2_by_nonresidue(&s3);
        self.c1 = s1 + Self::mul_fp2_by_nonresidue(&s4);
        self.c2 = s1 + s2 + s3 - s0 - s4;
    }

    fn inverse(&self) -> Option<Self> {
        if self.is_zero() {
            None
        } else {
            let mut c0 = self.c2;
            c0 = Self::mul_fp2_by_nonresidue(&c0);
            c0.mul_assign(&self.c1);
            c0 = c0.neg();
            {
                let mut c0s = self.c0;
                c0s.square_in_place();
                c0.add_assign(c0s);
            }
            let mut c1 = self.c2;
            c1.square_in_place();
            c1 = Self::mul_fp2_by_nonresidue(&c1);
            {
                let mut c01 = self.c0;
                c01.mul_assign(&self.c1);
                c1.sub_assign(&c01);
            }
            let mut c2 = self.c1;
            c2.square_in_place();
            {
                let mut c02 = self.c0;
                c02.mul_assign(&self.c2);
                c2.sub_assign(&c02);
            }

            let mut tmp1 = self.c2;
            tmp1.mul_assign(&c1);
            let mut tmp2 = self.c1;
            tmp2.mul_assign(&c2);
            tmp1.add_assign(tmp2);
            tmp1 = Self::mul_fp2_by_nonresidue(&tmp1);
            tmp2 = self.c0;
            tmp2.mul_assign(&c0);
            tmp1.add_assign(tmp2);

            tmp1.inverse().map(|t| Self::new(t * c0, t * c1, t * c2))
        }
    }

    fn inverse_in_place(&mut self) -> Option<&mut Self> {
        if let Some(inv) = self.inverse() {
            *self = inv;
            Some(self)
        } else {
            None
        }
    }

    // No-op
    fn sqrt(&self) -> Option<Self> {
        None
    }

    fn glv_endomorphism(&self) -> Self {
        Self::ZERO
    }
}

impl Add for Fp6 {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self.add_assign(other);
        self
    }
}

impl AddAssign for Fp6 {
    fn add_assign(&mut self, other: Self) {
        self.c0.add_assign(other.c0);
        self.c1.add_assign(other.c1);
        self.c2.add_assign(other.c2);
    }
}

impl Sub for Fp6 {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        self.sub_assign(other);
        self
    }
}

impl SubAssign for Fp6 {
    fn sub_assign(&mut self, other: Self) {
        self.c0.sub_assign(other.c0);
        self.c1.sub_assign(other.c1);
        self.c2.sub_assign(other.c2);
    }
}

impl Neg for Fp6 {
    type Output = Self;

    fn neg(self) -> Self {
        Fp6 {
            c0: self.c0.neg(),
            c1: self.c1.neg(),
            c2: self.c2.neg(),
        }
    }
}

impl Mul for Fp6 {
    type Output = Self;

    fn mul(mut self, other: Self) -> Self {
        self.mul_assign(other);
        self
    }
}

impl MulAssign for Fp6 {
    fn mul_assign(&mut self, other: Self) {
        let v0 = self.c0 * other.c0;
        let v1 = self.c1 * other.c1;
        let v2 = self.c2 * other.c2;

        let c0 =
            Self::mul_fp2_by_nonresidue(&((self.c1 + self.c2) * (other.c1 + other.c2) - v1 - v2))
                + v0;
        let c1 = (self.c0 + self.c1) * (other.c0 + other.c1) - v0 - v1
            + Self::mul_fp2_by_nonresidue(&v2);
        let c2 = (self.c0 + self.c2) * (other.c0 + other.c2) - v0 - v2 + v1;

        self.c0 = c0;
        self.c1 = c1;
        self.c2 = c2;
    }
}

impl Div for Fp6 {
    type Output = Self;

    fn div(mut self, other: Self) -> Self {
        self.div_assign(other);
        self
    }
}

impl DivAssign for Fp6 {
    fn div_assign(&mut self, other: Self) {
        self.mul_assign(other.inverse().unwrap());
    }
}

impl<'a> Add<&'a Self> for Fp6 {
    type Output = Self;

    fn add(mut self, other: &Self) -> Self {
        self.add_assign(other);
        self
    }
}

impl<'a> AddAssign<&'a Self> for Fp6 {
    fn add_assign(&mut self, other: &Self) {
        self.c0.add_assign(other.c0);
        self.c1.add_assign(other.c1);
        self.c2.add_assign(other.c2);
    }
}

impl<'a> Sub<&'a Self> for Fp6 {
    type Output = Self;

    fn sub(mut self, other: &Self) -> Self {
        self.sub_assign(other);
        self
    }
}

impl<'a> SubAssign<&'a Self> for Fp6 {
    fn sub_assign(&mut self, other: &Self) {
        self.c0.sub_assign(other.c0);
        self.c1.sub_assign(other.c1);
        self.c2.sub_assign(other.c2);
    }
}

impl<'a> Mul<&'a Self> for Fp6 {
    type Output = Self;

    fn mul(mut self, other: &Self) -> Self {
        self.mul_assign(other);
        self
    }
}

impl<'a> MulAssign<&'a Self> for Fp6 {
    fn mul_assign(&mut self, other: &Self) {
        let v0 = self.c0 * other.c0;
        let v1 = self.c1 * other.c1;
        let v2 = self.c2 * other.c2;

        let c0 =
            Self::mul_fp2_by_nonresidue(&((self.c1 + self.c2) * (other.c1 + other.c2) - v1 - v2))
                + v0;
        let c1 = (self.c0 + self.c1) * (other.c0 + other.c1) - v0 - v1
            + Self::mul_fp2_by_nonresidue(&v2);
        let c2 = (self.c0 + self.c2) * (other.c0 + other.c2) - v0 - v2 + v1;

        self.c0 = c0;
        self.c1 = c1;
        self.c2 = c2;
    }
}

impl<'a> Div<&'a Self> for Fp6 {
    type Output = Self;

    fn div(mut self, other: &Self) -> Self {
        self.div_assign(other);
        self
    }
}

impl<'a> DivAssign<&'a Self> for Fp6 {
    fn div_assign(&mut self, other: &Self) {
        self.mul_assign(other.inverse().unwrap());
    }
}

/// NONRESIDUE = U
#[allow(dead_code)]
const NONRESIDUE: Fp2 = Fp2 {
    c0: Fp(uint!(0_U384)),
    c1: Fp(uint!(1_U384)),
};

impl Fp6 {
    pub fn mul_fp2_by_nonresidue(fe: &Fp2) -> Fp2 {
        // Karatsuba multiplication with constant other = u.
        let c0 = Fp2::mul_fp_by_nonresidue(&fe.c1);
        let c1 = fe.c0;
        Fp2 { c0, c1 }
    }

    pub fn mul_by_1(&mut self, c1: &Fp2) {
        let mut b_b = self.c1;
        b_b.mul_assign(c1);

        let mut t1 = *c1;
        {
            let mut tmp = self.c1;
            tmp.add_assign(self.c2);

            t1.mul_assign(&tmp);
            t1.sub_assign(&b_b);
            t1 = Self::mul_fp2_by_nonresidue(&t1);
        }

        let mut t2 = *c1;
        {
            let mut tmp = self.c0;
            tmp.add_assign(self.c1);

            t2.mul_assign(&tmp);
            t2.sub_assign(&b_b);
        }

        self.c0 = t1;
        self.c1 = t2;
        self.c2 = b_b;
    }

    pub fn mul_by_01(&mut self, c0: &Fp2, c1: &Fp2) {
        let mut a_a = self.c0;
        let mut b_b = self.c1;
        a_a.mul_assign(c0);
        b_b.mul_assign(c1);

        let mut t1 = *c1;
        {
            let mut tmp = self.c1;
            tmp.add_assign(self.c2);

            t1.mul_assign(&tmp);
            t1.sub_assign(&b_b);
            t1 = Self::mul_fp2_by_nonresidue(&t1);
            t1.add_assign(a_a);
        }

        let mut t3 = *c0;
        {
            let mut tmp = self.c0;
            tmp.add_assign(self.c2);

            t3.mul_assign(&tmp);
            t3.sub_assign(&a_a);
            t3.add_assign(b_b);
        }

        let mut t2 = *c0;
        t2.add_assign(c1);
        {
            let mut tmp = self.c0;
            tmp.add_assign(self.c1);

            t2.mul_assign(&tmp);
            t2.sub_assign(&a_a);
            t2.sub_assign(&b_b);
        }

        self.c0 = t1;
        self.c1 = t2;
        self.c2 = t3;
    }
}

impl Sum<Fp6> for Fp6 {
    /// Returns the `sum` of `self` and `other`.
    fn sum<I: Iterator<Item = Fp6>>(iter: I) -> Self {
        iter.fold(Fp6::ZERO, |a, b| a + b)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fq2_mul_nonresidue() {
        let nqr = Fp2::new(Fp::ZERO, Fp::ONE);
        println!("One: {:?}", Fp::ONE);

        for _ in 0..1000 {
            let mut a = Fp2::rand();
            let mut b = a;
            a *= &NONRESIDUE;
            b *= &nqr;

            assert_eq!(a, b);
        }
    }
}
