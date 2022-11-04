use crate::bls12_377::{
    field::Field, group::Group, sw_affine::SWAffine, sw_projective::SWProjective, Affine, Fp, Fp2,
    G1Parameters, Projective, Scalar, X,
};
use bitvec::prelude::*;
use ruint::uint;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct G2Parameters;

impl Group for G2Parameters {
    type BaseField = Fp2;

    /// COFACTOR =
    /// 7923214915284317143930293550643874566881017850177945424769256759165301436616933228209277966774092486467289478618404761412630691835764674559376407658497
    const COFACTOR: &'static [u64] = &[
        0x0000000000000001,
        0x452217cc90000000,
        0xa0f3622fba094800,
        0xd693e8c36676bd09,
        0x8c505634fae2e189,
        0xfbb36b00e1dcc40c,
        0xddd88d99a6f6a829,
        0x26ba558ae9562a,
    ];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    ///              = 6764900296503390671038341982857278410319949526107311149686707033187604810669
    const COFACTOR_INV: Scalar = Scalar(
        uint!(6764900296503390671038341982857278410319949526107311149686707033187604810669_U256),
    );

    /// AFFINE_GENERATOR_COEFFS = (G2_GENERATOR_X, G2_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G2_GENERATOR_X, G2_GENERATOR_Y);

    // As per https://eprint.iacr.org/2012/072.pdf,
    // this curve has b' = b/i, where b is the COEFF_B of G1, and x^6 -i is
    // the irreducible poly used to extend from Fp2 to Fp12.
    // In our case, i = u (App A.3, T_6).
    /// WEIERSTRASS_B = [0,
    /// 155198655607781456406391640216936120121836107652948796323930557600032281009004493664981332883744016074664192874906]
    const B: Fp2 = Fp2 {
        c0: Fp(uint!(0_U384)),
        c1: Fp(
            uint!(155198655607781456406391640216936120121836107652948796323930557600032281009004493664981332883744016074664192874906_U384),
        ),
    };
}

pub const G2_GENERATOR_X: Fp2 = Fp2 {
    c0: G2_GENERATOR_X_C0,
    c1: G2_GENERATOR_X_C1,
};
pub const G2_GENERATOR_Y: Fp2 = Fp2 {
    c0: G2_GENERATOR_Y_C0,
    c1: G2_GENERATOR_Y_C1,
};

///
/// G2_GENERATOR_X_C0 =
/// 170590608266080109581922461902299092015242589883741236963254737235977648828052995125541529645051927918098146183295
///
pub const G2_GENERATOR_X_C0: Fp = Fp(
    uint!(170590608266080109581922461902299092015242589883741236963254737235977648828052995125541529645051927918098146183295_U384),
);

///
/// G2_GENERATOR_X_C1 =
/// 83407003718128594709087171351153471074446327721872642659202721143408712182996929763094113874399921859453255070254
///
pub const G2_GENERATOR_X_C1: Fp = Fp(
    uint!(83407003718128594709087171351153471074446327721872642659202721143408712182996929763094113874399921859453255070254_U384),
);

///
/// G2_GENERATOR_Y_C0 =
/// 1843833842842620867708835993770650838640642469700861403869757682057607397502738488921663703124647238454792872005
///
pub const G2_GENERATOR_Y_C0: Fp = Fp(
    uint!(1843833842842620867708835993770650838640642469700861403869757682057607397502738488921663703124647238454792872005_U384),
);

///
/// G2_GENERATOR_Y_C1 =
/// 33145532013610981697337930729788870077912093258611421158732879580766461459275194744385880708057348608045241477209
///
pub const G2_GENERATOR_Y_C1: Fp = Fp(
    uint!(33145532013610981697337930729788870077912093258611421158732879580766461459275194744385880708057348608045241477209_U384),
);

pub type G2Affine = SWAffine<G2Parameters>;
pub type G2Projective = SWProjective<G2Parameters>;

impl G2Affine {
    pub fn is_in_correct_subgroup_assuming_on_curve(&self) -> bool {
        self.mul_bits(
            Scalar::characteristic()
                .0
                .as_limbs()
                .iter()
                .flat_map(|limb| limb.view_bits::<Lsb0>())
                .map(|b| *b)
                .rev()
                .collect::<Vec<_>>(),
        )
        .is_zero()
    }
}

type CoeffTriplet = (Fp2, Fp2, Fp2);

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
    x: Fp2,
    y: Fp2,
    z: Fp2,
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
            z: Fp2::one(),
        };

        let bit_iterator = X.view_bits::<Msb0>();
        let mut ell_coeffs = Vec::with_capacity(bit_iterator.len());

        // `one_half` = 1/2 in the field.
        let one_half = Fp::half();

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
fn doubling_step(r: &mut G2HomProjective, two_inv: &Fp) -> CoeffTriplet {
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
