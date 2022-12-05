#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{test::tests::field::sum_of_products, Fp12};

fuzz_target!(|data: (Vec<Fp12>, Vec<Fp12>)| {
    let (a, b) = data;
    sum_of_products(a, b).unwrap();
});
