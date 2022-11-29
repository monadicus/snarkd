#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{test::tests::field::math_properties, Scalar};

fuzz_target!(|data: (Scalar, Scalar)| {
    let (a, b) = data;
    math_properties(a, b).unwrap();
});
