#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{test::tests::field::sqrt, Fp};

fuzz_target!(|data: Fp| {
    sqrt(data).unwrap();
});
