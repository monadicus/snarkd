#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{Affine, Fp2, G2Affine, G2Parameters, SWAffine, SWProjective};

fuzz_target!(|data: (
    SWAffine<G2Parameters>,
    SWAffine<G2Parameters>,
    Fp2,
    Fp2,
    bool,
    Vec<bool>,
    SWProjective<G2Parameters>,
)| {
    let (sw1, sw2, x, y, infinity, bits, swp) = data;

    let _ = SWAffine::<G2Parameters>::from_coordinates(x, y, infinity);
    let _ = SWAffine::<G2Parameters>::from_x_coordinate(x, infinity);
    let _ = SWAffine::<G2Parameters>::from_x_coordinate(x, infinity);
    let _ = sw1.mul_bits(bits);
    let _ = sw1.mul_by_cofactor_inv();
    let _ = sw1.to_projective();
    let _ = sw1.is_on_curve();
    let _ = SWAffine::<G2Parameters>::batch_add_loop_1(
        &mut sw1.clone(),
        &mut sw2.clone(),
        &x,
        &mut y.clone(),
    );
    let _ = SWAffine::<G2Parameters>::batch_add_loop_2(&mut sw1.clone(), sw2, &mut x.clone());

    let _ = -sw1;
    let _ = SWAffine::<G2Parameters>::from(swp);
});
