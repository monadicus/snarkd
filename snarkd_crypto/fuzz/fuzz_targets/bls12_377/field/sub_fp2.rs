#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{test::tests::field::sub, Fp2};

fuzz_target!(|data: (Fp2, Fp2)| {
    let (a, b) = data;
    sub(a, b).unwrap();
});
