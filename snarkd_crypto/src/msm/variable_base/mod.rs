mod batched;
mod standard;

#[cfg(all(feature = "cuda", target_arch = "x86_64"))]
mod cuda;

#[cfg(target_arch = "x86_64")]
pub mod prefetch;

use crate::bls12_377::{Affine, G1Affine};
use bitvec::prelude::*;
use core::any::TypeId;
use core::ops::Deref;
use ruint::Uint;

#[cfg(all(feature = "cuda", target_arch = "x86_64"))]
use core::sync::atomic::{AtomicBool, Ordering};

#[cfg(all(feature = "cuda", target_arch = "x86_64"))]
static HAS_CUDA_FAILED: AtomicBool = AtomicBool::new(false);

pub struct VariableBase;

impl VariableBase {
    pub fn msm<A: Affine + 'static>(bases: &[A], scalars: &[Uint<256, 4>]) -> A::Projective {
        // For BLS12-377, we perform variable base MSM using a batched addition technique.
        if TypeId::of::<A>() == TypeId::of::<G1Affine>() {
            #[cfg(all(feature = "cuda", target_arch = "x86_64"))]
            if !HAS_CUDA_FAILED.load(Ordering::SeqCst) {
                match cuda::msm_cuda(bases, scalars) {
                    Ok(x) => return x,
                    Err(_e) => {
                        HAS_CUDA_FAILED.store(true, Ordering::SeqCst);
                        eprintln!("CUDA failed, moving to the next MSM method");
                    }
                }
            }
            batched::msm(bases, scalars)
        }
        // For all other curves, we perform variable base MSM using Pippenger's algorithm.
        else {
            standard::msm(bases, scalars)
        }
    }

    #[cfg(test)]
    fn msm_naive<A: Affine>(bases: &[A], scalars: &[Uint<256, 4>]) -> A::Projective {
        use itertools::Itertools;

        bases
            .iter()
            .zip_eq(scalars)
            .map(|(base, scalar)| {
                base.mul_bits(
                    scalar
                        .as_limbs()
                        .iter()
                        .flat_map(|limb| limb.view_bits::<Lsb0>())
                        .map(|bit| *bit.deref())
                        .rev()
                        .collect::<Vec<_>>(),
                )
            })
            .sum()
    }

    #[cfg(test)]
    fn msm_naive_parallel<A: Affine>(bases: &[A], scalars: &[Uint<256, 4>]) -> A::Projective {
        use rayon::prelude::*;

        bases
            .par_iter()
            .zip_eq(scalars)
            .map(|(base, scalar)| {
                base.mul_bits(
                    scalar
                        .as_limbs()
                        .iter()
                        .flat_map(|limb| limb.view_bits::<Lsb0>())
                        .map(|bit| *bit.deref())
                        .rev()
                        .collect::<Vec<_>>(),
                )
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bls12_377::{Field, Fp, G1Affine, Projective, Scalar};
    use rayon::prelude::{IntoParallelIterator, ParallelIterator};
    use ruint::uint;

    fn create_scalar_bases(size: usize) -> (Vec<G1Affine>, Vec<Uint<256, 4>>) {
        let bases = (0..size).map(|_| G1Affine::rand()).collect::<Vec<_>>();
        let scalars = (0..size).map(|_| Scalar::rand().0).collect::<Vec<_>>();
        (bases, scalars)
    }

    #[test]
    fn test_msm() {
        [1, 5, 10, 50, 100, 500, 1000].into_par_iter().for_each(|msm_size| {
            let (bases, scalars) = create_scalar_bases(msm_size);

            let naive_a = VariableBase::msm_naive(bases.as_slice(), scalars.as_slice()).to_affine();
            let naive_b =
                VariableBase::msm_naive_parallel(bases.as_slice(), scalars.as_slice()).to_affine();
            assert_eq!(naive_a, naive_b, "MSM size: {msm_size}");

            let candidate = standard::msm(bases.as_slice(), scalars.as_slice()).to_affine();
            assert_eq!(naive_a, candidate, "MSM size: {msm_size}");

            let candidate = batched::msm(bases.as_slice(), scalars.as_slice()).to_affine();
            assert_eq!(naive_a, candidate, "MSM size: {msm_size}");
        });
    }

    #[cfg(all(feature = "cuda", target_arch = "x86_64"))]
    #[test]
    fn test_msm_cuda() {
        for _ in 0..100 {
            let (bases, scalars) = create_scalar_bases(1 << 10);
            let rust = standard::msm(bases.as_slice(), scalars.as_slice());

            let cuda = cuda::msm_cuda(bases.as_slice(), scalars.as_slice()).unwrap();
            assert_eq!(rust, cuda);
        }
    }
}
