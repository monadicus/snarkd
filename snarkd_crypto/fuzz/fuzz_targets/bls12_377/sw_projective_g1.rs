#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{
    Affine, Fp, G1Affine, G1Parameters, Projective, SWAffine, SWProjective, Scalar,
};

fuzz_target!(|data: (
    SWProjective<G1Parameters>,
    SWProjective<G1Parameters>,
    Vec<SWProjective<G1Parameters>>,
    SWAffine<G1Parameters>,
    Scalar,
)| {
    let (p1, p2, projectives, affine, s) = data;
    let _ = SWProjective::<G1Parameters>::batch_normalization(&mut projectives.clone());
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
