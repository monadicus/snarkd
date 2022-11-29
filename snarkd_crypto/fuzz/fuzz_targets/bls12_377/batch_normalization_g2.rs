#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{
    test::tests::projective::batch_normalization, G2Parameters, Projective, SWProjective,
};

fuzz_target!(|data: Vec<SWProjective<G2Parameters>>| {
    if data.iter().all(|v| !v.is_normalized()) {
        batch_normalization(data).unwrap();
    }
});
