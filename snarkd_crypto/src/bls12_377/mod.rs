#![allow(clippy::module_inception)]
// #![cfg_attr(nightly, feature(doc_cfg, external_doc))]
// #![cfg_attr(nightly, warn(missing_docs))]
#![cfg_attr(test, allow(clippy::assertions_on_result_states))]
// #![doc = include_str!("../documentation/the_aleo_curves/00_overview.md")]
#![cfg_attr(nightly, doc = include_str!("../documentation/the_aleo_curves/02_bls12-377.md"))]

use bitvec::prelude::*;
use ruint::uint;

pub mod errors;
pub use errors::*;

pub mod templates;
pub use templates::*;

pub mod field;
pub use field::*;

pub mod fr;
#[doc(inline)]
pub use fr::*;

pub mod fq;
#[doc(inline)]
pub use fq::*;

pub mod fq2;
#[doc(inline)]
pub use fq2::*;

pub mod fq6;
#[doc(inline)]
pub use fq6::*;

pub mod fq12;
#[doc(inline)]
pub use fq12::*;

pub mod g1;
#[doc(inline)]
pub use g1::*;

pub mod g2;
#[doc(inline)]
pub use g2::*;

pub mod group;
pub use group::*;

#[cfg(test)]
mod tests;

/// B1 = x^2 - 1
const B1: Fr = uint!(91893752504881257701523279626832445440_U256);

/// B2 = x^2
const B2: Fr = uint!(91893752504881257701523279626832445441_U256);

/// R128 = 2^128 - 1
const R128: Fr = uint!(340282366920938463463374607431768211455_U256);

const X: &'static [u64] = &[0x8508c00000000001];
/// `x` is positive.
const X_IS_NEGATIVE: bool = false;

/// Evaluate the line function at point p.
fn ell(f: &mut Fq12, c0: Fq2, c1: Fq2, c2: Fq2, p: &G1Affine) {
    c0.mul_by_fp(&p.y);
    c1.mul_by_fp(&p.x);
    f.mul_by_034(&c0, &c1, &c2);
}

fn exp_by_x(f: Fq12) -> Fq12 {
    f.cyclotomic_exp(X)
}

fn miller_loop<'a, I>(i: I) -> Fq12
where
    I: Iterator<Item = (&'a G1Prepared, &'a G2Prepared)>,
{
    let mut pairs = vec![];
    for (p, q) in i {
        if !p.is_zero() && !q.is_zero() {
            pairs.push((p, q.ell_coeffs.iter()));
        }
    }

    let mut f = Fq12::one();

    for i in BitVec::<Msb0, u8>::from(X.to_be_bytes()).skip(1) {
        f.square_in_place();

        for &mut (p, ref mut coeffs) in &mut pairs {
            ell(&mut f, coeffs.next().unwrap(), &p.0);
        }

        if i {
            for &mut (p, ref mut coeffs) in &mut pairs {
                ell(&mut f, coeffs.next().unwrap(), &p.0);
            }
        }
    }

    f
}

fn final_exponentiation(f: &Fq12) -> Option<Fq12> {
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
