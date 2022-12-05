#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{test::tests::field::inversion, Fp6, Field};

fuzz_target!(|data: Fp6| {
    if !data.is_zero() {
        inversion(data).unwrap();
    }
});
