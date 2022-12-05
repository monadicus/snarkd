#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{test::tests::projective::add, G2Parameters, SWProjective};

fuzz_target!(|data: (
    SWProjective<G2Parameters>,
    SWProjective<G2Parameters>,
    SWProjective<G2Parameters>
)| {
    let (a, b, c) = data;
    add(a, b, c).unwrap();
});
