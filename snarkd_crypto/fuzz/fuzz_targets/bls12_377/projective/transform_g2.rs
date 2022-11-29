#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{test::tests::projective::transform, G2Parameters, SWProjective};

fuzz_target!(|data: SWProjective<G2Parameters>| {
    transform(data).unwrap();
});
