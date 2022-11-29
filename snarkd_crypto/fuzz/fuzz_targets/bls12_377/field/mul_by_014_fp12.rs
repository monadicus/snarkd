#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{test::tests::field::Fp12Ns, Fp2, Fp12};

fuzz_target!(|data: (Fp2, Fp2, Fp2, Fp12)| {
    let (a, b, c, d) = data;
    Fp12Ns::mul_by_014(a, b, c, d).unwrap();
});
