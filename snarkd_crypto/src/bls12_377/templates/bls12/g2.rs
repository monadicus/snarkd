use crate::{
    templates::{
        bls12::{Bls12Parameters, TwistType},
        short_weierstrass_jacobian::{Affine, Projective},
    },
    traits::{AffineCurve, ShortWeierstrassParameters},
};
use snarkvm_fields::{Field, Fp2, One, Zero};
use snarkvm_utilities::{bititerator::BitIteratorBE, serialize::*, ToBytes};

use std::io::{Result as IoResult, Write};

pub type G2Affine<P> = Affine<<P as Bls12Parameters>::G2Parameters>;
pub type G2Projective<P> = Projective<<P as Bls12Parameters>::G2Parameters>;
