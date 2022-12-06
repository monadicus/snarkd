#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{test::tests::field::sub, Scalar};

fuzz_target!(|data: (Scalar, Scalar)| {
    let (a, b) = data;
    sub(a, b).unwrap();
});
