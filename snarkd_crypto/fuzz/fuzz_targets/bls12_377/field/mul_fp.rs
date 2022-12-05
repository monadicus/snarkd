#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{test::tests::field::mul, Fp};

fuzz_target!(|data: (Fp, Fp, Fp)| {
    let (a, b, c) = data;
    mul(a, b, c).unwrap();
});
