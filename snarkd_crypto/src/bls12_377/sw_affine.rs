use crate::bls12_377::{
    field::Field, parameters::Parameters, sw_projective::SWProjective, Affine, Projective, Scalar,
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
#[cfg_attr(
    any(test, feature = "fuzz"),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct SWAffine<P: Parameters> {
    pub x: P::BaseField,
    pub y: P::BaseField,
    pub infinity: bool,
}

impl<P: Parameters> SWAffine<P> {
    pub const fn new(x: P::BaseField, y: P::BaseField, infinity: bool) -> Self {
        Self { x, y, infinity }
    }
}

impl<P: Parameters> Default for SWAffine<P> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<P: Parameters> Display for SWAffine<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.infinity {
            write!(f, "SWAffine(Infinity)")
        } else {
            write!(f, "SWAffine(x={}, y={})", self.x, self.y)
        }
    }
}

impl<P: Parameters> Affine for SWAffine<P> {
    type Projective = SWProjective<P>;
    type Parameters = P;

    const ZERO: Self = Self::new(P::BaseField::ZERO, P::BaseField::ONE, true);

    fn is_zero(&self) -> bool {
        self.infinity
    }

    /// Initializes a new affine group element from the given coordinates.
    fn from_coordinates(x: P::BaseField, y: P::BaseField, infinity: bool) -> Self {
        let point = Self { x, y, infinity };
        assert!(point.is_on_curve());
        point
    }

    fn rand() -> Self {
        rand::thread_rng().sample(Standard)
    }

    fn cofactor() -> &'static [u64] {
        P::COFACTOR
    }

    fn prime_subgroup_generator() -> Self {
        Self::new(
            P::AFFINE_GENERATOR_COEFFS.0,
            P::AFFINE_GENERATOR_COEFFS.1,
            false,
        )
    }

    /// Attempts to construct an affine point given an x-coordinate. The
    /// point is not guaranteed to be in the prime order subgroup.
    ///
    /// If and only if `greatest` is set will the lexicographically
    /// largest y-coordinate be selected.
    fn from_x_coordinate(x: P::BaseField, greatest: bool) -> Option<Self> {
        // Compute x^3 + ax + b
        let x3b = (x.square() * x) + P::B;

        x3b.sqrt().map(|y| {
            let negy = -y;

            let y = if (y < negy) ^ greatest { y } else { negy };
            Self::new(x, y, false)
        })
    }

    fn mul_bits(&self, bits: Vec<bool>) -> SWProjective<P> {
        let mut output = SWProjective::ZERO;
        for i in bits.iter().skip_while(|b| !**b) {
            output.double_in_place();
            if *i {
                output.add_assign_mixed(self);
            }
        }
        output
    }

    fn mul_by_cofactor_inv(&self) -> Self {
        (*self * P::COFACTOR_INV).into()
    }

    fn to_projective(&self) -> SWProjective<P> {
        (*self).into()
    }

    /// Checks that the current point is on the elliptic curve.
    fn is_on_curve(&self) -> bool {
        if self.is_zero() {
            true
        } else {
            // Check that the point is on the curve
            let y2 = self.y.square();
            let x3b = (self.x.square() * self.x) + P::B;
            y2 == x3b
        }
    }

    /// Performs the first half of batch addition in-place:
    ///     `lambda` := `(y2 - y1) / (x2 - x1)`,
    /// for two given affine points.
    fn batch_add_loop_1(
        a: &mut Self,
        b: &mut Self,
        half: &P::BaseField,
        inversion_tmp: &mut P::BaseField,
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
                a.y = x_sq.double() + x_sq; // numerator = 3x^2
                b.y -= &(a.y * half); // y - (3x^2)/2
                a.y *= *inversion_tmp; // (3x^2) * tmp
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
    fn batch_add_loop_2(a: &mut Self, b: Self, inversion_tmp: &mut P::BaseField) {
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

impl<P: Parameters> Neg for SWAffine<P> {
    type Output = Self;

    fn neg(self) -> Self {
        if !self.is_zero() {
            Self::new(self.x, -self.y, false)
        } else {
            self
        }
    }
}

impl<P: Parameters> Mul<Scalar> for SWAffine<P> {
    type Output = SWProjective<P>;

    fn mul(self, other: Scalar) -> Self::Output {
        self.to_projective().mul(other)
    }
}

impl<P: Parameters> Distribution<SWAffine<P>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SWAffine<P> {
        loop {
            let x = P::BaseField::rand();
            let greatest = rng.gen();

            if let Some(p) = SWAffine::from_x_coordinate(x, greatest) {
                return p
                    .mul_bits(
                        P::COFACTOR
                            .iter()
                            .flat_map(|limb| limb.view_bits::<Lsb0>())
                            .map(|b| *b)
                            .rev()
                            .collect::<Vec<bool>>(),
                    )
                    .into();
            }
        }
    }
}

#[cfg(feature = "fuzz")]
impl<'a, P: Parameters> arbitrary::Arbitrary<'a> for SWAffine<P> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        loop {
            let x = P::BaseField::arbitrary(u)?;
            let greatest = bool::arbitrary(u)?;

            if let Some(p) = SWAffine::from_x_coordinate(x, greatest) {
                return Ok(p
                    .mul_bits(
                        P::COFACTOR
                            .iter()
                            .flat_map(|limb| limb.view_bits::<Lsb0>())
                            .map(|b| *b)
                            .rev()
                            .collect::<Vec<bool>>(),
                    )
                    .into());
            }
        }
    }
}

// The projective point X, Y, Z is represented in the affine coordinates as X/Z^2, Y/Z^3.
impl<P: Parameters> From<SWProjective<P>> for SWAffine<P> {
    fn from(p: SWProjective<P>) -> SWAffine<P> {
        if p.is_zero() {
            SWAffine::ZERO
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
