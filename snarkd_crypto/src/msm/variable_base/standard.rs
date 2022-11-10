use crate::bls12_377::{scalar, Affine, Projective};
use rayon::prelude::*;
use ruint::Uint;

fn update_buckets<A: Affine>(
    base: &A,
    mut scalar: Uint<256, 4>,
    w_start: usize,
    c: usize,
    buckets: &mut [A::Projective],
) {
    // We right-shift by w_start, thus getting rid of the lower bits.
    scalar.divn(w_start as u32);

    // We mod the remaining bits by the window size.
    let scalar = scalar.as_limbs()[0] % (1 << c);

    // If the scalar is non-zero, we update the corresponding bucket.
    // (Recall that `buckets` doesn't have a zero bucket.)
    if scalar != 0 {
        buckets[(scalar - 1) as usize].add_assign_mixed(base);
    }
}

fn standard_window<A: Affine>(
    bases: &[A],
    scalars: &[Uint<256, 4>],
    w_start: usize,
    c: usize,
) -> (A::Projective, usize) {
    let mut res = A::Projective::ZERO;
    let fr_one = Uint::<256, 4>::from(1);

    // We only process unit scalars once in the first window.
    if w_start == 0 {
        scalars
            .iter()
            .zip(bases)
            .filter(|(&s, _)| s == fr_one)
            .for_each(|(_, base)| {
                res.add_assign_mixed(base);
            });
    }

    // We don't need the "zero" bucket, so we only have 2^c - 1 buckets
    let window_size = if (w_start % c) != 0 { w_start % c } else { c };
    let mut buckets = vec![A::Projective::ZERO; (1 << window_size) - 1];
    scalars
        .iter()
        .zip(bases)
        .filter(|(&s, _)| s > fr_one)
        .for_each(|(&scalar, base)| update_buckets(base, scalar, w_start, c, &mut buckets));
    // G::Projective::batch_normalization(&mut buckets);

    for running_sum in buckets
        .into_iter()
        .rev()
        .scan(A::Projective::ZERO, |sum, b| {
            *sum += b;
            Some(*sum)
        })
    {
        res += running_sum;
    }

    (res, window_size)
}

pub fn msm<A: Affine>(bases: &[A], scalars: &[Uint<256, 4>]) -> A::Projective {
    // Determine the bucket size `c` (chosen empirically).
    let c = match scalars.len() < 32 {
        true => 1,
        false => crate::msm::ln_without_floats(scalars.len()) + 2,
    };

    let num_bits = scalar::MODULUS_BITS;

    // Each window is of size `c`.
    // We divide up the bits 0..num_bits into windows of size `c`, and
    // in parallel process each such window.
    let window_sums: Vec<_> = cfg_into_iter!(0..num_bits)
        .step_by(c)
        .map(|w_start| standard_window(bases, scalars, w_start, c))
        .collect();

    // We store the sum for the lowest window.
    let (lowest, window_sums) = window_sums.split_first().unwrap();

    // We're traversing windows from high to low.
    window_sums
        .iter()
        .rev()
        .fold(A::Projective::ZERO, |mut total, (sum_i, window_size)| {
            total += sum_i;
            for _ in 0..*window_size {
                total.double_in_place();
            }
            total
        })
        + lowest.0
}
