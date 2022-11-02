use crate::bls12_377::{
    field::Field,
    group::Group,
    templates::short_weierstrass_jacobian::{Affine, Projective},
    Fq, Fq2, Fr, G2Parameters, X,
};
use bitvec::prelude::*;

pub type G2Affine = Affine<G2Parameters>;
pub type G2Projective = Projective<G2Parameters>;

impl G2Affine {
    pub fn is_in_correct_subgroup_assuming_on_curve(&self) -> bool {
        self.mul_bits(
            Fr::characteristic()
                .0
                .to_be_bytes::<32>()
                .view_bits::<Msb0>(),
        )
        .is_zero()
    }
}

type CoeffTriplet = (Fq2, Fq2, Fq2);

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct G2Prepared {
    // Stores the coefficients of the line evaluations as calculated in
    // https://eprint.iacr.org/2013/722.pdf
    pub ell_coeffs: Vec<CoeffTriplet>,
    pub infinity: bool,
}

#[derive(Copy, Clone, Debug)]
struct G2HomProjective {
    x: Fq2,
    y: Fq2,
    z: Fq2,
}

impl Default for G2Prepared {
    fn default() -> Self {
        Self::from_affine(G2Affine::prime_subgroup_generator())
    }
}

impl G2Prepared {
    pub fn is_zero(&self) -> bool {
        self.infinity
    }

    pub fn from_affine(q: G2Affine) -> Self {
        if q.is_zero() {
            return Self {
                ell_coeffs: vec![],
                infinity: true,
            };
        }

        let mut r = G2HomProjective {
            x: q.x,
            y: q.y,
            z: Fq2::one(),
        };

        let bit_iterator = X.view_bits::<Msb0>();
        let mut ell_coeffs = Vec::with_capacity(bit_iterator.len());

        // `one_half` = 1/2 in the field.
        let one_half = Fq::half();

        for i in bit_iterator.iter().skip(1) {
            ell_coeffs.push(doubling_step(&mut r, &one_half));

            if *i {
                ell_coeffs.push(addition_step(&mut r, &q));
            }
        }

        Self {
            ell_coeffs,
            infinity: false,
        }
    }
}

#[allow(clippy::many_single_char_names)]
fn doubling_step(r: &mut G2HomProjective, two_inv: &Fq) -> CoeffTriplet {
    // Formula for line function when working with
    // homogeneous projective coordinates.

    let mut a = r.x * r.y;
    a.mul_by_fp(two_inv);
    let b = r.y.square();
    let c = r.z.square();
    let e = G2Parameters::B * (c.double() + c);
    let f = e.double() + e;
    let mut g = b + f;
    g.mul_by_fp(two_inv);
    let h = (r.y + r.z).square() - (b + c);
    let i = e - b;
    let j = r.x.square();
    let e_square = e.square();

    r.x = a * (b - f);
    r.y = g.square() - (e_square.double() + e_square);
    r.z = b * h;
    (-h, j.double() + j, i)
}

#[allow(clippy::many_single_char_names)]
fn addition_step(r: &mut G2HomProjective, q: &G2Affine) -> CoeffTriplet {
    // Formula for line function when working with
    // homogeneous projective coordinates.
    let theta = r.y - (q.y * r.z);
    let lambda = r.x - (q.x * r.z);
    let c = theta.square();
    let d = lambda.square();
    let e = lambda * d;
    let f = r.z * c;
    let g = r.x * d;
    let h = e + f - g.double();
    r.x = lambda * h;
    r.y = theta * (g - h) - (e * r.y);
    r.z *= &e;
    let j = theta * q.x - (lambda * q.y);

    (lambda, -theta, j)
}
