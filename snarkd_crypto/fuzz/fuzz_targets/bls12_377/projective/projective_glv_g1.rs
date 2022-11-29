#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{
    test::tests::projective::G1ProjectiveNs, G1Parameters, SWProjective, Scalar,
};

fuzz_target!(|data: (SWProjective<G1Parameters>, Scalar)| {
    let (a, b) = data;
    G1ProjectiveNs::projective_glv(a, b).unwrap();
});
