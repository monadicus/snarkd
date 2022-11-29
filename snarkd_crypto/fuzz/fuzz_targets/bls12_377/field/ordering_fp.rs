#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{test::tests::field::ordering, Fp};

fuzz_target!(|data: (Fp, Fp)| {
    let (a, b) = data;
    ordering(a, b);
});
