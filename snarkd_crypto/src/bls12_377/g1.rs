use snarkvm_fields::{field, Field, One, PrimeField, Zero};
use snarkvm_utilities::{
    biginteger::{BigInteger256, BigInteger384},
    BigInteger, BitIteratorBE,
};

use crate::{
    bls12_377::{Fq, Fr},
    templates::bls12::Bls12Parameters,
    traits::{ModelParameters, ShortWeierstrassParameters},
    AffineCurve, ProjectiveCurve,
};

use std::ops::Neg;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Bls12_377G1Parameters;

impl ModelParameters for Bls12_377G1Parameters {
    type BaseField = Fq;
    type ScalarField = Fr;
}

impl ShortWeierstrassParameters for Bls12_377G1Parameters {
    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G1_GENERATOR_X, G1_GENERATOR_Y);

    /// B1 = x^2 - 1
    const B1: Fr = uint!(91893752504881257701523279626832445440_U256);

    /// B2 = x^2
    const B2: Fr = uint!(91893752504881257701523279626832445441_U256);

    /// COFACTOR = (x - 1)^2 / 3  = 30631250834960419227450344600217059328
    const COFACTOR: &'static [u64] = &[0x0, 0x170b5d4430000000];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    ///              = 5285428838741532253824584287042945485047145357130994810877
    const COFACTOR_INV: Fr = uint!(5285428838741532253824584287042945485047145357130994810877_U256);

    const PHI: Fq = uint!(258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047231_U384);

    /// R128 = 2^128 - 1
    const R128: Fr = uint!(340282366920938463463374607431768211455_U256);

    /// WEIERSTRASS_A = 0
    const WEIERSTRASS_A: Fq = uint!(0_U384);

    /// WEIERSTRASS_B = 1
    const WEIERSTRASS_B: Fq = uint!(1_U384);

    #[inline(always)]
    fn mul_by_a(_: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }

    fn is_in_correct_subgroup_assuming_on_curve(p: &super::G1Affine) -> bool {
        let phi = |mut p: super::G1Affine| {
            debug_assert!(Self::PHI.pow([3]).is_one());
            p.x *= Self::PHI;
            p
        };
        let x_square = Fr::from(super::Bls12_377Parameters::X[0]).square();
        (phi(*p)
            .mul_bits(BitIteratorBE::new_without_leading_zeros(x_square.to_repr()))
            .add_mixed(p))
        .is_zero()
    }

    fn glv_endomorphism(
        mut p: crate::templates::short_weierstrass_jacobian::Affine<Self>,
    ) -> crate::templates::short_weierstrass_jacobian::Affine<Self> {
        p.x *= &Self::PHI;
        p
    }

    fn mul_projective(
        p: crate::templates::short_weierstrass_jacobian::Projective<Self>,
        by: Self::ScalarField,
    ) -> crate::templates::short_weierstrass_jacobian::Projective<Self> {
        type ScalarBigInt = <Fr as PrimeField>::BigInteger;

        /// The scalar multiplication window size.
        const GLV_WINDOW_SIZE: usize = 4;

        /// The table size, used for w-ary NAF recoding.
        const TABLE_SIZE: i64 = 1 << (GLV_WINDOW_SIZE + 1);
        const HALF_TABLE_SIZE: i64 = 1 << (GLV_WINDOW_SIZE);
        const MASK_FOR_MOD_TABLE_SIZE: u64 = (TABLE_SIZE as u64) - 1;
        /// The GLV table length.
        const L: usize = 1 << (GLV_WINDOW_SIZE - 1);

        let decomposition = by.decompose(
            &Self::Q1,
            &Self::Q2,
            Self::B1,
            Self::B2,
            Self::R128,
            &Self::HALF_R,
        );

        // Prepare tables.
        let mut t_1 = Vec::with_capacity(L);
        let double = crate::templates::short_weierstrass_jacobian::Affine::<Self>::from(p.double());
        t_1.push(p);
        for i in 1..L {
            t_1.push(t_1[i - 1].add_mixed(&double));
        }
        let t_1 =
            crate::templates::short_weierstrass_jacobian::Projective::<Self>::batch_normalization_into_affine(t_1);

        let t_2 = t_1
            .iter()
            .copied()
            .map(Self::glv_endomorphism)
            .collect::<Vec<_>>();

        let mod_signed = |d| {
            let d_mod_window_size = i64::try_from(d & MASK_FOR_MOD_TABLE_SIZE).unwrap();
            if d_mod_window_size >= HALF_TABLE_SIZE {
                d_mod_window_size - TABLE_SIZE
            } else {
                d_mod_window_size
            }
        };
        let to_wnaf = |e: Self::ScalarField| -> Vec<i32> {
            let mut naf = vec![];
            let mut e = e.to_repr();
            while !e.is_zero() {
                let next = if e.is_odd() {
                    let naf_sign = mod_signed(e.as_ref()[0]);
                    if naf_sign < 0 {
                        e.add_nocarry(&ScalarBigInt::from(-naf_sign as u64));
                    } else {
                        e.sub_noborrow(&ScalarBigInt::from(naf_sign as u64));
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

        let naf_add =
            |table: &[crate::templates::short_weierstrass_jacobian::Affine<Self>],
             naf: i32,
             acc: &mut crate::templates::short_weierstrass_jacobian::Projective<Self>| {
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
        let mut acc = crate::templates::short_weierstrass_jacobian::Projective::<Self>::zero();
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

///
/// G1_GENERATOR_X =
/// 89363714989903307245735717098563574705733591463163614225748337416674727625843187853442697973404985688481508350822
///
/// See `snarkvm_algorithms::hash_to_curve::tests::bls12_377` for tests.
///
pub const G1_GENERATOR_X: Fq = uint!(89363714989903307245735717098563574705733591463163614225748337416674727625843187853442697973404985688481508350822_U384);

///
/// G1_GENERATOR_Y =
/// 3702177272937190650578065972808860481433820514072818216637796320125658674906330993856598323293086021583822603349
///
/// See `snarkvm_algorithms::hash_to_curve::tests::bls12_377` for tests.
///
pub const G1_GENERATOR_Y: Fq = uint!(3702177272937190650578065972808860481433820514072818216637796320125658674906330993856598323293086021583822603349_U384);

#[cfg(test)]
mod tests {
    use rand::Rng;
    use snarkvm_fields::Field;
    use snarkvm_utilities::{BitIteratorBE, TestRng, Uniform};

    use crate::AffineCurve;

    use super::{super::G1Affine, *};

    #[test]
    fn test_subgroup_membership() {
        let rng = &mut TestRng::default();

        for _ in 0..1000 {
            let p = G1Affine::rand(rng);
            assert!(Bls12_377G1Parameters::is_in_correct_subgroup_assuming_on_curve(&p));
            let x = Fq::rand(rng);
            let greatest = rng.gen();

            if let Some(p) = G1Affine::from_x_coordinate(x, greatest) {
                assert_eq!(
                    Bls12_377G1Parameters::is_in_correct_subgroup_assuming_on_curve(&p),
                    p.mul_bits(BitIteratorBE::new(Fr::characteristic()))
                        .is_zero(),
                );
            }
        }
    }
}
