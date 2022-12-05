#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{test::tests::field::square, Fp12};

fuzz_target!(|data: Fp12| {
    square(data).unwrap();
});
