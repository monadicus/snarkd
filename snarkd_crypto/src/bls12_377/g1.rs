use crate::bls12_377::{
    field::Field, parameters::Parameters, sw_affine::SWAffine, sw_projective::SWProjective, Affine,
    Fp, Projective, Scalar, X,
};
use bitvec::prelude::*;

use ruint::{uint, Uint};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct G1Parameters;

impl Parameters for G1Parameters {
    type BaseField = Fp;

    /// COFACTOR = (x - 1)^2 / 3  = 30631250834960419227450344600217059328
    const COFACTOR: &'static [u64] = &[0x0, 0x170b5d4430000000];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    ///              = 5285428838741532253824584287042945485047145357130994810877
    const COFACTOR_INV: Scalar =
        Scalar(uint!(5285428838741532253824584287042945485047145357130994810877_U256));

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G1_GENERATOR_X, G1_GENERATOR_Y);

    /// WEIERSTRASS_B = 1
    const B: Fp = Fp(uint!(1_U384));
}

const G1_GENERATOR_X: Fp = Fp(
    uint!(89363714989903307245735717098563574705733591463163614225748337416674727625843187853442697973404985688481508350822_U384),
);

const G1_GENERATOR_Y: Fp = Fp(
    uint!(3702177272937190650578065972808860481433820514072818216637796320125658674906330993856598323293086021583822603349_U384),
);

pub type G1Affine = SWAffine<G1Parameters>;
pub type G1Projective = SWProjective<G1Parameters>;

impl G1Affine {
    pub fn is_in_correct_subgroup_assuming_on_curve(&self) -> bool {
        let phi = |mut p: Self| {
            debug_assert!(Fp::PHI.pow(&[3]).is_one());
            p.x *= Fp::PHI;
            p
        };
        let x_square = Scalar(Uint::from(X)).square();
        let bits = x_square
            .0
            .as_limbs()
            .iter()
            .flat_map(|x| x.view_bits::<Lsb0>())
            .map(|b| *b)
            .rev()
            .collect::<Vec<_>>();
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

#[cfg(test)]
mod tests {
    use super::{super::G1Affine, *};
    use crate::bls12_377::field::Field;
    use rand::Rng;
    use rayon::prelude::{IntoParallelIterator, ParallelIterator};

    #[test]
    fn test_subgroup_membership() {
        (0..1000).into_par_iter().for_each(|_| {
            let p = G1Affine::rand();
            assert!(p.is_in_correct_subgroup_assuming_on_curve());
            let x = Fp::rand();
            let greatest = rand::thread_rng().gen();

            if let Some(p) = G1Affine::from_x_coordinate(x, greatest) {
                assert_eq!(
                    p.is_in_correct_subgroup_assuming_on_curve(),
                    p.mul_bits(
                        Scalar::characteristic()
                            .iter()
                            .flat_map(|limb| limb.view_bits::<Lsb0>())
                            .map(|b| *b)
                            .rev()
                            .collect::<Vec<_>>()
                    )
                    .is_zero(),
                );
            }
        })
    }
}
