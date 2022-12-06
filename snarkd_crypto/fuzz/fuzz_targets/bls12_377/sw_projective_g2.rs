#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{
    Affine, Fp, G1Affine, G2Parameters, Projective, SWAffine, SWProjective, Scalar,
};

fuzz_target!(|data: (
    SWProjective<G2Parameters>,
    SWProjective<G2Parameters>,
    Vec<SWProjective<G2Parameters>>,
    SWAffine<G2Parameters>,
    Scalar,
)| {
    let (p1, p2, projectives, affine, s) = data;
    let _ = SWProjective::<G2Parameters>::batch_normalization(&mut projectives.clone());
    let _ = p1.add_mixed(&affine);
    let _ = p1.double();
    let _ = p1.to_affine();

    let _ = -p1;
    let _ = p1 + p2;
    let _ = p1 - p2;
    let _ = p1 * s;

    let mut tmp = p1.clone();
    tmp += p2;
    let mut tmp = p1.clone();
    tmp -= p2;
});
