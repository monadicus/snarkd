#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{Field, Fp2, Fp6};

fuzz_target!(|data: (Fp6, Fp6, Fp2, Fp2, Vec<u64>, usize)| {
    let (fp1, fp2, fp3, fp4, exp, power) = data;

    let _ = fp1.clone().frobenius_map(power);
    let _ = fp1.double();
    let _ = fp1.square();
    let _ = fp1.inverse();
    let _ = fp1.pow(&exp);

    let _ = fp1 + fp2;
    let _ = fp1 - fp2;
    let _ = fp1 * fp2;
    let _ = -fp1;
    let _ = fp1 / (fp2 + Fp6::ONE);

    let mut tmp = fp1;
    tmp += fp2;
    tmp -= fp2;
    tmp *= fp2;
    tmp /= fp2 + Fp6::ONE;

    let _ = Fp6::mul_fp2_by_nonresidue(&fp3);
    let _ = fp1.clone().mul_by_1(&fp3);
    let _ = fp1.clone().mul_by_01(&fp3, &fp4);
});
