#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{G2Affine, G2Prepared};

fuzz_target!(|data: G2Affine| {
    let _ = data.is_in_correct_subgroup_assuming_on_curve();
    let _ = G2Prepared::from_affine(data);
});
