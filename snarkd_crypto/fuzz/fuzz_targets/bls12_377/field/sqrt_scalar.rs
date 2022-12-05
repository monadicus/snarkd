#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{test::tests::field::sqrt, Scalar};

fuzz_target!(|data: Scalar| {
    sqrt(data).unwrap();
});
