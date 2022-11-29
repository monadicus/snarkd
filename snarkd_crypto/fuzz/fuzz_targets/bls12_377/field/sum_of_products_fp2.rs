#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{test::tests::field::sum_of_products, Fp2};

fuzz_target!(|data: (Vec<Fp2>, Vec<Fp2>)| {
    let (a, b) = data;
    sum_of_products(a, b).unwrap();
});
