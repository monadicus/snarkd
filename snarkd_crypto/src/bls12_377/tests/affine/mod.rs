mod g1;
pub use g1::*;

mod g2;
pub use g2::*;

use test_runner::{Namespace, Test, TestResult};

use crate::bls12_377::{Affine, Parameters, SWAffine, Scalar};

pub type AffineTuple<T> = (T, T, bool);

impl<T, B, P> From<AffineTuple<T>> for SWAffine<P>
where
    T: Copy + Into<B>,
    P: Parameters<BaseField = B>,
{
    fn from(v: AffineTuple<T>) -> Self {
        SWAffine {
            x: v.0.into(),
            y: v.1.into(),
            infinity: v.2,
        }
    }
}

impl<T, B, P> From<SWAffine<P>> for AffineTuple<T>
where
    B: Into<T>,
    P: Parameters<BaseField = B>,
{
    fn from(v: SWAffine<P>) -> Self {
        (v.x.into(), v.y.into(), v.infinity)
    }
}

pub fn neg<A: Affine>(a: A) -> TestResult {
    let mut outputs = Vec::new();

    let mut a_proj = a.to_projective();
    let neg_a = -a;
    outputs.push(neg_a.to_string());
    let neg_proj = neg_a.to_projective();
    outputs.push(neg_proj.to_string());

    a_proj += neg_proj;
    assert_eq!(a_proj, A::ZERO.to_projective());

    Ok(outputs.into())
}

pub fn mul<A: Affine>(a: A, s: Scalar) -> TestResult {
    let mut outputs = Vec::new();

    let a_proj = a.to_projective();

    let tmp1 = a_proj * s;
    outputs.push(tmp1.to_string());
    let tmp2 = a * s;
    outputs.push(tmp2.to_string());

    assert_eq!(tmp1, tmp2);

    Ok(outputs.into())
}
