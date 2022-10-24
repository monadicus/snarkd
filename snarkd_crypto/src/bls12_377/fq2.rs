use crate::bls12_377::Fq;
use ruint::{uint, Uint};
use serde::{Deserialize, Serialize};
use snarkvm_fields::{field, Field, Fp2, Fp2Parameters};

pub struct Fq2 {
    pub c0: Fq,
    pub c1: Fq,
}

/// Coefficients for the Frobenius automorphism.
const FROBENIUS_COEFF_FP2_C1: [Fq; 2] = [
    // NONRESIDUE**(((q^0) - 1) / 2)
    uint!(1_U384),
    // NONRESIDUE**(((q^1) - 1) / 2)
    uint!(258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458176_U384),
];

/// NONRESIDUE = -5
const NONRESIDUE: Fq = uint!(258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458172_U384);

/// QUADRATIC_NONRESIDUE = U
const QUADRATIC_NONRESIDUE: (Fq, Fq) = (uint!(0_U384), uint!(1_U384));

#[inline(always)]
fn mul_fp_by_nonresidue(fe: &Self::Fp) -> Self::Fp {
    let original = fe;
    let mut fe = -fe.double();
    fe.double_in_place();
    fe - original
}
