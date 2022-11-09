//! This module contains an `EvaluationDomain` abstraction for
//! performing various kinds of polynomial arithmetic on top of
//! the scalar field.
//!
//! In pairing-based SNARKs like GM17, we need to calculate
//! a quotient polynomial over a target polynomial with roots
//! at distinct points associated with each constraint of the
//! constraint system. In order to be efficient, we choose these
//! roots to be the powers of a 2^n root of unity in the field.
//! This allows us to perform polynomial operations in O(n)
//! by performing an O(n log n) FFT over such a domain.

use crate::{
    bls12_377::{scalar, Field, Scalar},
    fft::SparsePolynomial,
    utils::*,
};
use rand::Rng;
use rayon::prelude::*;
use std::{borrow::Cow, fmt};

#[cfg(not(feature = "parallel"))]
use itertools::Itertools;

/// Returns the ceiling of the base-2 logarithm of `x`.
///
/// ```
/// use snarkvm_algorithms::fft::domain::log2;
///
/// assert_eq!(log2(16), 4);
/// assert_eq!(log2(17), 5);
/// assert_eq!(log2(1), 0);
/// assert_eq!(log2(0), 0);
/// assert_eq!(log2(usize::MAX), (core::mem::size_of::<usize>() * 8) as u32);
/// assert_eq!(log2(1 << 15), 15);
/// assert_eq!(log2(2usize.pow(18)), 18);
/// ```
pub fn log2(x: usize) -> u32 {
    if x == 0 {
        0
    } else if x.is_power_of_two() {
        1usize.leading_zeros() - x.leading_zeros()
    } else {
        0usize.leading_zeros() - x.leading_zeros()
    }
}

// minimum size of a parallelized chunk
#[allow(unused)]
#[cfg(feature = "parallel")]
const MIN_PARALLEL_CHUNK_SIZE: usize = 1 << 7;

/// Defines a domain over which finite field (I)FFTs can be performed. Works
/// only for fields that have a large multiplicative subgroup of size that is
/// a power-of-2.
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct EvaluationDomain {
    /// The size of the domain.
    pub size: u64,
    /// `log_2(self.size)`.
    pub log_size_of_group: u32,
    /// Size of the domain as a field element.
    pub size_as_field_element: Scalar,
    /// Inverse of the size in the field.
    pub size_inv: Scalar,
    /// A generator of the subgroup.
    pub group_gen: Scalar,
    /// Inverse of the generator of the subgroup.
    pub group_gen_inv: Scalar,
    /// Multiplicative generator of the finite field.
    pub generator_inv: Scalar,
}

impl fmt::Debug for EvaluationDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Multiplicative subgroup of size {}", self.size)
    }
}

impl EvaluationDomain {
    /// Sample an element that is *not* in the domain.
    pub fn sample_element_outside_domain<R: Rng>(&self, rng: &mut R) -> Scalar {
        let mut t = Scalar::rand();
        while self.evaluate_vanishing_polynomial(t).is_zero() {
            t = Scalar::rand();
        }
        t
    }

    /// Construct a domain that is large enough for evaluations of a polynomial
    /// having `num_coeffs` coefficients.
    pub fn new(num_coeffs: usize) -> Option<Self> {
        // Compute the size of our evaluation domain
        let size = num_coeffs.checked_next_power_of_two()? as u64;
        let log_size_of_group = size.trailing_zeros();

        // libfqfft uses > https://github.com/scipr-lab/libfqfft/blob/e0183b2cef7d4c5deb21a6eaf3fe3b586d738fe0/libfqfft/evaluation_domain/domains/basic_radix2_domain.tcc#L33
        if log_size_of_group > scalar::TWO_ADICITY {
            return None;
        }

        // Compute the generator for the multiplicative subgroup.
        // It should be the 2^(log_size_of_group) root of unity.
        let group_gen = Scalar::get_root_of_unity(size as usize)?;

        // Check that it is indeed the 2^(log_size_of_group) root of unity.
        debug_assert_eq!(group_gen.pow([size]), Scalar::ONE);

        let size_as_field_element = Scalar::from(size);
        let size_inv = size_as_field_element.inverse()?;

        Some(EvaluationDomain {
            size,
            log_size_of_group,
            size_as_field_element,
            size_inv,
            group_gen,
            group_gen_inv: group_gen.inverse()?,
            generator_inv: Scalar(scalar::GENERATOR).inverse()?,
        })
    }

    /// Return the size of a domain that is large enough for evaluations of a polynomial
    /// having `num_coeffs` coefficients.
    pub fn compute_size_of_domain(num_coeffs: usize) -> Option<usize> {
        let size = num_coeffs.checked_next_power_of_two()?;
        if size.trailing_zeros() <= scalar::TWO_ADICITY {
            Some(size)
        } else {
            None
        }
    }

    /// Return the size of `self`.
    pub fn size(&self) -> usize {
        self.size as usize
    }

    /// Compute an FFT.
    pub fn fft(&self, coeffs: &[Scalar]) -> Vec<Scalar> {
        let mut coeffs = coeffs.to_vec();
        self.fft_in_place(&mut coeffs);
        coeffs
    }

    /// Compute an FFT, modifying the vector in place.
    pub fn fft_in_place(&self, coeffs: &mut Vec<Scalar>) {
        execute_with_max_available_threads(|| {
            coeffs.resize(self.size(), Scalar::ZERO);
            self.in_order_fft_in_place(&mut *coeffs);
        });
    }

    /// Compute an IFFT.
    pub fn ifft(&self, evals: &[Scalar]) -> Vec<Scalar> {
        let mut evals = evals.to_vec();
        self.ifft_in_place(&mut evals);
        evals
    }

    /// Compute an IFFT, modifying the vector in place.
    #[inline]
    pub fn ifft_in_place(&self, evals: &mut Vec<Scalar>) {
        execute_with_max_available_threads(|| {
            evals.resize(self.size(), Scalar::ZERO);
            self.in_order_ifft_in_place(&mut *evals);
        });
    }

    /// Compute an FFT over a coset of the domain.
    pub fn coset_fft(&self, coeffs: &[Scalar]) -> Vec<Scalar> {
        let mut coeffs = coeffs.to_vec();
        self.coset_fft_in_place(&mut coeffs);
        coeffs
    }

    /// Compute an FFT over a coset of the domain, modifying the input vector
    /// in place.
    pub fn coset_fft_in_place(&self, coeffs: &mut Vec<Scalar>) {
        execute_with_max_available_threads(|| {
            Self::distribute_powers(coeffs, Scalar(scalar::GENERATOR));
            self.fft_in_place(coeffs);
        });
    }

    /// Compute an IFFT over a coset of the domain.
    pub fn coset_ifft(&self, evals: &[Scalar]) -> Vec<Scalar> {
        let mut evals = evals.to_vec();
        self.coset_ifft_in_place(&mut evals);
        evals
    }

    /// Compute an IFFT over a coset of the domain, modifying the input vector in place.
    pub fn coset_ifft_in_place(&self, evals: &mut Vec<Scalar>) {
        execute_with_max_available_threads(|| {
            evals.resize(self.size(), Scalar::ZERO);
            self.in_order_coset_ifft_in_place(&mut *evals);
        });
    }

    /// Multiply the `i`-th element of `coeffs` with `g^i`.
    fn distribute_powers(coeffs: &mut [Scalar], g: Scalar) {
        Self::distribute_powers_and_mul_by_const(coeffs, g, Scalar::ONE);
    }

    /// Multiply the `i`-th element of `coeffs` with `c*g^i`.
    #[cfg(not(feature = "parallel"))]
    fn distribute_powers_and_mul_by_const(coeffs: &mut [Scalar], g: Scalar, c: Scalar) {
        // invariant: pow = c*g^i at the ith iteration of the loop
        let mut pow = c;
        coeffs.iter_mut().for_each(|coeff| {
            *coeff *= pow;
            pow *= &g
        })
    }

    /// Multiply the `i`-th element of `coeffs` with `c*g^i`.
    #[cfg(feature = "parallel")]
    fn distribute_powers_and_mul_by_const(coeffs: &mut [Scalar], g: Scalar, c: Scalar) {
        let min_parallel_chunk_size = 1024;
        let num_cpus_available = max_available_threads();
        let num_elem_per_thread =
            core::cmp::max(coeffs.len() / num_cpus_available, min_parallel_chunk_size);

        cfg_chunks_mut!(coeffs, num_elem_per_thread)
            .enumerate()
            .for_each(|(i, chunk)| {
                let offset = c * g.pow([(i * num_elem_per_thread) as u64]);
                let mut pow = offset;
                chunk.iter_mut().for_each(|coeff| {
                    *coeff *= pow;
                    pow *= &g
                })
            });
    }

    /// Evaluate all the lagrange polynomials defined by this domain at the point
    /// `tau`.
    pub fn evaluate_all_lagrange_coefficients(&self, tau: Scalar) -> Vec<Scalar> {
        // Evaluate all Lagrange polynomials
        let size = self.size as usize;
        let t_size = tau.pow(&[self.size]);
        let one = Scalar::ONE;
        if t_size.is_one() {
            let mut u = vec![Scalar::ZERO; size];
            let mut omega_i = one;
            for x in u.iter_mut().take(size) {
                if omega_i == tau {
                    *x = one;
                    break;
                }
                omega_i *= &self.group_gen;
            }
            u
        } else {
            let mut l = (t_size - one) * self.size_inv;
            let mut r = one;
            let mut u = vec![Scalar::ZERO; size];
            let mut ls = vec![Scalar::ZERO; size];
            for i in 0..size {
                u[i] = tau - r;
                ls[i] = l;
                l *= &self.group_gen;
                r *= &self.group_gen;
            }

            Scalar::batch_inversion(u.as_mut_slice());
            cfg_iter_mut!(u).zip_eq(ls).for_each(|(tau_minus_r, l)| {
                *tau_minus_r = l * *tau_minus_r;
            });
            u
        }
    }

    /// Return the sparse vanishing polynomial.
    pub fn vanishing_polynomial(&self) -> SparsePolynomial {
        let coeffs = [(0, -Scalar::ONE), (self.size(), Scalar::ONE)];
        SparsePolynomial::from_coefficients(coeffs)
    }

    /// This evaluates the vanishing polynomial for this domain at tau.
    /// For multiplicative subgroups, this polynomial is `z(X) = X^self.size - 1`.
    pub fn evaluate_vanishing_polynomial(&self, tau: Scalar) -> Scalar {
        tau.pow(&[self.size]) - Scalar::ONE
    }

    /// Return an iterator over the elements of the domain.
    pub fn elements(&self) -> Elements {
        Elements {
            cur_elem: Scalar::ONE,
            cur_pow: 0,
            domain: *self,
        }
    }

    /// The target polynomial is the zero polynomial in our
    /// evaluation domain, so we must perform division over
    /// a coset.
    pub fn divide_by_vanishing_poly_on_coset_in_place(&self, evals: &mut [Scalar]) {
        let i = self
            .evaluate_vanishing_polynomial(Scalar(scalar::GENERATOR))
            .inverse()
            .unwrap();

        cfg_iter_mut!(evals).for_each(|eval| *eval *= &i);
    }

    /// Given an index which assumes the first elements of this domain are the elements of
    /// another (sub)domain with size size_s,
    /// this returns the actual index into this domain.
    pub fn reindex_by_subdomain(&self, other: &Self, index: usize) -> usize {
        assert!(self.size() >= other.size());
        // Let this subgroup be G, and the subgroup we're re-indexing by be S.
        // Since its a subgroup, the 0th element of S is at index 0 in G, the first element of S is at
        // index |G|/|S|, the second at 2*|G|/|S|, etc.
        // Thus for an index i that corresponds to S, the index in G is i*|G|/|S|
        let period = self.size() / other.size();
        if index < other.size() {
            index * period
        } else {
            // Let i now be the index of this element in G \ S
            // Let x be the number of elements in G \ S, for every element in S. Then x = (|G|/|S| - 1).
            // At index i in G \ S, the number of elements in S that appear before the index in G to which
            // i corresponds to, is floor(i / x) + 1.
            // The +1 is because index 0 of G is S_0, so the position is offset by at least one.
            // The floor(i / x) term is because after x elements in G \ S, there is one more element from S
            // that will have appeared in G.
            let i = index - other.size();
            let x = period - 1;
            i + (i / x) + 1
        }
    }

    /// Perform O(n) multiplication of two polynomials that are presented by their
    /// evaluations in the domain.
    /// Returns the evaluations of the product over the domain.
    #[must_use]
    pub fn mul_polynomials_in_evaluation_domain(
        &self,
        self_evals: &[Scalar],
        other_evals: &[Scalar],
    ) -> Vec<Scalar> {
        let mut result = self_evals.to_vec();

        cfg_iter_mut!(result)
            .zip_eq(other_evals)
            .for_each(|(a, b)| *a *= b);

        result
    }
}

impl EvaluationDomain {
    pub fn precompute_fft(&self) -> FFTPrecomputation {
        execute_with_max_available_threads(|| FFTPrecomputation {
            roots: self.roots_of_unity(self.group_gen),
            domain: *self,
        })
    }

    pub fn precompute_ifft(&self) -> IFFTPrecomputation {
        execute_with_max_available_threads(|| IFFTPrecomputation {
            inverse_roots: self.roots_of_unity(self.group_gen_inv),
            domain: *self,
        })
    }

    pub(crate) fn in_order_fft_in_place(&self, x_s: &mut [Scalar]) {
        let pc = self.precompute_fft();
        self.fft_helper_in_place_with_pc(x_s, FFTOrder::II, &pc)
    }

    pub(crate) fn in_order_ifft_in_place(&self, x_s: &mut [Scalar]) {
        let pc = self.precompute_ifft();
        self.ifft_helper_in_place_with_pc(x_s, FFTOrder::II, &pc);
        cfg_iter_mut!(x_s).for_each(|val| *val *= self.size_inv);
    }

    pub(crate) fn in_order_coset_ifft_in_place(&self, x_s: &mut [Scalar]) {
        let pc = self.precompute_ifft();
        self.ifft_helper_in_place_with_pc(x_s, FFTOrder::II, &pc);
        let coset_shift = self.generator_inv;
        Self::distribute_powers_and_mul_by_const(x_s, coset_shift, self.size_inv);
    }

    #[allow(unused)]
    pub(crate) fn in_order_fft_in_place_with_pc(
        &self,
        x_s: &mut [Scalar],
        pre_comp: &FFTPrecomputation,
    ) {
        self.fft_helper_in_place_with_pc(x_s, FFTOrder::II, pre_comp)
    }

    pub(crate) fn out_order_fft_in_place_with_pc(
        &self,
        x_s: &mut [Scalar],
        pre_comp: &FFTPrecomputation,
    ) {
        self.fft_helper_in_place_with_pc(x_s, FFTOrder::IO, pre_comp)
    }

    pub(crate) fn in_order_ifft_in_place_with_pc(
        &self,
        x_s: &mut [Scalar],
        pre_comp: &IFFTPrecomputation,
    ) {
        self.ifft_helper_in_place_with_pc(x_s, FFTOrder::II, pre_comp);
        cfg_iter_mut!(x_s).for_each(|val| *val *= self.size_inv);
    }

    pub(crate) fn out_order_ifft_in_place_with_pc(
        &self,
        x_s: &mut [Scalar],
        pre_comp: &IFFTPrecomputation,
    ) {
        self.ifft_helper_in_place_with_pc(x_s, FFTOrder::OI, pre_comp);
        cfg_iter_mut!(x_s).for_each(|val| *val *= self.size_inv);
    }

    #[allow(unused)]
    pub(crate) fn in_order_coset_ifft_in_place_with_pc(
        &self,
        x_s: &mut [Scalar],
        pre_comp: &IFFTPrecomputation,
    ) {
        self.ifft_helper_in_place_with_pc(x_s, FFTOrder::II, pre_comp);
        let coset_shift = self.generator_inv;
        Self::distribute_powers_and_mul_by_const(x_s, coset_shift, self.size_inv);
    }

    fn fft_helper_in_place_with_pc(
        &self,
        x_s: &mut [Scalar],
        ord: FFTOrder,
        pre_comp: &FFTPrecomputation,
    ) {
        use FFTOrder::*;
        let pc = pre_comp.precomputation_for_subdomain(self).unwrap();

        let log_len = log2(x_s.len());

        if ord == OI {
            self.oi_helper_with_roots(x_s, &pc.roots);
        } else {
            self.io_helper_with_roots(x_s, &pc.roots);
        }

        if ord == II {
            derange_helper(x_s, log_len);
        }
    }

    // Handles doing an IFFT with handling of being in order and out of order.
    // The results here must all be divided by |x_s|,
    // which is left up to the caller to do.
    fn ifft_helper_in_place_with_pc(
        &self,
        x_s: &mut [Scalar],
        ord: FFTOrder,
        pre_comp: &IFFTPrecomputation,
    ) {
        use FFTOrder::*;
        let pc = pre_comp.precomputation_for_subdomain(self).unwrap();

        let log_len = log2(x_s.len());

        if ord == II {
            derange_helper(x_s, log_len);
        }

        if ord == IO {
            self.io_helper_with_roots(x_s, &pc.inverse_roots);
        } else {
            self.oi_helper_with_roots(x_s, &pc.inverse_roots);
        }
    }

    /// Computes the first `self.size / 2` roots of unity for the entire domain.
    /// e.g. for the domain [1, g, g^2, ..., g^{n - 1}], it computes
    // [1, g, g^2, ..., g^{(n/2) - 1}]
    #[cfg(not(feature = "parallel"))]
    pub fn roots_of_unity(&self, root: Scalar) -> Vec<Scalar> {
        compute_powers_serial((self.size as usize) / 2, root)
    }

    /// Computes the first `self.size / 2` roots of unity.
    #[cfg(feature = "parallel")]
    pub fn roots_of_unity(&self, root: Scalar) -> Vec<Scalar> {
        // TODO: check if this method can replace parallel compute powers.
        let log_size = log2(self.size as usize);
        // early exit for short inputs
        if log_size <= LOG_ROOTS_OF_UNITY_PARALLEL_SIZE {
            compute_powers_serial((self.size as usize) / 2, root)
        } else {
            let mut temp = root;
            // w, w^2, w^4, w^8, ..., w^(2^(log_size - 1))
            let log_powers: Vec<Scalar> = (0..(log_size - 1))
                .map(|_| {
                    let old_value = temp;
                    temp.square_in_place();
                    old_value
                })
                .collect();

            // allocate the return array and start the recursion
            let mut powers = vec![Scalar::ZERO; 1 << (log_size - 1)];
            Self::roots_of_unity_recursive(&mut powers, &log_powers);
            powers
        }
    }

    #[cfg(feature = "parallel")]
    fn roots_of_unity_recursive(out: &mut [Scalar], log_powers: &[Scalar]) {
        assert_eq!(out.len(), 1 << log_powers.len());
        // base case: just compute the powers sequentially,
        // g = log_powers[0], out = [1, g, g^2, ...]
        if log_powers.len() <= LOG_ROOTS_OF_UNITY_PARALLEL_SIZE as usize {
            out[0] = Scalar::ONE;
            for idx in 1..out.len() {
                out[idx] = out[idx - 1] * log_powers[0];
            }
            return;
        }

        // recursive case:
        // 1. split log_powers in half
        let (lr_lo, lr_hi) = log_powers.split_at((1 + log_powers.len()) / 2);
        let mut scr_lo = vec![Scalar::default(); 1 << lr_lo.len()];
        let mut scr_hi = vec![Scalar::default(); 1 << lr_hi.len()];
        // 2. compute each half individually
        rayon::join(
            || Self::roots_of_unity_recursive(&mut scr_lo, lr_lo),
            || Self::roots_of_unity_recursive(&mut scr_hi, lr_hi),
        );
        // 3. recombine halves
        // At this point, out is a blank slice.
        out.par_chunks_mut(scr_lo.len())
            .zip(&scr_hi)
            .for_each(|(out_chunk, scr_hi)| {
                for (out_elem, scr_lo) in out_chunk.iter_mut().zip(&scr_lo) {
                    *out_elem = *scr_hi * scr_lo;
                }
            });
    }

    #[inline(always)]
    fn butterfly_fn_io(((lo, hi), root): ((&mut Scalar, &mut Scalar), &Scalar)) {
        let neg = *lo - *hi;
        *lo += *hi;
        *hi = neg;
        *hi *= *root;
    }

    #[inline(always)]
    fn butterfly_fn_oi(((lo, hi), root): ((&mut Scalar, &mut Scalar), &Scalar)) {
        *hi *= *root;
        let neg = *lo - *hi;
        *lo += *hi;
        *hi = neg;
    }

    #[allow(clippy::too_many_arguments)]
    fn apply_butterfly<G: Fn(((&mut Scalar, &mut Scalar), &Scalar)) + Copy + Sync + Send>(
        g: G,
        xi: &mut [Scalar],
        roots: &[Scalar],
        step: usize,
        chunk_size: usize,
        num_chunks: usize,
        max_threads: usize,
        gap: usize,
    ) {
        cfg_chunks_mut!(xi, chunk_size).for_each(|cxi| {
            let (lo, hi) = cxi.split_at_mut(gap);
            // If the chunk is sufficiently big that parallelism helps,
            // we parallelize the butterfly operation within the chunk.

            if gap > MIN_GAP_SIZE_FOR_PARALLELISATION && num_chunks < max_threads {
                cfg_iter_mut!(lo)
                    .zip(hi)
                    .zip(cfg_iter!(roots).step_by(step))
                    .for_each(g);
            } else {
                lo.iter_mut()
                    .zip(hi)
                    .zip(roots.iter().step_by(step))
                    .for_each(g);
            }
        });
    }

    fn io_helper_with_roots(&self, xi: &mut [Scalar], roots: &[Scalar]) {
        let mut roots = std::borrow::Cow::Borrowed(roots);

        let mut step = 1;
        let mut first = true;

        #[cfg(feature = "parallel")]
        let max_threads = snarkvm_utilities::parallel::max_available_threads();
        #[cfg(not(feature = "parallel"))]
        let max_threads = 1;

        let mut gap = xi.len() / 2;
        while gap > 0 {
            // each butterfly cluster uses 2*gap positions
            let chunk_size = 2 * gap;
            let num_chunks = xi.len() / chunk_size;

            // Only compact roots to achieve cache locality/compactness if
            // the roots lookup is done a significant amount of times
            // Which also implies a large lookup stride.
            if num_chunks >= MIN_NUM_CHUNKS_FOR_COMPACTION {
                if !first {
                    roots = Cow::Owned(
                        cfg_into_iter!(roots.into_owned())
                            .step_by(step * 2)
                            .collect(),
                    );
                }
                step = 1;
                roots.to_mut().shrink_to_fit();
            } else {
                step = num_chunks;
            }
            first = false;

            Self::apply_butterfly(
                Self::butterfly_fn_io,
                xi,
                &roots[..],
                step,
                chunk_size,
                num_chunks,
                max_threads,
                gap,
            );

            gap /= 2;
        }
    }

    fn oi_helper_with_roots(&self, xi: &mut [Scalar], roots_cache: &[Scalar]) {
        // The `cmp::min` is only necessary for the case where
        // `MIN_NUM_CHUNKS_FOR_COMPACTION = 1`. Else, notice that we compact
        // the roots cache by a stride of at least `MIN_NUM_CHUNKS_FOR_COMPACTION`.

        let compaction_max_size = core::cmp::min(
            roots_cache.len() / 2,
            roots_cache.len() / MIN_NUM_CHUNKS_FOR_COMPACTION,
        );
        let mut compacted_roots = vec![Scalar::ZERO; compaction_max_size];

        #[cfg(feature = "parallel")]
        let max_threads = snarkvm_utilities::parallel::max_available_threads();
        #[cfg(not(feature = "parallel"))]
        let max_threads = 1;

        let mut gap = 1;
        while gap < xi.len() {
            // each butterfly cluster uses 2*gap positions
            let chunk_size = 2 * gap;
            let num_chunks = xi.len() / chunk_size;

            // Only compact roots to achieve cache locality/compactness if
            // the roots lookup is done a significant amount of times
            // Which also implies a large lookup stride.
            let (roots, step) = if num_chunks >= MIN_NUM_CHUNKS_FOR_COMPACTION && gap < xi.len() / 2
            {
                cfg_iter_mut!(compacted_roots[..gap])
                    .zip(cfg_iter!(roots_cache[..(gap * num_chunks)]).step_by(num_chunks))
                    .for_each(|(a, b)| *a = *b);
                (&compacted_roots[..gap], 1)
            } else {
                (roots_cache, num_chunks)
            };

            Self::apply_butterfly(
                Self::butterfly_fn_oi,
                xi,
                roots,
                step,
                chunk_size,
                num_chunks,
                max_threads,
                gap,
            );

            gap *= 2;
        }
    }
}

/// The minimum number of chunks at which root compaction
/// is beneficial.
const MIN_NUM_CHUNKS_FOR_COMPACTION: usize = 1 << 7;

/// The minimum size of a chunk at which parallelization of `butterfly`s is
/// beneficial. This value was chosen empirically.
const MIN_GAP_SIZE_FOR_PARALLELISATION: usize = 1 << 10;

// minimum size at which to parallelize.
#[cfg(feature = "parallel")]
const LOG_ROOTS_OF_UNITY_PARALLEL_SIZE: u32 = 7;

#[inline]
pub(super) fn bitrev(a: u64, log_len: u32) -> u64 {
    a.reverse_bits() >> (64 - log_len)
}

pub(crate) fn derange<T>(xi: &mut [T]) {
    derange_helper(xi, log2(xi.len()))
}

fn derange_helper<T>(xi: &mut [T], log_len: u32) {
    for idx in 1..(xi.len() as u64 - 1) {
        let ridx = bitrev(idx, log_len);
        if idx < ridx {
            xi.swap(idx as usize, ridx as usize);
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum FFTOrder {
    /// Both the input and the output of the FFT must be in-order.
    II,
    /// The input of the FFT must be in-order, but the output does not have to
    /// be.
    IO,
    /// The input of the FFT can be out of order, but the output must be
    /// in-order.
    OI,
}

pub(crate) fn compute_powers_serial(size: usize, root: Scalar) -> Vec<Scalar> {
    compute_powers_and_mul_by_const_serial(size, root, Scalar::ONE)
}

pub(crate) fn compute_powers_and_mul_by_const_serial(
    size: usize,
    root: Scalar,
    c: Scalar,
) -> Vec<Scalar> {
    let mut value = c;
    (0..size)
        .map(|_| {
            let old_value = value;
            value *= root;
            old_value
        })
        .collect()
}

#[allow(unused)]
#[cfg(feature = "parallel")]
pub(crate) fn compute_powers(size: usize, g: Scalar) -> Vec<Scalar> {
    if size < MIN_PARALLEL_CHUNK_SIZE {
        return compute_powers_serial(size, g);
    }
    // compute the number of threads we will be using.
    let num_cpus_available = max_available_threads();
    let num_elem_per_thread = core::cmp::max(size / num_cpus_available, MIN_PARALLEL_CHUNK_SIZE);
    let num_cpus_used = size / num_elem_per_thread;

    // Split up the powers to compute across each thread evenly.
    let res: Vec<Scalar> = (0..num_cpus_used)
        .into_par_iter()
        .flat_map(|i| {
            let offset = g.pow([(i * num_elem_per_thread) as u64]);
            // Compute the size that this chunks' output should be
            // (num_elem_per_thread, unless there are less than num_elem_per_thread elements remaining)
            let num_elements_to_compute =
                core::cmp::min(size - i * num_elem_per_thread, num_elem_per_thread);
            compute_powers_and_mul_by_const_serial(num_elements_to_compute, g, offset)
        })
        .collect();
    res
}

/// An iterator over the elements of the domain.
#[derive(Clone)]
pub struct Elements {
    cur_elem: Scalar,
    cur_pow: u64,
    domain: EvaluationDomain,
}

impl Iterator for Elements {
    type Item = Scalar;

    fn next(&mut self) -> Option<Scalar> {
        if self.cur_pow == self.domain.size {
            None
        } else {
            let cur_elem = self.cur_elem;
            self.cur_elem *= &self.domain.group_gen;
            self.cur_pow += 1;
            Some(cur_elem)
        }
    }
}

/// An iterator over the elements of the domain.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct FFTPrecomputation {
    roots: Vec<Scalar>,
    domain: EvaluationDomain,
}

impl FFTPrecomputation {
    pub fn to_ifft_precomputation(&self) -> IFFTPrecomputation {
        let mut inverse_roots = self.roots.clone();
        Scalar::batch_inversion(&mut inverse_roots);
        IFFTPrecomputation {
            inverse_roots,
            domain: self.domain,
        }
    }

    pub fn precomputation_for_subdomain<'a>(
        &'a self,
        domain: &EvaluationDomain,
    ) -> Option<Cow<'a, Self>> {
        if domain.size() == 1 {
            return Some(Cow::Owned(Self {
                roots: vec![],
                domain: *domain,
            }));
        }
        if &self.domain == domain {
            Some(Cow::Borrowed(self))
        } else if domain.size() < self.domain.size() {
            let size_ratio = self.domain.size() / domain.size();
            let roots = self.roots.iter().step_by(size_ratio).copied().collect();
            Some(Cow::Owned(Self {
                roots,
                domain: *domain,
            }))
        } else {
            None
        }
    }
}

/// An iterator over the elements of the domain.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IFFTPrecomputation {
    inverse_roots: Vec<Scalar>,
    domain: EvaluationDomain,
}

impl IFFTPrecomputation {
    pub fn precomputation_for_subdomain<'a>(
        &'a self,
        domain: &EvaluationDomain,
    ) -> Option<Cow<'a, Self>> {
        if domain.size() == 1 {
            return Some(Cow::Owned(Self {
                inverse_roots: vec![],
                domain: *domain,
            }));
        }
        if &self.domain == domain {
            Some(Cow::Borrowed(self))
        } else if domain.size() < self.domain.size() {
            let size_ratio = self.domain.size() / domain.size();
            let inverse_roots = self
                .inverse_roots
                .iter()
                .step_by(size_ratio)
                .copied()
                .collect();
            Some(Cow::Owned(Self {
                inverse_roots,
                domain: *domain,
            }))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fft::{DensePolynomial, EvaluationDomain};
    use rand::Rng;
    use snarkvm_curves::bls12_377::Scalar;
    use snarkvm_fields::{FftField, Field, One, Zero};
    use snarkvm_utilities::{TestRng, Uniform};

    #[test]
    fn vanishing_polynomial_evaluation() {
        let rng = &mut TestRng::default();
        for coeffs in 0..10 {
            let domain = EvaluationDomain::<Scalar>::new(coeffs).unwrap();
            let z = domain.vanishing_polynomial();
            for _ in 0..100 {
                let point = rng.gen();
                assert_eq!(
                    z.evaluate(point),
                    domain.evaluate_vanishing_polynomial(point)
                )
            }
        }
    }

    #[test]
    fn vanishing_polynomial_vanishes_on_domain() {
        for coeffs in 0..1000 {
            let domain = EvaluationDomain::<Scalar>::new(coeffs).unwrap();
            let z = domain.vanishing_polynomial();
            for point in domain.elements() {
                assert!(z.evaluate(point).is_zero())
            }
        }
    }

    #[test]
    fn size_of_elements() {
        for coeffs in 1..10 {
            let size = 1 << coeffs;
            let domain = EvaluationDomain::<Scalar>::new(size).unwrap();
            let domain_size = domain.size();
            assert_eq!(domain_size, domain.elements().count());
        }
    }

    #[test]
    fn elements_contents() {
        for coeffs in 1..10 {
            let size = 1 << coeffs;
            let domain = EvaluationDomain::<Scalar>::new(size).unwrap();
            for (i, element) in domain.elements().enumerate() {
                assert_eq!(element, domain.group_gen.pow([i as u64]));
            }
        }
    }

    /// Test that lagrange interpolation for a random polynomial at a random point works.
    #[test]
    fn non_systematic_lagrange_coefficients_test() {
        let mut rng = TestRng::default();
        for domain_dimension in 1..10 {
            let domain_size = 1 << domain_dimension;
            let domain = EvaluationDomain::<Scalar>::new(domain_size).unwrap();
            // Get random point & lagrange coefficients
            let random_point = Scalar::rand();
            let lagrange_coefficients = domain.evaluate_all_lagrange_coefficients(random_point);

            // Sample the random polynomial, evaluate it over the domain and the random point.
            let random_polynomial = DensePolynomial::<Scalar>::rand(domain_size - 1, &mut rng);
            let polynomial_evaluations = domain.fft(random_polynomial.coeffs());
            let actual_evaluations = random_polynomial.evaluate(random_point);

            // Do lagrange interpolation, and compare against the actual evaluation
            let mut interpolated_evaluation = Scalar::ZERO;
            for i in 0..domain_size {
                interpolated_evaluation += lagrange_coefficients[i] * polynomial_evaluations[i];
            }
            assert_eq!(actual_evaluations, interpolated_evaluation);
        }
    }

    /// Test that lagrange coefficients for a point in the domain is correct.
    #[test]
    fn systematic_lagrange_coefficients_test() {
        // This runs in time O(N^2) in the domain size, so keep the domain dimension low.
        // We generate lagrange coefficients for each element in the domain.
        for domain_dimension in 1..5 {
            let domain_size = 1 << domain_dimension;
            let domain = EvaluationDomain::<Scalar>::new(domain_size).unwrap();
            let all_domain_elements: Vec<Scalar> = domain.elements().collect();
            for (i, domain_element) in all_domain_elements.iter().enumerate().take(domain_size) {
                let lagrange_coefficients =
                    domain.evaluate_all_lagrange_coefficients(*domain_element);
                for (j, lagrange_coefficient) in
                    lagrange_coefficients.iter().enumerate().take(domain_size)
                {
                    // Lagrange coefficient for the evaluation point, which should be 1
                    if i == j {
                        assert_eq!(*lagrange_coefficient, Scalar::ONE);
                    } else {
                        assert_eq!(*lagrange_coefficient, Scalar::ZERO);
                    }
                }
            }
        }
    }

    /// Tests that the roots of unity result is the same as domain.elements().
    #[test]
    fn test_roots_of_unity() {
        let max_degree = 10;
        for log_domain_size in 0..max_degree {
            let domain_size = 1 << log_domain_size;
            let domain = EvaluationDomain::<Scalar>::new(domain_size).unwrap();
            let actual_roots = domain.roots_of_unity(domain.group_gen);
            for &value in &actual_roots {
                assert!(domain.evaluate_vanishing_polynomial(value).is_zero());
            }
            let expected_roots_elements = domain.elements();
            for (expected, &actual) in expected_roots_elements.zip(&actual_roots) {
                assert_eq!(expected, actual);
            }
            assert_eq!(actual_roots.len(), domain_size / 2);
        }
    }

    /// Tests that the FFTs output the correct result.
    #[test]
    fn test_fft_correctness() {
        // This assumes a correct polynomial evaluation at point procedure.
        // It tests consistency of FFT/IFFT, and coset_fft/coset_ifft,
        // along with testing that each individual evaluation is correct.

        let mut rng = TestRng::default();

        // Runs in time O(degree^2)
        let log_degree = 5;
        let degree = 1 << log_degree;
        let random_polynomial = DensePolynomial::<Scalar>::rand(degree - 1, &mut rng);

        for log_domain_size in log_degree..(log_degree + 2) {
            let domain_size = 1 << log_domain_size;
            let domain = EvaluationDomain::<Scalar>::new(domain_size).unwrap();
            let polynomial_evaluations = domain.fft(&random_polynomial.coeffs);
            let polynomial_coset_evaluations = domain.coset_fft(&random_polynomial.coeffs);
            for (i, x) in domain.elements().enumerate() {
                let coset_x = Scalar(scalar::GENERATOR) * x;

                assert_eq!(polynomial_evaluations[i], random_polynomial.evaluate(x));
                assert_eq!(
                    polynomial_coset_evaluations[i],
                    random_polynomial.evaluate(coset_x)
                );
            }

            let randon_polynomial_from_subgroup =
                DensePolynomial::from_coefficients_vec(domain.ifft(&polynomial_evaluations));
            let random_polynomial_from_coset = DensePolynomial::from_coefficients_vec(
                domain.coset_ifft(&polynomial_coset_evaluations),
            );

            assert_eq!(
                random_polynomial, randon_polynomial_from_subgroup,
                "degree = {}, domain size = {}",
                degree, domain_size
            );
            assert_eq!(
                random_polynomial, random_polynomial_from_coset,
                "degree = {}, domain size = {}",
                degree, domain_size
            );
        }
    }

    /// Tests that FFT precomputation is correctly subdomained
    #[test]
    fn test_fft_precomputation() {
        for i in 1..10 {
            let big_domain = EvaluationDomain::<Scalar>::new(i).unwrap();
            let pc = big_domain.precompute_fft();
            for j in 1..i {
                let small_domain = EvaluationDomain::<Scalar>::new(j).unwrap();
                let small_pc = small_domain.precompute_fft();
                assert_eq!(
                    pc.precomputation_for_subdomain(&small_domain)
                        .unwrap()
                        .as_ref(),
                    &small_pc
                );
            }
        }
    }

    /// Tests that IFFT precomputation is correctly subdomained
    #[test]
    fn test_ifft_precomputation() {
        for i in 1..10 {
            let big_domain = EvaluationDomain::<Scalar>::new(i).unwrap();
            let pc = big_domain.precompute_ifft();
            for j in 1..i {
                let small_domain = EvaluationDomain::<Scalar>::new(j).unwrap();
                let small_pc = small_domain.precompute_ifft();
                assert_eq!(
                    pc.precomputation_for_subdomain(&small_domain)
                        .unwrap()
                        .as_ref(),
                    &small_pc
                );
            }
        }
    }

    /// Tests that IFFT precomputation can be correctly computed from
    /// FFT precomputation
    #[test]
    fn test_ifft_precomputation_from_fft() {
        for i in 1..10 {
            let domain = EvaluationDomain::<Scalar>::new(i).unwrap();
            let pc = domain.precompute_ifft();
            let fft_pc = domain.precompute_fft();
            assert_eq!(pc, fft_pc.to_ifft_precomputation())
        }
    }
}
