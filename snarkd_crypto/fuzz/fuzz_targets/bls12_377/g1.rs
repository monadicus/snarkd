#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::G1Affine;

fuzz_target!(|data: G1Affine| {
    let _ = data.is_in_correct_subgroup_assuming_on_curve();
});
