mod batched;
mod standard;

#[cfg(all(feature = "cuda", target_arch = "x86_64"))]
mod cuda;

#[cfg(target_arch = "x86_64")]
pub mod prefetch;

use snarkvm_curves::{bls12_377::G1Affine, traits::AffineCurve};
use snarkvm_fields::PrimeField;

use core::any::TypeId;

#[cfg(all(feature = "cuda", target_arch = "x86_64"))]
use core::sync::atomic::{AtomicBool, Ordering};

#[cfg(all(feature = "cuda", target_arch = "x86_64"))]
static HAS_CUDA_FAILED: AtomicBool = AtomicBool::new(false);

pub struct VariableBase;

impl VariableBase {
    pub fn msm<G: AffineCurve>(
        bases: &[G],
        scalars: &[<G::ScalarField as PrimeField>::BigInteger],
    ) -> G::Projective {
        // For BLS12-377, we perform variable base MSM using a batched addition technique.
        if TypeId::of::<G>() == TypeId::of::<G1Affine>() {
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
    fn msm_naive<G: AffineCurve>(
        bases: &[G],
        scalars: &[<G::ScalarField as PrimeField>::BigInteger],
    ) -> G::Projective {
        use itertools::Itertools;
        use snarkvm_utilities::BitIteratorBE;

        bases
            .iter()
            .zip_eq(scalars)
            .map(|(base, scalar)| base.mul_bits(BitIteratorBE::new(*scalar)))
            .sum()
    }

    #[cfg(test)]
    fn msm_naive_parallel<G: AffineCurve>(
        bases: &[G],
        scalars: &[<G::ScalarField as PrimeField>::BigInteger],
    ) -> G::Projective {
        use rayon::prelude::*;
        use snarkvm_utilities::BitIteratorBE;

        bases
            .par_iter()
            .zip_eq(scalars)
            .map(|(base, scalar)| base.mul_bits(BitIteratorBE::new(*scalar)))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_curves::bls12_377::{Fr, G1Affine};
    use snarkvm_fields::PrimeField;
    use snarkvm_utilities::rand::TestRng;

    fn create_scalar_bases<G: AffineCurve<ScalarField = F>, F: PrimeField>(
        rng: &mut TestRng,
        size: usize,
    ) -> (Vec<G>, Vec<F::BigInteger>) {
        let bases = (0..size).map(|_| G::rand(rng)).collect::<Vec<_>>();
        let scalars = (0..size)
            .map(|_| F::rand(rng).to_repr())
            .collect::<Vec<_>>();
        (bases, scalars)
    }

    #[test]
    fn test_msm() {
        use snarkvm_curves::ProjectiveCurve;
        for msm_size in [1, 5, 10, 50, 100, 500, 1000] {
            let mut rng = TestRng::default();
            let (bases, scalars) = create_scalar_bases::<G1Affine, Fr>(&mut rng, msm_size);

            let naive_a = VariableBase::msm_naive(bases.as_slice(), scalars.as_slice()).to_affine();
            let naive_b =
                VariableBase::msm_naive_parallel(bases.as_slice(), scalars.as_slice()).to_affine();
            assert_eq!(naive_a, naive_b, "MSM size: {msm_size}");

            let candidate = standard::msm(bases.as_slice(), scalars.as_slice()).to_affine();
            assert_eq!(naive_a, candidate, "MSM size: {msm_size}");

            let candidate = batched::msm(bases.as_slice(), scalars.as_slice()).to_affine();
            assert_eq!(naive_a, candidate, "MSM size: {msm_size}");
        }
    }

    #[cfg(all(feature = "cuda", target_arch = "x86_64"))]
    #[test]
    fn test_msm_cuda() {
        let mut rng = TestRng::default();
        for _ in 0..100 {
            let (bases, scalars) = create_scalar_bases::<G1Affine, Fr>(&mut rng, 1 << 10);
            let rust = standard::msm(bases.as_slice(), scalars.as_slice());

            let cuda = cuda::msm_cuda(bases.as_slice(), scalars.as_slice()).unwrap();
            assert_eq!(rust, cuda);
        }
    }
}
