#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{adc, mac_with_carry, pairing, G1Affine, G2Affine};

fuzz_target!(|data: (G1Affine, G2Affine, u64, u64, u64, u64)| {
    let (g1, g2, a, b, c, carry) = data;

    let _ = pairing(g1, g2);
    let _ = mac_with_carry(a, b, c, &mut carry.clone());
    let _ = adc(&mut a.clone(), b, carry);
});
