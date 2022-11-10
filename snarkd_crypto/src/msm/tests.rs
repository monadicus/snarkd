use crate::{
    bls12_377::{Affine, Field, G1Affine, Projective, Scalar},
    msm::*,
};
use bitvec::prelude::*;
use core::ops::Deref;
use ruint::Uint;

fn naive_variable_base_msm<A: Affine>(bases: &[A], scalars: &[Uint<256, 4>]) -> A::Projective {
    let mut acc = A::Projective::ZERO;

    for (base, scalar) in bases.iter().zip(scalars.iter()) {
        acc += base.mul_bits(
            scalar
                .as_limbs()
                .iter()
                .flat_map(|limb| limb.view_bits::<Lsb0>())
                .map(|bit| *bit.deref())
                .rev()
                .collect::<Vec<_>>(),
        );
    }
    acc
}

#[test]
fn variable_base_test_with_bls12() {
    const SAMPLES: usize = 1 << 10;

    let v = (0..SAMPLES).map(|_| Scalar::rand().0).collect::<Vec<_>>();
    let g = (0..SAMPLES).map(|_| G1Affine::rand()).collect::<Vec<_>>();

    let naive = naive_variable_base_msm(g.as_slice(), v.as_slice());
    let fast = VariableBase::msm(g.as_slice(), v.as_slice());

    assert_eq!(naive.to_affine(), fast.to_affine());
}

#[test]
fn variable_base_test_with_bls12_unequal_numbers() {
    const SAMPLES: usize = 1 << 10;

    let v = (0..SAMPLES - 1)
        .map(|_| Scalar::rand().0)
        .collect::<Vec<_>>();
    let g = (0..SAMPLES).map(|_| G1Affine::rand()).collect::<Vec<_>>();

    let naive = naive_variable_base_msm(g.as_slice(), v.as_slice());
    let fast = VariableBase::msm(g.as_slice(), v.as_slice());

    assert_eq!(naive.to_affine(), fast.to_affine());
}
