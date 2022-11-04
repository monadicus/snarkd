use crate::bls12_377::{
    field::Field, parameters::Parameters, sw_affine::SWAffine, Affine, Projective, Scalar, B1, B2,
    HALF_R, Q1, Q2, R128,
};
use bitvec::prelude::*;
use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use ruint::{uint, Uint};

#[derive(Copy, Clone, Debug)]
pub struct SWProjective<P: Parameters> {
    pub x: P::BaseField,
    pub y: P::BaseField,
    pub z: P::BaseField,
}

impl<P: Parameters> SWProjective<P> {
    pub const fn new(x: P::BaseField, y: P::BaseField, z: P::BaseField) -> Self {
        Self { x, y, z }
    }
}

impl<P: Parameters> Default for SWProjective<P> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<P: Parameters> Display for SWProjective<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.to_affine())
    }
}

impl<P: Parameters> Hash for SWProjective<P> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_affine().hash(state);
    }
}

impl<P: Parameters> Eq for SWProjective<P> {}

impl<P: Parameters> PartialEq for SWProjective<P> {
    fn eq(&self, other: &Self) -> bool {
        if self.is_zero() {
            return other.is_zero();
        }

        if other.is_zero() {
            return false;
        }

        // The points (X, Y, Z) and (X', Y', Z')
        // are equal when (X * Z^2) = (X' * Z'^2)
        // and (Y * Z^3) = (Y' * Z'^3).
        let z1 = self.z.square();
        let z2 = other.z.square();

        !(self.x * z2 != other.x * z1 || self.y * (z2 * other.z) != other.y * (z1 * self.z))
    }
}

impl<P: Parameters> PartialEq<SWAffine<P>> for SWProjective<P> {
    fn eq(&self, other: &SWAffine<P>) -> bool {
        if self.is_zero() {
            return other.is_zero();
        }

        if other.is_zero() {
            return false;
        }

        // The points (X, Y, Z) and (X', Y', Z')
        // are equal when (X * Z^2) = (X' * Z'^2)
        // and (Y * Z^3) = (Y' * Z'^3).
        let z1 = self.z.square();
        (self.x == other.x * z1) & (self.y == other.y * z1 * self.z)
    }
}

impl<P: Parameters> Distribution<SWProjective<P>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SWProjective<P> {
        loop {
            let x = P::BaseField::rand();
            let greatest = rng.gen();

            if let Some(p) = SWAffine::from_x_coordinate(x, greatest) {
                return p.mul_bits(
                    P::COFACTOR
                        .iter()
                        .flat_map(|limb| limb.view_bits::<Lsb0>())
                        .map(|b| *b)
                        .rev()
                        .collect::<Vec<bool>>(),
                );
            }
        }
    }
}

impl<P: Parameters> Projective for SWProjective<P> {
    type Affine = SWAffine<P>;
    type Parameters = P;

    fn rand() -> Self {
        rand::thread_rng().sample(Standard)
    }

    // The point at infinity is always represented by Z = 0.
    fn zero() -> Self {
        Self::new(
            P::BaseField::zero(),
            P::BaseField::one(),
            P::BaseField::zero(),
        )
    }

    fn is_zero(&self) -> bool {
        self.z.is_zero()
    }

    fn prime_subgroup_generator() -> Self {
        SWAffine::prime_subgroup_generator().into()
    }

    fn cofactor() -> &'static [u64] {
        P::COFACTOR
    }

    fn is_normalized(&self) -> bool {
        self.is_zero() || self.z.is_one()
    }

    fn batch_normalization(v: &mut [Self]) {
        // Montgomery’s Trick and Fast Implementation of Masked AES
        // Genelle, Prouff and Quisquater
        // Section 3.2

        // First pass: compute [a, ab, abc, ...]
        let mut prod = Vec::with_capacity(v.len());
        let mut tmp = P::BaseField::one();
        for g in v
            .iter_mut()
            // Ignore normalized elements
            .filter(|g| !g.is_normalized())
        {
            tmp.mul_assign(&g.z);
            prod.push(tmp);
        }

        // Invert `tmp`.
        tmp = tmp.inverse().unwrap(); // Guaranteed to be nonzero.

        // Second pass: iterate backwards to compute inverses
        for (g, s) in v
            .iter_mut()
            // Backwards
            .rev()
            // Ignore normalized elements
            .filter(|g| !g.is_normalized())
            // Backwards, skip last element, fill in one for last term.
            .zip(
                prod.into_iter()
                    .rev()
                    .skip(1)
                    .chain(Some(P::BaseField::one())),
            )
        {
            // tmp := tmp * g.z; g.z := tmp * s = 1/z
            let newtmp = tmp * g.z;
            g.z = tmp * s;
            tmp = newtmp;
        }
        #[cfg(not(feature = "parallel"))]
        {
            // Perform affine transformations
            for g in v.iter_mut().filter(|g| !g.is_normalized()) {
                let z2 = g.z.square(); // 1/z
                g.x *= &z2; // x/z^2
                g.y *= &(z2 * g.z); // y/z^3
                g.z = P::BaseField::one(); // z = 1
            }
        }

        #[cfg(feature = "parallel")]
        {
            use rayon::prelude::*;
            // Perform affine transformations
            v.par_iter_mut()
                .filter(|g| !g.is_normalized())
                .for_each(|g| {
                    let z2 = g.z.square(); // 1/z
                    g.x *= &z2; // x/z^2
                    g.y *= &(z2 * g.z); // y/z^3
                    g.z = P::BaseField::one(); // z = 1
                });
        }
    }

    /// Adds an affine element to this element.
    fn add_mixed(&self, other: &SWAffine<P>) -> Self {
        let mut copy = *self;
        copy.add_assign_mixed(other);
        copy
    }

    #[allow(clippy::many_single_char_names)]
    fn add_assign_mixed(&mut self, other: &SWAffine<P>) {
        if other.is_zero() {
            return;
        }

        if self.is_zero() {
            self.x = other.x;
            self.y = other.y;
            self.z = P::BaseField::one();
            return;
        }

        // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#addition-madd-2007-bl
        // Works for all curves.

        // Z1Z1 = Z1^2
        let z1z1 = self.z.square();

        // U2 = X2*Z1Z1
        let u2 = other.x * z1z1;

        // S2 = Y2*Z1*Z1Z1
        let s2 = (other.y * self.z) * z1z1;

        if self.x == u2 && self.y == s2 {
            // The two points are equal, so we double.
            self.double_in_place();
        } else {
            // If we're adding -a and a together, self.z becomes zero as H becomes zero.

            // H = U2-X1
            let mut h = u2;
            h -= &self.x;

            // HH = H^2
            let hh = h.square();

            // I = 4*HH
            let mut i = hh;
            i.double_in_place();
            i.double_in_place();

            // J = H*I
            let mut j = h;
            j *= &i;

            // r = 2*(S2-Y1)
            let mut r = s2;
            r -= &self.y;
            r.double_in_place();

            // V = X1*I
            let mut v = self.x;
            v *= &i;

            // X3 = r^2 - J - 2*V
            self.x = r.square();
            self.x -= &j;
            self.x -= &v.double();

            // Y3 = r*(V-X3)-2*Y1*J
            self.y = P::BaseField::sum_of_products(
                [r, -self.y.double()].into_iter(),
                [(v - self.x), j].into_iter(),
            );

            // Z3 = (Z1+H)^2-Z1Z1-HH
            self.z += &h;
            self.z.square_in_place();
            self.z -= &z1z1;
            self.z -= &hh;
        }
    }

    fn double(&self) -> Self {
        let mut tmp = *self;
        tmp.double_in_place();
        tmp
    }

    fn double_in_place(&mut self) {
        if self.is_zero() {
            return;
        }

        // A = X1^2
        let mut a = self.x.square();

        // B = Y1^2
        let b = self.y.square();

        // C = B^2
        let mut c = b.square();

        // D = 2*((X1+B)2-A-C)
        let d = ((self.x + b).square() - a - c).double();

        // E = 3*A
        let old_a = a;
        a.double_in_place();
        let e = old_a + a;

        // F = E^2
        let f = e.square();

        // Z3 = 2*Y1*Z1
        self.z *= &self.y;
        self.z.double_in_place();

        // X3 = F-2*D
        self.x = f - d.double();

        // Y3 = E*(D-X3)-8*C
        c.double_in_place();
        c.double_in_place();
        c.double_in_place();
        self.y = (d - self.x) * e - c;
    }

    fn to_affine(&self) -> SWAffine<P> {
        (*self).into()
    }
}

impl<P: Parameters> Neg for SWProjective<P> {
    type Output = Self;

    fn neg(self) -> Self {
        if !self.is_zero() {
            Self::new(self.x, -self.y, self.z)
        } else {
            self
        }
    }
}

impl<P: Parameters> Add<Self> for SWProjective<P> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl<P: Parameters> AddAssign<Self> for SWProjective<P> {
    fn add_assign(&mut self, other: Self) {
        *self += &other;
    }
}

impl<P: Parameters> Sub<Self> for SWProjective<P> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        self -= other;
        self
    }
}

impl<P: Parameters> SubAssign<Self> for SWProjective<P> {
    fn sub_assign(&mut self, other: Self) {
        *self -= &other;
    }
}

impl<'a, P: Parameters> Add<&'a Self> for SWProjective<P> {
    type Output = Self;

    fn add(self, other: &'a Self) -> Self {
        let mut copy = self;
        copy += other;
        copy
    }
}

impl<'a, P: Parameters> AddAssign<&'a Self> for SWProjective<P> {
    #[allow(clippy::many_single_char_names)]
    #[allow(clippy::suspicious_op_assign_impl)]
    fn add_assign(&mut self, other: &'a Self) {
        if self.is_zero() {
            *self = *other;
            return;
        }

        if other.is_zero() {
            return;
        }

        // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#addition-add-2007-bl
        // Works for all curves.

        // Z1Z1 = Z1^2
        let z1z1 = self.z.square();

        // Z2Z2 = Z2^2
        let z2z2 = other.z.square();

        // U1 = X1*Z2Z2
        let u1 = self.x * z2z2;

        // U2 = X2*Z1Z1
        let u2 = other.x * z1z1;

        // S1 = Y1*Z2*Z2Z2
        let s1 = self.y * other.z * z2z2;

        // S2 = Y2*Z1*Z1Z1
        let s2 = other.y * self.z * z1z1;

        if u1 == u2 && s1 == s2 {
            // The two points are equal, so we double.
            self.double_in_place();
        } else {
            // If we're adding -a and a together, self.z becomes zero as H becomes zero.

            // H = U2-U1
            let h = u2 - u1;

            // I = (2*H)^2
            let i = (h.double()).square();

            // J = H*I
            let j = h * i;

            // r = 2*(S2-S1)
            let r = (s2 - s1).double();

            // V = U1*I
            let v = u1 * i;

            // X3 = r^2 - J - 2*V
            self.x = r.square() - j - (v.double());

            // Y3 = r*(V - X3) - 2*S1*J
            self.y = P::BaseField::sum_of_products(
                [r, -s1.double()].into_iter(),
                [(v - self.x), j].into_iter(),
            );

            // Z3 = ((Z1+Z2)^2 - Z1Z1 - Z2Z2)*H
            self.z = ((self.z + other.z).square() - z1z1 - z2z2) * h;
        }
    }
}

impl<'a, P: Parameters> Sub<&'a Self> for SWProjective<P> {
    type Output = Self;

    fn sub(self, other: &'a Self) -> Self {
        let mut copy = self;
        copy -= other;
        copy
    }
}

impl<'a, P: Parameters> SubAssign<&'a Self> for SWProjective<P> {
    fn sub_assign(&mut self, other: &'a Self) {
        *self += &(-(*other));
    }
}

impl<P: Parameters> Mul<Scalar> for SWProjective<P> {
    type Output = Self;

    /// Performs scalar multiplication of this element.
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, other: Scalar) -> Self {
        /// The scalar multiplication window size.
        const GLV_WINDOW_SIZE: usize = 4;

        /// The table size, used for w-ary NAF recoding.
        const TABLE_SIZE: i64 = 1 << (GLV_WINDOW_SIZE + 1);
        const HALF_TABLE_SIZE: i64 = 1 << (GLV_WINDOW_SIZE);
        const MASK_FOR_MOD_TABLE_SIZE: u64 = (TABLE_SIZE as u64) - 1;
        /// The GLV table length.
        const L: usize = 1 << (GLV_WINDOW_SIZE - 1);

        let decomposition = other.decompose(&Q1, &Q2, B1, B2, R128, &HALF_R);

        // Prepare tables.
        let mut t_1 = Vec::with_capacity(L);
        let double = self.double().to_affine();
        t_1.push(self);
        for i in 1..L {
            t_1.push(t_1[i - 1].add_mixed(&double));
        }
        SWProjective::<P>::batch_normalization(&mut t_1);
        let t_1 = t_1.into_iter().map(|v| v.to_affine()).collect::<Vec<_>>();

        let t_2 = t_1
            .clone()
            .into_iter()
            .map(|mut v| {
                v.x = v.x.glv_endomorphism();
                v
            })
            .collect::<Vec<_>>();

        let mod_signed = |d| {
            let d_mod_window_size = i64::try_from(d & MASK_FOR_MOD_TABLE_SIZE).unwrap();
            if d_mod_window_size >= HALF_TABLE_SIZE {
                d_mod_window_size - TABLE_SIZE
            } else {
                d_mod_window_size
            }
        };
        let to_wnaf = |mut e: Scalar| -> Vec<i32> {
            let mut naf = vec![];
            while !e.is_zero() {
                let next = if e.0 % uint!(2_U256) == uint!(1_U256) {
                    let naf_sign = mod_signed(e.0.as_limbs()[0]);
                    if naf_sign < 0 {
                        e.0 += Uint::from(-naf_sign as u64);
                    } else {
                        e.0 -= Uint::from(naf_sign as u64);
                    }
                    naf_sign.try_into().unwrap()
                } else {
                    0
                };
                naf.push(next);
                e.0 >>= 1;
            }

            naf
        };

        let wnaf = |k1: Scalar, k2: Scalar, s1: bool, s2: bool| -> (Vec<i32>, Vec<i32>) {
            let mut wnaf_1 = to_wnaf(k1);
            let mut wnaf_2 = to_wnaf(k2);

            if s1 {
                wnaf_1.iter_mut().for_each(|e| *e = -*e);
            }
            if !s2 {
                wnaf_2.iter_mut().for_each(|e| *e = -*e);
            }

            (wnaf_1, wnaf_2)
        };

        let naf_add = |table: &[SWAffine<P>], naf: i32, acc: &mut SWProjective<P>| {
            if naf != 0 {
                let mut p_1 = table[(naf.abs() >> 1) as usize];
                if naf < 0 {
                    p_1 = p_1.neg();
                }
                acc.add_assign_mixed(&p_1);
            }
        };

        // Recode scalars.
        let (naf_1, naf_2) = wnaf(
            decomposition.0,
            decomposition.1,
            decomposition.2,
            decomposition.3,
        );
        let max_len = naf_1.len().max(naf_2.len());
        let mut acc = SWProjective::<P>::zero();
        for i in (0..max_len).rev() {
            if i < naf_1.len() {
                naf_add(&t_1, naf_1[i], &mut acc)
            }

            if i < naf_2.len() {
                naf_add(&t_2, naf_2[i], &mut acc)
            }

            if i != 0 {
                acc.double_in_place();
            }
        }

        acc
    }
}

impl<P: Parameters> MulAssign<Scalar> for SWProjective<P> {
    /// Performs scalar multiplication of this element.
    fn mul_assign(&mut self, other: Scalar) {
        *self = *self * other
    }
}

/// The affine point X, Y is represented in the Jacobian coordinates with Z = 1.
impl<P: Parameters> From<SWAffine<P>> for SWProjective<P> {
    fn from(p: SWAffine<P>) -> SWProjective<P> {
        if p.is_zero() {
            Self::zero()
        } else {
            Self::new(p.x, p.y, P::BaseField::one())
        }
    }
}
