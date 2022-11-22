#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{Field, Fp12, Fp2};

fuzz_target!(|data: (Fp12, Fp12, Fp2, Fp2, Fp2, u64, Vec<u64>, usize)| {
    let (fp1, fp2, fp3, fp4, fp5, exp1, exp2, power) = data;

    let _ = fp1.cyclotomic_square();
    let _ = fp1.cyclotomic_exp(exp1);
    let _ = fp1.clone().mul_by_034(&fp3, &fp4, &fp5);
    let _ = fp1.clone().mul_by_014(&fp3, &fp4, &fp5);
    let _ = fp1.clone().frobenius_map(power);
    let _ = fp1.double();
    let _ = fp1.square();
    let _ = fp1.inverse();
    let _ = fp1.pow(&exp2);

    let _ = fp1 + fp2;
    let _ = fp1 - fp2;
    let _ = fp1 * fp2;
    let _ = -fp1;
    let _ = fp1 / (fp2 + Fp12::ONE);

    let mut tmp = fp1;
    tmp += fp2;
    tmp -= fp2;
    tmp *= fp2;
    tmp /= fp2 + Fp12::ONE;
});
