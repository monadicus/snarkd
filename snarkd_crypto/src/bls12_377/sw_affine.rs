use crate::bls12_377::{
    field::Field, group::Group, sw_projective::SWProjective, Affine, Fr, Projective,
};
use bitvec::prelude::*;
use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Mul, Neg},
};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SWAffine<G: Group> {
    pub x: G::BaseField,
    pub y: G::BaseField,
    pub infinity: bool,
}

impl<G: Group> SWAffine<G> {
    #[inline]
    pub const fn new(x: G::BaseField, y: G::BaseField, infinity: bool) -> Self {
        Self { x, y, infinity }
    }
}

impl<G: Group> Default for SWAffine<G> {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl<G: Group> Display for SWAffine<G> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.infinity {
            write!(f, "SWAffine(Infinity)")
        } else {
            write!(f, "SWAffine(x={}, y={})", self.x, self.y)
        }
    }
}

impl<G: Group> Affine for SWAffine<G> {
    type Projective = SWProjective<G>;
    type Parameters = G;

    #[inline]
    fn zero() -> Self {
        Self::new(G::BaseField::zero(), G::BaseField::one(), true)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.infinity
    }

    /// Initializes a new affine group element from the given coordinates.
    fn from_coordinates(x: G::BaseField, y: G::BaseField, infinity: bool) -> Self {
        let point = Self { x, y, infinity };
        assert!(point.is_on_curve());
        point
    }

    fn rand() -> Self {
        rand::thread_rng().sample(Standard)
    }

    #[inline]
    fn cofactor() -> &'static [u64] {
        G::COFACTOR
    }

    #[inline]
    fn prime_subgroup_generator() -> Self {
        Self::new(
            G::AFFINE_GENERATOR_COEFFS.0,
            G::AFFINE_GENERATOR_COEFFS.1,
            false,
        )
    }

    /// Attempts to construct an affine point given an x-coordinate. The
    /// point is not guaranteed to be in the prime order subgroup.
    ///
    /// If and only if `greatest` is set will the lexicographically
    /// largest y-coordinate be selected.
    fn from_x_coordinate(x: G::BaseField, greatest: bool) -> Option<Self> {
        // Compute x^3 + ax + b
        let x3b = (x.square() * x) + G::B;

        x3b.sqrt().map(|y| {
            let negy = -y;

            let y = if (y < negy) ^ greatest { y } else { negy };
            Self::new(x, y, false)
        })
    }

    /// Attempts to construct an affine point given a y-coordinate. The
    /// point is not guaranteed to be in the prime order subgroup.
    ///
    /// If and only if `greatest` is set will the lexicographically
    /// largest y-coordinate be selected.
    fn from_y_coordinate(_y: G::BaseField, _greatest: bool) -> Option<Self> {
        unimplemented!()
    }

    fn mul_bits(&self, bits: &BitSlice<u8, Msb0>) -> SWProjective<G> {
        let mut output = SWProjective::zero();
        for i in bits.iter().skip_while(|b| !(*b.clone())) {
            output.double_in_place();
            if *i {
                output.add_assign_mixed(self);
            }
        }
        output
    }

    fn mul_by_cofactor_to_projective(&self) -> SWProjective<G> {
        self.mul_bits(
            G::COFACTOR
                .iter()
                .map(|limb| limb.to_be_bytes())
                .flatten()
                .collect::<Vec<u8>>()
                .view_bits::<Msb0>(),
        )
    }

    fn mul_by_cofactor(&self) -> Self {
        self.mul_by_cofactor_to_projective().into()
    }

    fn mul_by_cofactor_inv(&self) -> Self {
        (*self * G::COFACTOR_INV).into()
    }

    #[inline]
    fn to_projective(&self) -> SWProjective<G> {
        (*self).into()
    }

    /// Checks that the current point is on the elliptic curve.
    fn is_on_curve(&self) -> bool {
        if self.is_zero() {
            true
        } else {
            // Check that the point is on the curve
            let y2 = self.y.square();
            let x3b = (self.x.square() * self.x) + G::B;
            y2 == x3b
        }
    }

    /// Performs the first half of batch addition in-place:
    ///     `lambda` := `(y2 - y1) / (x2 - x1)`,
    /// for two given affine points.
    fn batch_add_loop_1(
        a: &mut Self,
        b: &mut Self,
        half: &G::BaseField,
        inversion_tmp: &mut G::BaseField,
    ) {
        if a.is_zero() || b.is_zero() {
        } else if a.x == b.x {
            // Double
            // In our model, we consider self additions rare.
            // So we consider it inconsequential to make them more expensive
            // This costs 1 modular mul more than a standard squaring,
            // and one amortised inversion
            if a.y == b.y {
                // Compute one half (1/2) and cache it.

                let x_sq = b.x.square();
                b.x -= &b.y; // x - y
                a.x = b.y.double(); // denominator = 2y
                a.y = x_sq.double() + x_sq + G::A; // numerator = 3x^2 + a
                b.y -= &(a.y * half); // y - (3x^2 + a)/2
                a.y *= *inversion_tmp; // (3x^2 + a) * tmp
                *inversion_tmp *= &a.x; // update tmp
            } else {
                // No inversions take place if either operand is zero
                a.infinity = true;
                b.infinity = true;
            }
        } else {
            // We can recover x1 + x2 from this. Note this is never 0.
            a.x -= &b.x; // denominator = x1 - x2
            a.y -= &b.y; // numerator = y1 - y2
            a.y *= *inversion_tmp; // (y1 - y2)*tmp
            *inversion_tmp *= &a.x // update tmp
        }
    }

    /// Performs the second half of batch addition in-place:
    ///     `x3` := `lambda^2 - x1 - x2`
    ///     `y3` := `lambda * (x1 - x3) - y1`.
    fn batch_add_loop_2(a: &mut Self, b: Self, inversion_tmp: &mut G::BaseField) {
        if a.is_zero() {
            *a = b;
        } else if !b.is_zero() {
            let lambda = a.y * *inversion_tmp;
            *inversion_tmp *= &a.x; // Remove the top layer of the denominator

            // x3 = l^2 - x1 - x2 or for squaring: 2y + l^2 + 2x - 2y = l^2 - 2x
            a.x += &b.x.double();
            a.x = lambda.square() - a.x;
            // y3 = l*(x2 - x3) - y2 or
            // for squaring: (3x^2 + a)/2y(x - y - x3) - (y - (3x^2 + a)/2) = l*(x - x3) - y
            a.y = lambda * (b.x - a.x) - b.y;
        }
    }
}

impl<G: Group> Neg for SWAffine<G> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        if !self.is_zero() {
            Self::new(self.x, -self.y, false)
        } else {
            self
        }
    }
}

impl<G: Group> Mul<Fr> for SWAffine<G> {
    type Output = SWProjective<G>;

    fn mul(self, other: Fr) -> Self::Output {
        self.to_projective().mul(other)
    }
}

impl<G: Group> Distribution<SWAffine<G>> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SWAffine<G> {
        loop {
            let x = G::BaseField::rand();
            let greatest = rng.gen();

            if let Some(p) = SWAffine::from_x_coordinate(x, greatest) {
                return p.mul_by_cofactor();
            }
        }
    }
}

// The projective point X, Y, Z is represented in the affine coordinates as X/Z^2, Y/Z^3.
impl<G: Group> From<SWProjective<G>> for SWAffine<G> {
    #[inline]
    fn from(p: SWProjective<G>) -> SWAffine<G> {
        if p.is_zero() {
            SWAffine::zero()
        } else if p.z.is_one() {
            // If Z is one, the point is already normalized.
            SWAffine::new(p.x, p.y, false)
        } else {
            // Z is nonzero, so it must have an inverse in a field.
            let zinv = p.z.inverse().unwrap();
            let zinv_squared = zinv.square();

            // X/Z^2
            let x = p.x * zinv_squared;

            // Y/Z^3
            let y = p.y * (zinv_squared * zinv);

            SWAffine::new(x, y, false)
        }
    }
}
