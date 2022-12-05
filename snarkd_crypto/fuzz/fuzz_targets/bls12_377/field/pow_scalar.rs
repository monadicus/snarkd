#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{test::tests::field::pow, Scalar};

fuzz_target!(|data: Scalar| {
    pow(data).unwrap();
});
