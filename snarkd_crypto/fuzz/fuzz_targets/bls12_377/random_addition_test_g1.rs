#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{tests::random_addition_test, G1Parameters, SWProjective};

fuzz_target!(|data: (
    SWProjective<G1Parameters>,
    SWProjective<G1Parameters>,
    SWProjective<G1Parameters>
)| {
    let (a, b, c) = data;
    random_addition_test(a, b, c);
});
