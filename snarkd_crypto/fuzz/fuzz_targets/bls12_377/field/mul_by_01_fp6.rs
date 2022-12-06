#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{test::tests::field::Fp6Ns, Fp2, Fp6};

fuzz_target!(|data: (Fp2, Fp2, Fp6)| {
    let (a, b, c) = data;
    Fp6Ns::mul_by_01(a, b, c).unwrap();
});
