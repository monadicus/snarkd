//! Here we construct a polynomial commitment that enables users to commit to a
//! single polynomial `p`, and then later provide an evaluation proof that
//! convinces verifiers that a claimed value `v` is the true evaluation of `p`
//! at a chosen point `x`. Our construction follows the template of the construction
//! proposed by Kate, Zaverucha, and Goldberg ([KZG11](http://cacr.uwaterloo.ca/techreports/2010/cacr2010-10.pdf)).
//! This construction achieves extractability in the algebraic group model (AGM).

use crate::{
    bls12_377::{
        pairing, product_of_pairings, Affine, Field, G1Affine, G1Prepared, G1Projective,
        Projective, Scalar,
    },
    fft::{DensePolynomial, Polynomial},
    msm::VariableBase,
    polycommit::PCError,
    utils::*,
};
use anyhow::anyhow;
use bitvec::prelude::*;
use core::{
    marker::PhantomData,
    ops::Mul,
    sync::atomic::{AtomicBool, Ordering},
};
use itertools::Itertools;
use rand::Rng;
use rand_core::RngCore;
use rayon::prelude::*;
use ruint::Uint;

mod data_structures;
pub use data_structures::*;

use super::sonic_pc::LabeledPolynomialWithBasis;

#[derive(Debug, PartialEq, Eq)]
pub enum KZGDegreeBounds {
    All,
    Marlin,
    List(Vec<usize>),
    None,
}

impl KZGDegreeBounds {
    pub fn get_list(&self, max_degree: usize) -> Vec<usize> {
        match self {
            KZGDegreeBounds::All => (0..max_degree).collect(),
            KZGDegreeBounds::Marlin => {
                // In Marlin, the degree bounds are all of the forms `domain_size - 2`.
                // Consider that we are using radix-2 FFT,
                // there are only a few possible domain sizes and therefore degree bounds.
                //
                // We do not consider mixed-radix FFT for simplicity, as the curves that we
                // are using have very high two-arity.

                let mut radix_2_possible_domain_sizes = vec![];

                let mut cur = 2usize;
                while cur - 2 <= max_degree {
                    radix_2_possible_domain_sizes.push(cur - 2);
                    cur *= 2;
                }

                radix_2_possible_domain_sizes
            }
            KZGDegreeBounds::List(v) => v.clone(),
            KZGDegreeBounds::None => vec![],
        }
    }
}

/// `KZG10` is an implementation of the polynomial commitment scheme of
/// [Kate, Zaverucha and Goldbgerg][kzg10]
///
/// [kzg10]: http://cacr.uwaterloo.ca/techreports/2010/cacr2010-10.pdf
#[derive(Clone, Debug)]
pub struct KZG10;

impl KZG10 {
    /// Constructs public parameters when given as input the maximum degree `degree`
    /// for the polynomial commitment scheme.
    pub fn load_srs(max_degree: usize) -> Result<UniversalParams, PCError> {
        let params = UniversalParams::load()?;
        params.download_powers_for(0..(max_degree + 1))?;
        Ok(params)
    }

    /// Outputs a commitment to `polynomial`.
    pub fn commit(
        powers: &Powers,
        polynomial: &Polynomial<'_>,
        hiding_bound: Option<usize>,
        terminator: &AtomicBool,
        rng: Option<&mut dyn RngCore>,
    ) -> Result<(KZGCommitment, KZGRandomness), PCError> {
        Self::check_degree_is_too_large(polynomial.degree(), powers.size())?;

        let mut commitment = match polynomial {
            Polynomial::Dense(polynomial) => {
                let (num_leading_zeros, plain_coeffs) =
                    skip_leading_zeros_and_convert_to_bigints(polynomial);

                let commitment =
                    VariableBase::msm(&powers.powers_of_beta_g[num_leading_zeros..], &plain_coeffs);

                if terminator.load(Ordering::Relaxed) {
                    return Err(PCError::Terminated);
                }
                commitment
            }
            Polynomial::Sparse(polynomial) => polynomial
                .coeffs()
                .map(|(i, coeff)| {
                    powers.powers_of_beta_g[*i].mul_bits(
                        coeff
                            .0
                            .as_limbs()
                            .iter()
                            .flat_map(|limb| limb.view_bits::<Lsb0>())
                            .map(|b| *b)
                            .rev()
                            .collect::<Vec<_>>(),
                    )
                })
                .sum(),
        };

        let mut randomness = KZGRandomness::empty();
        if let Some(hiding_degree) = hiding_bound {
            let mut rng = rng.ok_or(PCError::MissingRng)?;

            randomness = KZGRandomness::rand(hiding_degree, false, &mut rng);
            Self::check_hiding_bound(
                randomness.blinding_polynomial.degree(),
                powers.powers_of_beta_times_gamma_g.len(),
            )?;
        }

        let random_ints = convert_to_bigints(&randomness.blinding_polynomial.coeffs);
        let random_commitment =
            VariableBase::msm(&powers.powers_of_beta_times_gamma_g, random_ints.as_slice())
                .to_affine();

        if terminator.load(Ordering::Relaxed) {
            return Err(PCError::Terminated);
        }

        commitment.add_assign_mixed(&random_commitment);

        Ok((KZGCommitment(commitment.into()), randomness))
    }

    /// Outputs a commitment to `polynomial`.
    pub fn commit_lagrange(
        lagrange_basis: &LagrangeBasis,
        evaluations: &[Scalar],
        hiding_bound: Option<usize>,
        terminator: &AtomicBool,
        rng: Option<&mut dyn RngCore>,
    ) -> Result<(KZGCommitment, KZGRandomness), PCError> {
        Self::check_degree_is_too_large(evaluations.len() - 1, lagrange_basis.size())?;
        assert_eq!(
            evaluations
                .len()
                .checked_next_power_of_two()
                .ok_or(PCError::LagrangeBasisSizeIsTooLarge)?,
            lagrange_basis.size()
        );

        let evaluations = evaluations.iter().map(|e| e.0).collect::<Vec<_>>();
        let mut commitment =
            VariableBase::msm(&lagrange_basis.lagrange_basis_at_beta_g, &evaluations);

        if terminator.load(Ordering::Relaxed) {
            return Err(PCError::Terminated);
        }

        let mut randomness = KZGRandomness::empty();
        if let Some(hiding_degree) = hiding_bound {
            let mut rng = rng.ok_or(PCError::MissingRng)?;

            randomness = KZGRandomness::rand(hiding_degree, false, &mut rng);
            Self::check_hiding_bound(
                randomness.blinding_polynomial.degree(),
                lagrange_basis.powers_of_beta_times_gamma_g.len(),
            )?;
        }

        let random_ints = convert_to_bigints(&randomness.blinding_polynomial.coeffs);
        let random_commitment = VariableBase::msm(
            &lagrange_basis.powers_of_beta_times_gamma_g,
            random_ints.as_slice(),
        )
        .to_affine();

        if terminator.load(Ordering::Relaxed) {
            return Err(PCError::Terminated);
        }

        commitment.add_assign_mixed(&random_commitment);

        Ok((KZGCommitment(commitment.into()), randomness))
    }

    /// Compute witness polynomial.
    ///
    /// The witness polynomial w(x) the quotient of the division (p(x) - p(z)) / (x - z)
    /// Observe that this quotient does not change with z because
    /// p(z) is the remainder term. We can therefore omit p(z) when computing the quotient.
    #[allow(clippy::type_complexity)]
    pub fn compute_witness_polynomial(
        polynomial: &DensePolynomial,
        point: Scalar,
        randomness: &KZGRandomness,
    ) -> Result<(DensePolynomial, Option<DensePolynomial>), PCError> {
        let divisor = DensePolynomial::from_coefficients_vec(vec![-point, Scalar::ONE]);

        let witness_polynomial = polynomial / &divisor;

        let random_witness_polynomial = if randomness.is_hiding() {
            let random_p = &randomness.blinding_polynomial;

            let random_witness_polynomial = random_p / &divisor;
            Some(random_witness_polynomial)
        } else {
            None
        };

        Ok((witness_polynomial, random_witness_polynomial))
    }

    pub(crate) fn open_with_witness_polynomial(
        powers: &Powers,
        point: Scalar,
        randomness: &KZGRandomness,
        witness_polynomial: &DensePolynomial,
        hiding_witness_polynomial: Option<&DensePolynomial>,
    ) -> Result<KZGProof, PCError> {
        Self::check_degree_is_too_large(witness_polynomial.degree(), powers.size())?;
        let (num_leading_zeros, witness_coeffs) =
            skip_leading_zeros_and_convert_to_bigints(witness_polynomial);

        let mut w = VariableBase::msm(
            &powers.powers_of_beta_g[num_leading_zeros..],
            &witness_coeffs,
        );

        let random_v = if let Some(hiding_witness_polynomial) = hiding_witness_polynomial {
            let blinding_p = &randomness.blinding_polynomial;
            let blinding_evaluation = blinding_p.evaluate(point);

            let random_witness_coeffs = convert_to_bigints(&hiding_witness_polynomial.coeffs);
            w += &VariableBase::msm(&powers.powers_of_beta_times_gamma_g, &random_witness_coeffs);
            Some(blinding_evaluation)
        } else {
            None
        };

        Ok(KZGProof {
            w: w.to_affine(),
            random_v,
        })
    }

    /// On input a polynomial `p` in Lagrange basis, and a point `point`,
    /// outputs an evaluation proof for the same.
    pub fn open_lagrange(
        lagrange_basis: &LagrangeBasis,
        domain_elements: &[Scalar],
        evaluations: &[Scalar],
        point: Scalar,
        evaluation_at_point: Scalar,
    ) -> Result<KZGProof, PCError> {
        Self::check_degree_is_too_large(evaluations.len() - 1, lagrange_basis.size())?;
        // Ensure that the point is not in the domain
        if lagrange_basis
            .domain
            .evaluate_vanishing_polynomial(point)
            .is_zero()
        {
            Err(anyhow!("Point cannot be in the domain"))?;
        }
        if evaluations
            .len()
            .checked_next_power_of_two()
            .ok_or_else(|| anyhow!("Evaluations length is too large"))?
            != lagrange_basis.size()
        {
            Err(anyhow!("`evaluations.len()` must equal `domain.size()`"))?;
        }

        let mut divisor_evals = cfg_iter!(domain_elements)
            .map(|&e| e - point)
            .collect::<Vec<_>>();
        Scalar::batch_inversion(&mut divisor_evals);
        cfg_iter_mut!(divisor_evals)
            .zip_eq(evaluations)
            .for_each(|(divisor_eval, &eval)| {
                *divisor_eval *= eval - evaluation_at_point;
            });
        let (witness_comm, _) = Self::commit_lagrange(
            lagrange_basis,
            &divisor_evals,
            None,
            &AtomicBool::new(false),
            None,
        )?;

        Ok(KZGProof {
            w: witness_comm.0,
            random_v: None,
        })
    }

    /// On input a polynomial `p` and a point `point`, outputs a proof for the same.
    pub fn open(
        powers: &Powers,
        polynomial: &DensePolynomial,
        point: Scalar,
        rand: &KZGRandomness,
    ) -> Result<KZGProof, PCError> {
        Self::check_degree_is_too_large(polynomial.degree(), powers.size())?;

        let (witness_poly, hiding_witness_poly) =
            Self::compute_witness_polynomial(polynomial, point, rand)?;

        let proof = Self::open_with_witness_polynomial(
            powers,
            point,
            rand,
            &witness_poly,
            hiding_witness_poly.as_ref(),
        );

        proof
    }

    /// Verifies that `value` is the evaluation at `point` of the polynomial
    /// committed inside `commitment`.
    pub fn check(
        vk: &VerifierKey,
        commitment: &KZGCommitment,
        point: Scalar,
        value: Scalar,
        proof: &KZGProof,
    ) -> Result<bool, PCError> {
        let mut inner = commitment.0.to_projective() - vk.g.to_projective().mul(value);
        if let Some(random_v) = proof.random_v {
            inner -= &vk.gamma_g.mul(random_v);
        }
        let lhs = pairing(inner, vk.h);

        let inner = vk.beta_h.to_projective() - vk.h.mul(point);
        let rhs = pairing(proof.w, inner);

        Ok(lhs == rhs)
    }

    /// Check that each `proof_i` in `proofs` is a valid proof of evaluation for
    /// `commitment_i` at `point_i`.
    pub fn batch_check<R: RngCore>(
        vk: &VerifierKey,
        commitments: &[KZGCommitment],
        points: &[Scalar],
        values: &[Scalar],
        proofs: &[KZGProof],
        rng: &mut R,
    ) -> Result<bool, PCError> {
        let g = vk.g.to_projective();
        let gamma_g = vk.gamma_g.to_projective();

        let mut total_c = G1Projective::ZERO;
        let mut total_w = G1Projective::ZERO;

        let mut randomizer = Scalar::ONE;
        // Instead of multiplying g and gamma_g in each turn, we simply accumulate
        // their coefficients and perform a final multiplication at the end.
        let mut g_multiplier = Scalar::ZERO;
        let mut gamma_g_multiplier = Scalar::ZERO;
        for (((c, z), v), proof) in commitments
            .iter()
            .zip_eq(points)
            .zip_eq(values)
            .zip_eq(proofs)
        {
            let w = proof.w;
            let mut temp = w.mul(*z);
            temp.add_assign_mixed(&c.0);
            let c = temp;
            g_multiplier += &(randomizer * v);
            if let Some(random_v) = proof.random_v {
                gamma_g_multiplier += &(randomizer * random_v);
            }
            total_c += &c.mul(randomizer);
            total_w += &w.mul(randomizer);
            // We don't need to sample randomizers from the full field,
            // only from 128-bit strings.
            randomizer = rng.gen::<u128>().into();
        }
        total_c -= &g.mul(g_multiplier);
        total_c -= &gamma_g.mul(gamma_g_multiplier);

        let mut points = vec![-total_w, total_c];
        G1Projective::batch_normalization(&mut points);
        let affine_points: Vec<G1Affine> = points.into_iter().map(|p| p.into()).collect::<Vec<_>>();
        let (total_w, total_c) = (affine_points[0], affine_points[1]);

        let result = product_of_pairings(
            [
                (&G1Prepared::from_affine(total_w), &vk.prepared_beta_h),
                (&G1Prepared::from_affine(total_c), &vk.prepared_h),
            ]
            .iter()
            .copied(),
        )
        .is_one();
        Ok(result)
    }

    pub(crate) fn check_degree_is_too_large(
        degree: usize,
        num_powers: usize,
    ) -> Result<(), PCError> {
        let num_coefficients = degree + 1;
        if num_coefficients > num_powers {
            Err(PCError::TooManyCoefficients {
                num_coefficients,
                num_powers,
            })
        } else {
            Ok(())
        }
    }

    pub(crate) fn check_hiding_bound(
        hiding_poly_degree: usize,
        num_powers: usize,
    ) -> Result<(), PCError> {
        if hiding_poly_degree == 0 {
            Err(PCError::HidingBoundIsZero)
        } else if hiding_poly_degree >= num_powers {
            // The above check uses `>=` because committing to a hiding poly with
            // degree `hiding_poly_degree` requires `hiding_poly_degree + 1` powers.
            Err(PCError::HidingBoundToolarge {
                hiding_poly_degree,
                num_powers,
            })
        } else {
            Ok(())
        }
    }

    pub(crate) fn check_degrees_and_bounds<'a>(
        supported_degree: usize,
        max_degree: usize,
        enforced_degree_bounds: Option<&[usize]>,
        p: impl Into<LabeledPolynomialWithBasis<'a>>,
    ) -> Result<(), PCError> {
        let p = p.into();
        if let Some(bound) = p.degree_bound() {
            let enforced_degree_bounds =
                enforced_degree_bounds.ok_or(PCError::UnsupportedDegreeBound(bound))?;

            if enforced_degree_bounds.binary_search(&bound).is_err() {
                Err(PCError::UnsupportedDegreeBound(bound))
            } else if bound < p.degree() || bound > max_degree {
                return Err(PCError::IncorrectDegreeBound {
                    poly_degree: p.degree(),
                    degree_bound: p.degree_bound().unwrap(),
                    supported_degree,
                    label: p.label().to_string(),
                });
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}

fn skip_leading_zeros_and_convert_to_bigints(p: &DensePolynomial) -> (usize, Vec<Uint<256, 4>>) {
    if p.coeffs.is_empty() {
        (0, vec![])
    } else {
        let mut num_leading_zeros = 0;
        while p.coeffs[num_leading_zeros].is_zero() && num_leading_zeros < p.coeffs.len() {
            num_leading_zeros += 1;
        }
        let coeffs = convert_to_bigints(&p.coeffs[num_leading_zeros..]);
        (num_leading_zeros, coeffs)
    }
}

fn convert_to_bigints(p: &[Scalar]) -> Vec<Uint<256, 4>> {
    let coeffs = cfg_iter!(p).map(|s| s.0).collect::<Vec<_>>();
    coeffs
}

#[cfg(test)]
mod tests {
    #![allow(non_camel_case_types)]
    #![allow(clippy::needless_borrow)]
    use super::*;
    use std::borrow::Cow;

    type KZG_Bls12_377 = KZG10;

    impl KZG10 {
        /// Specializes the public parameters for a given maximum degree `d` for polynomials
        /// `d` should be less that `pp.max_degree()`.
        pub(crate) fn trim(
            pp: &UniversalParams,
            mut supported_degree: usize,
            hiding_bound: Option<usize>,
        ) -> (Powers, VerifierKey) {
            if supported_degree == 1 {
                supported_degree += 1;
            }
            let powers_of_beta_g = pp
                .powers_of_beta_g(0, supported_degree + 1)
                .unwrap()
                .to_vec();

            let powers_of_beta_times_gamma_g = if let Some(hiding_bound) = hiding_bound {
                (0..=(hiding_bound + 1))
                    .map(|i| pp.powers_of_beta_times_gamma_g()[&i])
                    .collect()
            } else {
                vec![]
            };

            let powers = Powers {
                powers_of_beta_g: Cow::Owned(powers_of_beta_g),
                powers_of_beta_times_gamma_g: Cow::Owned(powers_of_beta_times_gamma_g),
            };
            let vk = VerifierKey {
                g: pp.power_of_beta_g(0).unwrap(),
                gamma_g: pp.powers_of_beta_times_gamma_g()[&0],
                h: pp.h,
                beta_h: pp.beta_h(),
                prepared_h: pp.prepared_h.clone(),
                prepared_beta_h: pp.prepared_beta_h.clone(),
            };
            (powers, vk)
        }
    }

    fn end_to_end_test_template() -> Result<(), PCError> {
        let rng = &mut rand::thread_rng();
        for _ in 0..100 {
            let mut degree = 0;
            while degree <= 1 {
                degree = rng.gen::<usize>() % 20;
            }
            let pp = KZG10::load_srs(degree)?;
            let hiding_bound = Some(1);
            let (ck, vk) = KZG10::trim(&pp, degree, hiding_bound);
            let p = DensePolynomial::rand(degree, rng);
            let (comm, rand) = KZG10::commit(
                &ck,
                &(&p).into(),
                hiding_bound,
                &AtomicBool::new(false),
                Some(rng),
            )?;
            let point = Scalar::rand();
            let value = p.evaluate(point);
            let proof = KZG10::open(&ck, &p, point, &rand)?;
            assert!(
                KZG10::check(&vk, &comm, point, value, &proof)?,
                "proof was incorrect for max_degree = {}, polynomial_degree = {}, hiding_bound = {:?}",
                degree,
                p.degree(),
                hiding_bound,
            );
        }
        Ok(())
    }

    fn linear_polynomial_test_template() -> Result<(), PCError> {
        let rng = &mut rand::thread_rng();
        for _ in 0..100 {
            let degree = 50;
            let pp = KZG10::load_srs(degree)?;
            let hiding_bound = Some(1);
            let (ck, vk) = KZG10::trim(&pp, 2, hiding_bound);
            let p = DensePolynomial::rand(1, rng);
            let (comm, rand) = KZG10::commit(
                &ck,
                &(&p).into(),
                hiding_bound,
                &AtomicBool::new(false),
                Some(rng),
            )?;
            let point = Scalar::rand();
            let value = p.evaluate(point);
            let proof = KZG10::open(&ck, &p, point, &rand)?;
            assert!(
                KZG10::check(&vk, &comm, point, value, &proof)?,
                "proof was incorrect for max_degree = {}, polynomial_degree = {}, hiding_bound = {:?}",
                degree,
                p.degree(),
                hiding_bound,
            );
        }
        Ok(())
    }

    fn batch_check_test_template() -> Result<(), PCError> {
        let rng = &mut rand::thread_rng();
        for _ in 0..10 {
            let hiding_bound = Some(1);
            let mut degree = 0;
            while degree <= 1 {
                degree = rng.gen::<usize>() % 20;
            }
            let pp = KZG10::load_srs(degree)?;
            let (ck, vk) = KZG10::trim(&pp, degree, hiding_bound);

            let mut comms = Vec::new();
            let mut values = Vec::new();
            let mut points = Vec::new();
            let mut proofs = Vec::new();

            for _ in 0..10 {
                let p = DensePolynomial::rand(degree, rng);
                let (comm, rand) = KZG10::commit(
                    &ck,
                    &(&p).into(),
                    hiding_bound,
                    &AtomicBool::new(false),
                    Some(rng),
                )?;
                let point = Scalar::rand();
                let value = p.evaluate(point);
                let proof = KZG10::open(&ck, &p, point, &rand)?;

                assert!(KZG10::check(&vk, &comm, point, value, &proof)?);
                comms.push(comm);
                values.push(value);
                points.push(point);
                proofs.push(proof);
            }
            assert!(KZG10::batch_check(
                &vk, &comms, &points, &values, &proofs, rng
            )?);
        }
        Ok(())
    }

    #[test]
    fn test_end_to_end() {
        end_to_end_test_template().expect("test failed for bls12-377");
    }

    #[test]
    fn test_linear_polynomial() {
        linear_polynomial_test_template().expect("test failed for bls12-377");
    }

    #[test]
    fn test_batch_check() {
        batch_check_test_template().expect("test failed for bls12-377");
    }

    #[test]
    fn test_degree_is_too_large() {
        let rng = &mut rand::thread_rng();

        let max_degree = 123;
        let pp = KZG_Bls12_377::load_srs(max_degree).unwrap();
        let (powers, _) = KZG_Bls12_377::trim(&pp, max_degree, None);

        let p = DensePolynomial::rand(max_degree + 1, rng);
        assert!(p.degree() > max_degree);
        assert!(KZG_Bls12_377::check_degree_is_too_large(p.degree(), powers.size()).is_err());
    }
}
