use crate::bls12_377::{field::Field, Fq, Fq2, Fq6};
use bitvec::prelude::*;
use core::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};
use ruint::uint;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Fq12 {
    pub c0: Fq6,
    pub c1: Fq6,
}

const FROBENIUS_COEFF_FP12_C1: [Fq2; 12] = [
    // Fp2::NONRESIDUE^(((q^0) - 1) / 6)
    Fq2 {
        c0: Fq(uint!(1_U384)),
        c1: Fq(uint!(0_U384)),
    },
    // Fp2::NONRESIDUE^(((q^1) - 1) / 6)
    Fq2 {
        c0: Fq(
            uint!(92949345220277864758624960506473182677953048909283248980960104381795901929519566951595905490535835115111760994353_U384),
        ),
        c1: Fq(uint!(0_U384)),
    },
    // Fp2::NONRESIDUE^(((q^2) - 1) / 6)
    Fq2 {
        c0: Fq(
            uint!(80949648264912719408558363140637477264845294720710499478137287262712535938301461879813459410946_U384),
        ),
        c1: Fq(uint!(0_U384)),
    },
    // Fp2::NONRESIDUE^(((q^3) - 1) / 6)
    Fq2 {
        c0: Fq(
            uint!(216465761340224619389371505802605247630151569547285782856803747159100223055385581585702401816380679166954762214499_U384),
        ),
        c1: Fq(uint!(0_U384)),
    },
    // Fp2::NONRESIDUE^(((q^4) - 1) / 6)
    Fq2 {
        c0: Fq(
            uint!(80949648264912719408558363140637477264845294720710499478137287262712535938301461879813459410945_U384),
        ),
        c1: Fq(uint!(0_U384)),
    },
    // Fp2::NONRESIDUE^(((q^5) - 1) / 6)
    Fq2 {
        c0: Fq(
            uint!(123516416119946754630746545296132064952198520638002533875843642777304321125866014634106496325844844051843001220146_U384),
        ),
        c1: Fq(uint!(0_U384)),
    },
    // Fp2::NONRESIDUE^(((q^6) - 1) / 6)
    Fq2 {
        c0: Fq(
            uint!(258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458176_U384),
        ),
        c1: Fq(uint!(0_U384)),
    },
    // Fp2::NONRESIDUE^(((q^7) - 1) / 6)
    Fq2 {
        c0: Fq(
            uint!(165715080792691229252027773188420350858440463845631411558924158284924566418821255823372982649037525009328560463824_U384),
        ),
        c1: Fq(uint!(0_U384)),
    },
    // Fp2::NONRESIDUE^(((q^8) - 1) / 6)
    Fq2 {
        c0: Fq(
            uint!(258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047231_U384),
        ),
        c1: Fq(uint!(0_U384)),
    },
    // Fp2::NONRESIDUE^(((q^9) - 1) / 6)
    Fq2 {
        c0: Fq(
            uint!(42198664672744474621281227892288285906241943207628877683080515507620245292955241189266486323192680957485559243678_U384),
        ),
        c1: Fq(uint!(0_U384)),
    },
    // Fp2::NONRESIDUE^(((q^10) - 1) / 6)
    Fq2 {
        c0: Fq(
            uint!(258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047232_U384),
        ),
        c1: Fq(uint!(0_U384)),
    },
    // Fp2::NONRESIDUE^(((q^11) - 1) / 6)
    Fq2 {
        c0: Fq(
            uint!(135148009893022339379906188398761468584194992116912126664040619889416147222474808140862391813728516072597320238031_U384),
        ),
        c1: Fq(uint!(0_U384)),
    },
];

impl Fq12 {
    pub fn new(c0: Fq6, c1: Fq6) -> Self {
        Fq12 { c0, c1 }
    }

    /// Multiply by quadratic nonresidue v.
    pub(crate) fn mul_fp6_by_nonresidue(fe: &Fq6) -> Fq6 {
        let new_c0 = Fq6::mul_fp2_by_nonresidue(&fe.c2);
        let new_c1 = fe.c0;
        let new_c2 = fe.c1;
        Fq6::new(new_c0, new_c1, new_c2)
    }

    pub fn conjugate(&mut self) {
        self.c1 = -self.c1;
    }

    pub fn cyclotomic_square(&self) -> Self {
        let mut result = Self::zero();
        let fp2_nr = Fq6::mul_fp2_by_nonresidue;

        let mut z0 = self.c0.c0;
        let mut z4 = self.c0.c1;
        let mut z3 = self.c0.c2;
        let mut z2 = self.c1.c0;
        let mut z1 = self.c1.c1;
        let mut z5 = self.c1.c2;

        // t0 + t1*y = (z0 + z1*y)^2 = a^2
        let mut tmp = z0 * z1;
        let t0 = (z0 + z1) * (z0 + fp2_nr(&z1)) - tmp - fp2_nr(&tmp);
        let t1 = tmp.double();

        // t2 + t3*y = (z2 + z3*y)^2 = b^2
        tmp = z2 * z3;
        let t2 = (z2 + z3) * (z2 + fp2_nr(&z3)) - tmp - fp2_nr(&tmp);
        let t3 = tmp.double();

        // t4 + t5*y = (z4 + z5*y)^2 = c^2
        tmp = z4 * z5;
        let t4 = (z4 + z5) * (z4 + fp2_nr(&z5)) - tmp - fp2_nr(&tmp);
        let t5 = tmp.double();

        // for A

        // z0 = 3 * t0 - 2 * z0
        z0 = t0 - z0;
        z0 = z0 + z0;
        result.c0.c0 = z0 + t0;

        // z1 = 3 * t1 + 2 * z1
        z1 = t1 + z1;
        z1 = z1 + z1;
        result.c1.c1 = z1 + t1;

        // for B

        // z2 = 3 * (xi * t5) + 2 * z2
        tmp = fp2_nr(&t5);
        z2 = tmp + z2;
        z2 = z2 + z2;
        result.c1.c0 = z2 + tmp;

        // z3 = 3 * t4 - 2 * z3
        z3 = t4 - z3;
        z3 = z3 + z3;
        result.c0.c2 = z3 + t4;

        // for C

        // z4 = 3 * t2 - 2 * z4
        z4 = t2 - z4;
        z4 = z4 + z4;
        result.c0.c1 = z4 + t2;

        // z5 = 3 * t3 + 2 * z5
        z5 = t3 + z5;
        z5 = z5 + z5;
        result.c1.c2 = z5 + t3;

        result
    }

    pub fn cyclotomic_exp(&self, exp: u64) -> Self {
        let mut res = Self::one();

        let mut found_one = false;

        for i in exp.view_bits::<Msb0>().iter() {
            if !found_one {
                if *i {
                    found_one = true;
                } else {
                    panic!("unhit");
                    continue;
                }
            }

            res = res.cyclotomic_square();

            if *i {
                res *= self;
            }
        }
        res
    }

    pub fn mul_by_034(&mut self, c0: &Fq2, c3: &Fq2, c4: &Fq2) {
        let a0 = self.c0.c0 * c0;
        let a1 = self.c0.c1 * c0;
        let a2 = self.c0.c2 * c0;
        let a = Fq6::new(a0, a1, a2);
        let mut b = self.c1;
        b.mul_by_01(c3, c4);

        let c0 = *c0 + c3;
        let c1 = c4;
        let mut e = self.c0 + self.c1;
        e.mul_by_01(&c0, c1);
        self.c1 = e - (a + b);
        self.c0 = a + Self::mul_fp6_by_nonresidue(&b);
    }

    pub fn mul_by_014(&mut self, c0: &Fq2, c1: &Fq2, c4: &Fq2) {
        let mut aa = self.c0;
        aa.mul_by_01(c0, c1);
        let mut bb = self.c1;
        bb.mul_by_1(c4);
        let mut o = *c1;
        o.add_assign(c4);
        self.c1.add_assign(self.c0);
        self.c1.mul_by_01(c0, &o);
        self.c1.sub_assign(&aa);
        self.c1.sub_assign(&bb);
        self.c0 = bb;
        self.c0 = Self::mul_fp6_by_nonresidue(&self.c0);
        self.c0.add_assign(aa);
    }
}

impl Field for Fq12 {
    // We don't need to compute the GLV endomorphism for Fq12.
    const PHI: Fq12 = Fq12 {
        c0: Fq6 {
            c0: Fq2 {
                c0: Fq(uint!(0_U384)),
                c1: Fq(uint!(0_U384)),
            },
            c1: Fq2 {
                c0: Fq(uint!(0_U384)),
                c1: Fq(uint!(0_U384)),
            },
            c2: Fq2 {
                c0: Fq(uint!(0_U384)),
                c1: Fq(uint!(0_U384)),
            },
        },
        c1: Fq6 {
            c0: Fq2 {
                c0: Fq(uint!(0_U384)),
                c1: Fq(uint!(0_U384)),
            },
            c1: Fq2 {
                c0: Fq(uint!(0_U384)),
                c1: Fq(uint!(0_U384)),
            },
            c2: Fq2 {
                c0: Fq(uint!(0_U384)),
                c1: Fq(uint!(0_U384)),
            },
        },
    };

    fn zero() -> Self {
        Self {
            c0: Fq6::zero(),
            c1: Fq6::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.c0.is_zero() && self.c1.is_zero()
    }

    fn one() -> Self {
        Self {
            c0: Fq6::one(),
            c1: Fq6::zero(),
        }
    }

    fn is_one(&self) -> bool {
        self.c0.is_one() && self.c1.is_zero()
    }

    fn rand() -> Self {
        Self {
            c0: Fq6::rand(),
            c1: Fq6::rand(),
        }
    }

    fn characteristic<'a>() -> Self {
        panic!("unhit");
        Self {
            c0: Fq6::characteristic(),
            c1: Fq6::zero(),
        }
    }

    fn double(&self) -> Self {
        Self {
            c0: self.c0.double(),
            c1: self.c1.double(),
        }
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
        let mut ab = self.c0;
        ab.mul_assign(&self.c1);
        let mut c0c1 = self.c0;
        c0c1.add_assign(self.c1);
        let mut c0 = self.c1;
        c0 = Self::mul_fp6_by_nonresidue(&c0);
        c0.add_assign(self.c0);
        c0.mul_assign(&c0c1);
        c0.sub_assign(&ab);
        self.c1 = ab;
        self.c1.add_assign(ab);
        ab = Self::mul_fp6_by_nonresidue(&ab);
        c0.sub_assign(&ab);
        self.c0 = c0;
    }

    fn inverse(&self) -> Option<Self> {
        if self.is_zero() {
            None
        } else {
            let mut c0s = self.c0;
            c0s.square_in_place();
            let mut c1s = self.c1;
            c1s.square_in_place();
            c1s = Self::mul_fp6_by_nonresidue(&c1s);
            c0s.sub_assign(&c1s);

            c0s.inverse().map(|t| {
                let mut tmp = Fq12::new(t, t);
                tmp.c0.mul_assign(&self.c0);
                tmp.c1.mul_assign(&self.c1);
                tmp.c1 = -tmp.c1;

                tmp
            })
        }
    }

    fn inverse_in_place(&mut self) -> Option<&mut Self> {
        if let Some(inv) = self.inverse() {
            *self = inv;
            Some(self)
        } else {
            panic!("unhit");
            None
        }
    }

    // No-op
    fn sqrt(&self) -> Option<Self> {
        panic!("unhit");
        None
    }

    fn frobenius_map(&mut self, power: usize) {
        self.c0.frobenius_map(power);
        self.c1.frobenius_map(power);
        self.c1.c0.mul_assign(FROBENIUS_COEFF_FP12_C1[power % 12]);
        self.c1.c1.mul_assign(FROBENIUS_COEFF_FP12_C1[power % 12]);
        self.c1.c2.mul_assign(FROBENIUS_COEFF_FP12_C1[power % 12]);
    }

    fn glv_endomorphism(&self) -> Self {
        panic!("unhit");
        Self::zero()
    }
}

impl Add for Fq12 {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self.add_assign(other);
        self
    }
}

impl AddAssign for Fq12 {
    fn add_assign(&mut self, other: Self) {
        self.c0.add_assign(other.c0);
        self.c1.add_assign(other.c1);
    }
}

impl Sub for Fq12 {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        self.sub_assign(other);
        self
    }
}

impl SubAssign for Fq12 {
    fn sub_assign(&mut self, other: Self) {
        self.c0.sub_assign(other.c0);
        self.c1.sub_assign(other.c1);
    }
}

impl Mul for Fq12 {
    type Output = Self;

    fn mul(mut self, other: Self) -> Self {
        self.mul_assign(other);
        self
    }
}

impl MulAssign for Fq12 {
    fn mul_assign(&mut self, other: Self) {
        let v0 = self.c0 * other.c0;
        let v1 = self.c1 * other.c1;
        self.c1 = (self.c0 + self.c1) * (other.c0 + other.c1) - v0 - v1;
        self.c0 = v0 + Self::mul_fp6_by_nonresidue(&v1);
    }
}

impl Neg for Fq12 {
    type Output = Self;

    fn neg(mut self) -> Self {
        self.c0 = -self.c0;
        self.c1 = -self.c1;
        self
    }
}

impl Div for Fq12 {
    type Output = Self;

    fn div(mut self, other: Self) -> Self {
        self.div_assign(other);
        self
    }
}

impl DivAssign for Fq12 {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn div_assign(&mut self, other: Self) {
        *self *= other.inverse().unwrap();
    }
}

impl<'a> Add<&'a Self> for Fq12 {
    type Output = Self;

    fn add(mut self, other: &Self) -> Self {
        panic!("unhit");
        self.add_assign(other);
        self
    }
}

impl<'a> AddAssign<&'a Self> for Fq12 {
    fn add_assign(&mut self, other: &Self) {
        self.c0.add_assign(other.c0);
        self.c1.add_assign(other.c1);
    }
}

impl<'a> Sub<&'a Self> for Fq12 {
    type Output = Self;

    fn sub(mut self, other: &Self) -> Self {
        panic!("unhit");
        self.sub_assign(other);
        self
    }
}

impl<'a> SubAssign<&'a Self> for Fq12 {
    fn sub_assign(&mut self, other: &Self) {
        self.c0.sub_assign(other.c0);
        self.c1.sub_assign(other.c1);
    }
}

impl<'a> Mul<&'a Self> for Fq12 {
    type Output = Self;

    fn mul(mut self, other: &Self) -> Self {
        self.mul_assign(other);
        self
    }
}

impl<'a> MulAssign<&'a Self> for Fq12 {
    fn mul_assign(&mut self, other: &Self) {
        let v0 = self.c0 * other.c0;
        let v1 = self.c1 * other.c1;
        self.c1 = (self.c0 + self.c1) * (other.c0 + other.c1) - v0 - v1;
        self.c0 = v0 + Self::mul_fp6_by_nonresidue(&v1);
    }
}

impl<'a> Div<&'a Self> for Fq12 {
    type Output = Self;

    fn div(mut self, other: &Self) -> Self {
        panic!("unhit");
        self.div_assign(other);
        self
    }
}

impl<'a> DivAssign<&'a Self> for Fq12 {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn div_assign(&mut self, other: &Self) {
        panic!("unhit");
        *self *= other.inverse().unwrap();
    }
}

impl Sum<Fq12> for Fq12 {
    /// Returns the `sum` of `self` and `other`.
    fn sum<I: Iterator<Item = Fq12>>(iter: I) -> Self {
        iter.fold(Fq12::zero(), |a, b| a + b)
    }
}
