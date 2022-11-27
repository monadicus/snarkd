#![no_main]

use libfuzzer_sys::fuzz_target;
use snarkd_crypto::bls12_377::{Field, Scalar};

fuzz_target!(|data: (
    Scalar,
    Scalar,
    Scalar,
    Scalar,
    [u64; 4],
    [u64; 4],
    [u64; 8],
    usize,
    Vec<Scalar>,
    Vec<u64>,
)| {
    let (s1, s2, s3, s4, q1, q2, half_r, n, v, exp) = data;

    let _ = s1.legendre();
    let _ = s1.decompose(&q1, &q2, s2, s3, s4, &half_r);
    let _ = Scalar::get_root_of_unity(n);
    let _ = Scalar::batch_inversion(&mut v.clone());
    let _ = Scalar::batch_inversion_and_mul(&mut v.clone(), &s1);

    let _ = s1.double();
    let _ = s1.square();
    let _ = s1.inverse();
    let _ = s1.sqrt();
    let _ = s1.pow(&exp);

    let _ = s1 + s2;
    let _ = s1 - s2;
    let _ = s1 * s2;
    let _ = -s1;
    let _ = s1 / (s2 + Scalar::ONE);

    let mut tmp = s1;
    tmp += s2;
    tmp -= s2;
    tmp *= s2;
    tmp /= s2 + Scalar::ONE;
});
