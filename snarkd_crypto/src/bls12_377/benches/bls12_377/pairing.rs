use snarkvm_curves::{
    bls12_377::{
        Bls12_377, Bls12_377Parameters, Fq12, G1Affine, G1Projective as G1, G2Affine,
        G2Projective as G2,
    },
    templates::bls12::{G1Prepared, G2Prepared},
    traits::{PairingCurve, PairingEngine},
};
use snarkvm_utilities::rand::{TestRng, Uniform};

use criterion::Criterion;

use std::iter;

pub fn bench_pairing_miller_loop(c: &mut Criterion) {
    const SAMPLES: usize = 1000;

    let mut rng = TestRng::default();

    let v: Vec<(
        G1Prepared<Bls12_377Parameters>,
        G2Prepared<Bls12_377Parameters>,
    )> = (0..SAMPLES)
        .map(|_| {
            (
                G1Affine::from(G1::rand(&mut rng)).prepare(),
                G2Affine::from(G2::rand(&mut rng)).prepare(),
            )
        })
        .collect();

    let mut count = 0;
    c.bench_function("bls12_377: pairing_miller_loop", |c| {
        c.iter(|| {
            let tmp = Bls12_377::miller_loop(iter::once((&v[count].0, &v[count].1)));
            count = (count + 1) % SAMPLES;
            tmp
        })
    });
}

pub fn bench_pairing_final_exponentiation(c: &mut Criterion) {
    const SAMPLES: usize = 1000;

    let mut rng = TestRng::default();

    let v: Vec<Fq12> = (0..SAMPLES)
        .map(|_| {
            (
                G1Affine::from(G1::rand(&mut rng)).prepare(),
                G2Affine::from(G2::rand(&mut rng)).prepare(),
            )
        })
        .map(|(ref p, ref q)| Bls12_377::miller_loop([(p, q)].iter().copied()))
        .collect();

    let mut count = 0;
    c.bench_function("bls12_377: pairing_final_exponentiation", |c| {
        c.iter(|| {
            let tmp = Bls12_377::final_exponentiation(&v[count]);
            count = (count + 1) % SAMPLES;
            tmp
        })
    });
}

pub fn bench_pairing_full(c: &mut Criterion) {
    const SAMPLES: usize = 1000;

    let mut rng = TestRng::default();

    let v: Vec<(G1, G2)> = (0..SAMPLES)
        .map(|_| (G1::rand(&mut rng), G2::rand(&mut rng)))
        .collect();

    let mut count = 0;
    c.bench_function("bls12_377: pairing_full", |c| {
        c.iter(|| {
            let tmp = Bls12_377::pairing(v[count].0, v[count].1);
            count = (count + 1) % SAMPLES;
            tmp
        })
    });
}
