#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{test::tests::field::neg, Scalar};

fuzz_target!(|data: Scalar| {
    neg(data).unwrap();
});
