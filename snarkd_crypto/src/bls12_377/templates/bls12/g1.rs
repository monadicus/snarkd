use crate::bls12_377::{
    templates::short_weierstrass_jacobian::{Affine, Projective},
    Fr, G1Parameters, X,
};
use bitvec::prelude::*;

pub type G1Affine = Affine<G1Parameters>;
pub type G1Projective = Projective<G1Parameters>;

impl G1Affine {
    fn is_in_correct_subgroup_assuming_on_curve(&self) -> bool {
        let phi = |mut p: &Self| {
            debug_assert!(Self::BaseField::PHI.pow([3]).is_one());
            p.x *= Self::PHI;
            p
        };
        let x_square = Fr::from(X[0]).square();
        let bits = BitVec::<Msb0, u8>::from(x_square.to_be_bytes());
        let bits = bits[bits.leading_zeros()..];
        (phi(*self).mul_bits(bits).add_mixed(self)).is_zero()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct G1Prepared(pub G1Affine);

impl G1Prepared {
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn from_affine(p: G1Affine) -> Self {
        G1Prepared(p)
    }
}

impl Default for G1Prepared {
    fn default() -> Self {
        G1Prepared(G1Affine::prime_subgroup_generator())
    }
}
