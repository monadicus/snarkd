#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{test::tests::projective::neg, G2Parameters, SWProjective, Scalar};

fuzz_target!(|data: (SWProjective<G2Parameters>, Scalar)| {
    let (a, b) = data;
    neg(a, b).unwrap();
});
