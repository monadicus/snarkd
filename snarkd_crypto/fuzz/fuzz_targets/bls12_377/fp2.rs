#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{Field, Fp, Fp2};

fuzz_target!(|data: (Fp2, Fp2, Fp, Vec<u64>, usize)| {
    let (fp1, fp2, fp3, exp, power) = data;

    let _ = Fp2::mul_fp_by_nonresidue(&fp3);
    let _ = fp1.clone().mul_by_fp(&fp3);
    let _ = fp1.norm();
    let _ = fp1.legendre();
    let _ = fp1.clone().frobenius_map(power);
    let _ = fp1.double();
    let _ = fp1.square();
    let _ = fp1.inverse();
    let _ = fp1.sqrt();
    let _ = fp1.glv_endomorphism();
    let _ = fp1.pow(&exp);

    let _ = fp1 + fp2;
    let _ = fp1 - fp2;
    let _ = fp1 * fp2;
    let _ = -fp1;
    let _ = fp1 / (fp2 + Fp2::ONE);

    let mut tmp = fp1;
    tmp += fp2;
    tmp -= fp2;
    tmp *= fp2;
    tmp /= fp2 + Fp2::ONE;
});
