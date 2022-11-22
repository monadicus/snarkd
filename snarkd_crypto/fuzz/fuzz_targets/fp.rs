#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{Field, Fp};

fuzz_target!(|data: (Fp, Fp, Vec<u64>)| {
    let (fp1, fp2, exp) = data;

    let _ = fp1.legendre();
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
    let _ = fp1 / (fp2 + Fp::ONE);

    let mut tmp = fp1;
    tmp += fp2;
    tmp -= fp2;
    tmp *= fp2;
    tmp /= fp2 + Fp::ONE;
});
