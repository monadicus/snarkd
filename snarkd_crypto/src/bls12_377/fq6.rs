use crate::bls12_377::{Fq, Fq2, Fq2Parameters};
use snarkvm_fields::{
    field,
    fp6_3over2::{Fp6, Fp6Parameters},
    Fp2Parameters,
};
use snarkvm_utilities::biginteger::BigInteger384;

pub struct Fq6 {
    pub c0: Fq2,
    pub c1: Fq2,
    pub c2: Fq2,
}

const FROBENIUS_COEFF_FP6_C1: [Fq2; 6] = [
    // Fp2::NONRESIDUE^(((q^0) - 1) / 3)
    Fq2 {
        c0: uint!(1_U384),
        c1: uint!(0_U384),
    },
    // Fp2::NONRESIDUE^(((q^1) - 1) / 3)
    Fq2 {
        c0: uint!(80949648264912719408558363140637477264845294720710499478137287262712535938301461879813459410946_U384),
        c1: uint!(0_U384),
    },
    // Fp2::NONRESIDUE^(((q^2) - 1) / 3)
    Fq2 {
        c0: uint!(80949648264912719408558363140637477264845294720710499478137287262712535938301461879813459410945_U384),
        c1: uint!(0_U384),
    },
    // Fp2::NONRESIDUE^(((q^3) - 1) / 3)
    Fq2 {
        c0: uint!(258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458176_U384),
        c1: uint!(0_U384),
    },
    // Fp2::NONRESIDUE^(((q^4) - 1) / 3)
    Fq2 {
        c0: uint!(258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047231_U384),
        c1: uint!(0_U384),
    },
    // Fp2::NONRESIDUE^(((q^5) - 1) / 3)
    Fq2 {
        c0: uint!(258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047232_U384),
        c1: uint!(0_U384),
    },
];

const FROBENIUS_COEFF_FP6_C2: [Fq2; 6] = [
    // Fp2::NONRESIDUE^((2*(q^0) - 2) / 3)
    Fq2 {
        c0: uint!(1_U384),
        c1: uint!(0_U384),
    },
    // Fp2::NONRESIDUE^((2*(q^1) - 2) / 3)
    Fq2 {
        c0: uint!(80949648264912719408558363140637477264845294720710499478137287262712535938301461879813459410945_U384),
        c1: uint!(0_U384),
    },
    // Fp2::NONRESIDUE^((2*(q^2) - 2) / 3)
    Fq2 {
        c0: uint!(258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047231_U384),
        c1: uint!(0_U384),
    },
    // Fp2::NONRESIDUE^((2*(q^3) - 2) / 3)
    Fq2 {
        c0: uint!(1_U384),
        c1: uint!(0_U384),
    },
    // Fp2::NONRESIDUE^((2*(q^4) - 2) / 3)
    Fq2 {
        c0: uint!(80949648264912719408558363140637477264845294720710499478137287262712535938301461879813459410945_U384),
        c1: uint!(0_U384),
    },
    // Fp2::NONRESIDUE^((2*(q^5) - 2) / 3)
    Fq2 {
        c0: uint!(258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047231_U384),
        c1: uint!(0_U384),
    },
];

/// NONRESIDUE = U
const NONRESIDUE: Fq2 = Fq2 {
    c0: uint!(0_U384),
    c1: uint!(1_U384),
};

#[inline(always)]
fn mul_fp2_by_nonresidue(fe: &Fq2) -> Fq2 {
    // Karatsuba multiplication with constant other = u.
    let c0 = Fq2Parameters::mul_fp_by_nonresidue(&fe.c1);
    let c1 = fe.c0;
    Fq2 { c0, c1 }
}

#[cfg(test)]
mod test {
    use snarkvm_fields::{One, Zero};
    use snarkvm_utilities::rand::{TestRng, Uniform};

    use super::*;

    #[test]
    fn test_fq2_mul_nonresidue() {
        let mut rng = TestRng::default();

        let nqr = Fq2::new(Fq::zero(), Fq::one());
        println!("One: {:?}", Fq::one());

        for _ in 0..1000 {
            let mut a = Fq2::rand(&mut rng);
            let mut b = a;
            a *= &Fq6Parameters::NONRESIDUE;
            b *= &nqr;

            assert_eq!(a, b);
        }
    }
}
