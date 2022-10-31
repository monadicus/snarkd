use crate::bls12_377::{group::Group, templates::short_weierstrass_jacobian::Affine, Fr};

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::io::Write;

#[derive(Copy, Clone, Debug)]
pub struct Projective<G: Group> {
    pub x: G::BaseField,
    pub y: G::BaseField,
    pub z: G::BaseField,
}

impl<G: Group> Projective<G> {
    pub const fn new(x: G::BaseField, y: G::BaseField, z: G::BaseField) -> Self {
        Self { x, y, z }
    }

    // The point at infinity is always represented by Z = 0.
    #[inline]
    pub const fn zero() -> Self {
        Self::new(
            G::BaseField::zero(),
            G::BaseField::one(),
            G::BaseField::zero(),
        )
    }
}

impl<G: Group> Default for Projective<G> {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl<G: Group> Display for Projective<G> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.to_affine())
    }
}

impl<G: Group> Hash for Projective<G> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_affine().hash(state);
    }
}

impl<G: Group> Eq for Projective<G> {}

impl<G: Group> PartialEq for Projective<G> {
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

impl<G: Group> PartialEq<Affine<G>> for Projective<G> {
    fn eq(&self, other: &Affine<G>) -> bool {
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

impl<G: Group> Distribution<Projective<G>> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Projective<G> {
        loop {
            let x = G::BaseField::rand(rng);
            let greatest = rng.gen();

            if let Some(p) = Affine::from_x_coordinate(x, greatest) {
                return p.mul_by_cofactor_to_projective();
            }
        }
    }
}

impl<G: Group> Projective<G> {
    #[inline]
    fn prime_subgroup_generator() -> Self {
        Affine::prime_subgroup_generator().into()
    }

    #[inline]
    fn cofactor() -> &'static [u64] {
        G::COFACTOR
    }

    #[inline]
    fn is_normalized(&self) -> bool {
        self.is_zero() || self.z.is_one()
    }

    /// TODO (howardwu): This method can likely be sped up.
    #[inline]
    fn batch_normalization(v: &mut [Self]) {
        // Montgomery’s Trick and Fast Implementation of Masked AES
        // Genelle, Prouff and Quisquater
        // Section 3.2

        // First pass: compute [a, ab, abc, ...]
        let mut prod = Vec::with_capacity(v.len());
        let mut tmp = G::BaseField::one();
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
                    .chain(Some(G::BaseField::one())),
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
                g.z = G::BaseField::one(); // z = 1
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
                    g.z = G::BaseField::one(); // z = 1
                });
        }
    }

    #[allow(clippy::many_single_char_names)]
    fn add_assign_mixed(&mut self, other: &Affine<G>) {
        if other.is_zero() {
            return;
        }

        if self.is_zero() {
            self.x = other.x;
            self.y = other.y;
            self.z = G::BaseField::one();
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
            self.y = G::BaseField::sum_of_products(
                [r, -self.y.double()].iter(),
                [(v - self.x), j].iter(),
            );

            // Z3 = (Z1+H)^2-Z1Z1-HH
            self.z += &h;
            self.z.square_in_place();
            self.z -= &z1z1;
            self.z -= &hh;
        }
    }

    #[inline]
    #[must_use]
    fn double(&self) -> Self {
        let mut tmp = *self;
        tmp.double_in_place();
        tmp
    }

    #[inline]
    fn double_in_place(&mut self) {
        if self.is_zero() {
            return;
        }

        if G::WEIERSTRASS_A.is_zero() {
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
        } else {
            // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#doubling-dbl-2009-l
            // XX = X1^2
            let xx = self.x.square();

            // YY = Y1^2
            let yy = self.y.square();

            // YYYY = YY^2
            let mut yyyy = yy.square();

            // ZZ = Z1^2
            let zz = self.z.square();

            // S = 2*((X1+YY)^2-XX-YYYY)
            let s = ((self.x + yy).square() - xx - yyyy).double();

            // M = 3*XX+a*ZZ^2
            let m = xx.double() + xx + G::mul_by_a(&zz.square());

            // T = M^2-2*S
            let t = m.square() - s.double();

            // X3 = T
            self.x = t;
            // Y3 = M*(S-T)-8*YYYY
            let old_y = self.y;
            yyyy.double_in_place();
            yyyy.double_in_place();
            yyyy.double_in_place();
            self.y = m * (s - t) - yyyy;
            // Z3 = (Y1+Z1)^2-YY-ZZ
            self.z = (old_y + self.z).square() - yy - zz;
        }
    }

    #[inline]
    fn to_affine(&self) -> Affine<G> {
        (*self).into()
    }
}

impl<G: Group> Neg for Projective<G> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        if !self.is_zero() {
            Self::new(self.x, -self.y, self.z)
        } else {
            self
        }
    }
}

impl<'a, G: Group> Add<&'a Self> for Projective<G> {
    type Output = Self;

    #[inline]
    fn add(self, other: &'a Self) -> Self {
        let mut copy = self;
        copy += other;
        copy
    }
}

impl<'a, G: Group> AddAssign<&'a Self> for Projective<G> {
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
            self.y =
                G::BaseField::sum_of_products([r, -s1.double()].iter(), [(v - self.x), j].iter());

            // Z3 = ((Z1+Z2)^2 - Z1Z1 - Z2Z2)*H
            self.z = ((self.z + other.z).square() - z1z1 - z2z2) * h;
        }
    }
}

impl<'a, G: Group> Sub<&'a Self> for Projective<G> {
    type Output = Self;

    #[inline]
    fn sub(self, other: &'a Self) -> Self {
        let mut copy = self;
        copy -= other;
        copy
    }
}

impl<'a, G: Group> SubAssign<&'a Self> for Projective<G> {
    fn sub_assign(&mut self, other: &'a Self) {
        *self += &(-(*other));
    }
}

impl<G: Group> Mul<G::BaseField> for Projective<G> {
    type Output = Self;

    /// Performs scalar multiplication of this element.
    #[allow(clippy::suspicious_arithmetic_impl)]
    #[inline]
    fn mul(self, other: G::BaseField) -> Self {
        /// The scalar multiplication window size.
        const GLV_WINDOW_SIZE: usize = 4;

        /// The table size, used for w-ary NAF recoding.
        const TABLE_SIZE: i64 = 1 << (GLV_WINDOW_SIZE + 1);
        const HALF_TABLE_SIZE: i64 = 1 << (GLV_WINDOW_SIZE);
        const MASK_FOR_MOD_TABLE_SIZE: u64 = (TABLE_SIZE as u64) - 1;
        /// The GLV table length.
        const L: usize = 1 << (GLV_WINDOW_SIZE - 1);

        let decomposition = other.decompose(
            &Self::Q1,
            &Self::Q2,
            Self::B1,
            Self::B2,
            Self::R128,
            &Self::HALF_R,
        );

        // Prepare tables.
        let mut t_1 = Vec::with_capacity(L);
        let double = Affine::<Self>::from(self.double());
        t_1.push(self);
        for i in 1..L {
            t_1.push(t_1[i - 1].add_mixed(&double));
        }
        let t_1 = Projective::<Self>::batch_normalization_into_affine(t_1);

        let t_2 = t_1
            .iter()
            .copied()
            .map(|v| v.x.glv_endomorphism())
            .collect::<Vec<_>>();

        let mod_signed = |d| {
            let d_mod_window_size = i64::try_from(d & MASK_FOR_MOD_TABLE_SIZE).unwrap();
            if d_mod_window_size >= HALF_TABLE_SIZE {
                d_mod_window_size - TABLE_SIZE
            } else {
                d_mod_window_size
            }
        };
        let to_wnaf = |mut e: Fr| -> Vec<i32> {
            let mut naf = vec![];
            while !e.is_zero() {
                let next = if e.is_odd() {
                    let naf_sign = mod_signed(e.to_le_limbs()[0]);
                    if naf_sign < 0 {
                        e += -naf_sign as u64;
                    } else {
                        e -= naf_sign as u64;
                    }
                    naf_sign.try_into().unwrap()
                } else {
                    0
                };
                naf.push(next);
                e.div2();
            }

            naf
        };

        let wnaf = |k1: Self::ScalarField,
                    k2: Self::ScalarField,
                    s1: bool,
                    s2: bool|
         -> (Vec<i32>, Vec<i32>) {
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

        let naf_add = |table: &[Affine<Self>], naf: i32, acc: &mut Projective<Self>| {
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
        let mut acc = Projective::<Self>::zero();
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

impl<G: Group> MulAssign<G::BaseField> for Projective<G> {
    /// Performs scalar multiplication of this element.
    fn mul_assign(&mut self, other: G::BaseField) {
        *self = *self * other
    }
}

/// The affine point X, Y is represented in the Jacobian coordinates with Z = 1.
impl<G: Group> From<Affine<G>> for Projective<G> {
    #[inline]
    fn from(p: Affine<G>) -> Projective<G> {
        if p.is_zero() {
            Self::zero()
        } else {
            Self::new(p.x, p.y, G::BaseField::one())
        }
    }
}
