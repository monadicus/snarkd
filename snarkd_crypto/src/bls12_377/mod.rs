use bitvec::prelude::*;
use ruint::uint;

pub mod affine;
pub use affine::*;

pub mod field;
pub use field::*;

pub mod scalar;
pub use scalar::*;

pub mod fp;
pub use fp::*;

pub mod fp2;
pub use fp2::*;

pub mod fp6;
pub use fp6::*;

pub mod fp12;
pub use fp12::*;

pub mod g1;
pub use g1::*;

pub mod g2;
pub use g2::*;

pub mod parameters;
pub use parameters::*;

pub mod projective;
pub use projective::*;

pub mod sw_affine;
pub use sw_affine::*;

pub mod sw_projective;
pub use sw_projective::*;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LegendreSymbol {
    Zero,
    QuadraticNonResidue,
    QuadraticResidue,
}

/// Q1 = x^2 * R / q
const Q1: [u64; 4] = [9183663392111466540, 12968021215939883360, 3, 0];

/// Q2 = R / q = 13
const Q2: [u64; 4] = [13, 0, 0, 0];

/// B1 = x^2 - 1
const B1: Scalar = Scalar(uint!(91893752504881257701523279626832445440_U256));

/// B2 = x^2
const B2: Scalar = Scalar(uint!(91893752504881257701523279626832445441_U256));

/// R128 = 2^128 - 1
const R128: Scalar = Scalar(uint!(340282366920938463463374607431768211455_U256));

/// HALF_R = 2^256 / 2
const HALF_R: [u64; 8] = [0, 0, 0, 0x8000000000000000, 0, 0, 0, 0];

const X: u64 = 0x8508c00000000001;

/// Performs multiple pairing operations
pub fn pairing<G1: Into<G1Affine>, G2: Into<G2Affine>>(p: G1, q: G2) -> Fp12 {
    final_exponentiation(&miller_loop(core::iter::once((
        &G1Prepared::from_affine(p.into()),
        &G2Prepared::from_affine(q.into()),
    ))))
    .unwrap()
}

/// Evaluate the line function at point p.
fn ell(f: &mut Fp12, c0: Fp2, c1: Fp2, c2: Fp2, p: &G1Affine) {
    let mut c0 = c0;
    let mut c1 = c1;
    c0.mul_by_fp(&p.y);
    c1.mul_by_fp(&p.x);
    f.mul_by_034(&c0, &c1, &c2);
}

fn exp_by_x(f: Fp12) -> Fp12 {
    f.cyclotomic_exp(X)
}

fn miller_loop<'a, I>(i: I) -> Fp12
where
    I: Iterator<Item = (&'a G1Prepared, &'a G2Prepared)>,
{
    let mut pairs = i
        .filter_map(|(p, q)| (!p.is_zero() && !q.is_zero()).then(|| (p, q.ell_coeffs.iter())))
        .collect::<Vec<_>>();

    let mut f = Fp12::ONE;

    for i in X.view_bits::<Msb0>().iter().skip(1) {
        f.square_in_place();

        for &mut (p, ref mut coeffs) in &mut pairs {
            let coeffs = coeffs.next().unwrap();
            ell(&mut f, coeffs.0, coeffs.1, coeffs.2, &p.0);
        }

        if *i {
            for &mut (p, ref mut coeffs) in &mut pairs {
                let coeffs = coeffs.next().unwrap();
                ell(&mut f, coeffs.0, coeffs.1, coeffs.2, &p.0);
            }
        }
    }

    f
}

fn final_exponentiation(f: &Fp12) -> Option<Fp12> {
    // Computing the final exponentiation following
    // https://eprint.iacr.org/2016/130.pdf.
    // We don't use their "faster" formula because it is difficult to make
    // it work for curves with odd `P::X`.
    // Hence we implement the algorithm from Table 1 below.

    // f1 = r.conjugate() = f^(p^6)
    let mut f1 = *f;
    f1.conjugate();

    match f.inverse() {
        Some(mut f2) => {
            // f2 = f^(-1);
            // r = f^(p^6 - 1)
            let mut r = f1 * f2;

            // f2 = f^(p^6 - 1)
            f2 = r;
            // r = f^((p^6 - 1)(p^2))
            r.frobenius_map(2);

            // r = f^((p^6 - 1)(p^2) + (p^6 - 1))
            // r = f^((p^6 - 1)(p^2 + 1))
            r *= &f2;

            // Hard part of the final exponentation is below:
            // From https://eprint.iacr.org/2016/130.pdf, Table 1
            let mut y0 = r.cyclotomic_square();
            y0.conjugate();

            let mut y5 = exp_by_x(r);

            let mut y1 = y5.cyclotomic_square();
            let mut y3 = y0 * y5;
            y0 = exp_by_x(y3);
            let y2 = exp_by_x(y0);
            let mut y4 = exp_by_x(y2);
            y4 *= &y1;
            y1 = exp_by_x(y4);
            y3.conjugate();
            y1 *= &y3;
            y1 *= &r;
            y3 = r;
            y3.conjugate();
            y0 *= &r;
            y0.frobenius_map(3);
            y4 *= &y3;
            y4.frobenius_map(1);
            y5 *= &y2;
            y5.frobenius_map(2);
            y5 *= &y0;
            y5 *= &y4;
            y5 *= &y1;
            Some(y5)
        }
        None => None,
    }
}

/// Calculate a + (b * c) + carry, returning the least significant digit
/// and setting carry to the most significant digit.
pub fn mac_with_carry(a: u64, b: u64, c: u64, carry: &mut u64) -> u64 {
    let tmp = (u128::from(a)) + u128::from(b) * u128::from(c) + u128::from(*carry);

    *carry = (tmp >> 64) as u64;

    tmp as u64
}

/// Calculate a + b + carry, returning the sum and modifying the
/// carry value.
pub fn adc(a: &mut u64, b: u64, carry: u64) -> u64 {
    let tmp = u128::from(*a) + u128::from(b) + u128::from(carry);
    *a = tmp as u64;
    (tmp >> 64) as u64
}
