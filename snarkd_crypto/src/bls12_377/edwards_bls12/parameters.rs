use crate::{
    edwards_bls12::{Fq, Fr},
    errors::GroupError,
    templates::twisted_edwards_extended::{Affine, Projective},
    traits::{AffineCurve, ModelParameters, MontgomeryParameters, TwistedEdwardsParameters},
};
use snarkvm_fields::field;
use snarkvm_utilities::biginteger::BigInteger256;

use std::str::FromStr;

pub type EdwardsAffine = Affine<EdwardsParameters>;
pub type EdwardsProjective = Projective<EdwardsParameters>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EdwardsParameters;

impl ModelParameters for EdwardsParameters {
    type BaseField = Fq;
    type ScalarField = Fr;
}

impl TwistedEdwardsParameters for EdwardsParameters {
    type MontgomeryParameters = EdwardsParameters;

    /// Generated randomly
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) = (GENERATOR_X, GENERATOR_Y);

    /// COFACTOR = 4
    const COFACTOR: &'static [u64] = &[4];

    /// COFACTOR_INV = 527778859339273151515551558673846658209717731602102048798421311598680340096
    const COFACTOR_INV: Fr =
        uint!(527778859339273151515551558673846658209717731602102048798421311598680340096_U256);

    /// EDWARDS_A = -1
    const EDWARDS_A: Fq =
        uint!(8444461749428370424248824938781546531375899335154063827935233455917409239040_U384);

    /// EDWARDS_D = 3021
    const EDWARDS_D: Fq = uint!(3021_U384);

    /// Multiplication by `a` is just negation.
    /// Is `a` 1 or -1?
    #[inline(always)]
    fn mul_by_a(elem: &Self::BaseField) -> Self::BaseField {
        -*elem
    }
}

impl MontgomeryParameters for EdwardsParameters {
    type TwistedEdwardsParameters = EdwardsParameters;

    /// MONTGOMERY_A = 3990301581132929505568273333084066329187552697088022219156688740916631500114
    ///              = 0x8D26E3FADA9010A26949031ECE3971B93952AD84D4753DDEDB748DA37E8F552
    const MONTGOMERY_A: Fq =
        uint!(3990301581132929505568273333084066329187552697088022219156688740916631500114_U384);

    /// MONTGOMERY_B = 4454160168295440918680551605697480202188346638066041608778544715000777738925
    ///              = 0x9D8F71EEC83A44C3A1FBCEC6F5418E5C6154C2682B8AC231C5A3725C8170AAD
    const MONTGOMERY_B: Fq =
        uint!(4454160168295440918680551605697480202188346638066041608778544715000777738925_U384);
}

impl FromStr for EdwardsAffine {
    type Err = GroupError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        s = s.trim();
        if s.is_empty() {
            return Err(GroupError::ParsingEmptyString);
        }
        if s.len() < 3 {
            return Err(GroupError::InvalidString);
        }
        if !(s.starts_with('(') && s.ends_with(')')) {
            return Err(GroupError::InvalidString);
        }
        let mut point = Vec::new();
        for substr in s.split(|c| c == '(' || c == ')' || c == ',' || c == ' ') {
            if !substr.is_empty() {
                point.push(Fq::from_str(substr)?);
            }
        }
        if point.len() != 2 {
            return Err(GroupError::InvalidGroupElement);
        }
        let point = EdwardsAffine::new(point[0], point[1], point[0] * point[1]);

        if !point.is_on_curve() {
            Err(GroupError::InvalidGroupElement)
        } else {
            Ok(point)
        }
    }
}

/// GENERATOR_X =
/// 7810607721416582242904415504650443951498042435501746664987470571546413371306
const GENERATOR_X: Fq =
    uint!(7810607721416582242904415504650443951498042435501746664987470571546413371306_U384);

/// GENERATOR_Y =
/// 1867362672570137759132108893390349941423731440336755218616442213142473202417
const GENERATOR_Y: Fq =
    uint!(1867362672570137759132108893390349941423731440336755218616442213142473202417_U384);
