use crate::{
    bls12_377::{Field, Scalar, ToScalar},
    fft::EvaluationDomain,
    marlin::{
        ahp::{AHPError, AHPForR1CS, EvaluationsProvider},
        proof, prover, witness_label, CircuitProvingKey, CircuitVerifyingKey, MarlinError, Prepare,
        Proof,
    },
    polycommit::sonic_pc::{
        Commitment, Evaluations, LabeledCommitment, QuerySet, Randomness, SonicKZG10,
        UniversalParams,
    },
    r1cs::ConstraintSynthesizer,
    utils::*,
    SNARKError, SNARK, SRS,
};
use itertools::Itertools;
use rand::{CryptoRng, Rng};

use std::{borrow::Borrow, sync::Arc};

use core::sync::atomic::{AtomicBool, Ordering};

use super::Certificate;

/// The Marlin proof system.
#[derive(Clone, Debug)]
pub struct MarlinSNARK {
    pub mode: bool,
}

impl MarlinSNARK {
    /// The personalization string for this protocol.
    /// Used to personalize the Fiat-Shamir RNG.
    pub const PROTOCOL_NAME: &'static [u8] = b"MARLIN-2019";

    /// Generate the index-specific (i.e., circuit-specific) prover and verifier
    /// keys. This is a trusted setup.
    ///
    /// # Warning
    ///
    /// This method should be used *only* for testing purposes, and not in production.
    /// In production, one should instead perform a universal setup via [`Self::universal_setup`],
    /// and then deterministically specialize the resulting universal SRS via [`Self::circuit_setup`].
    #[allow(clippy::type_complexity)]
    pub fn circuit_specific_setup<C: ConstraintSynthesizer<Scalar>>(
        c: &C,
        mode: bool,
    ) -> Result<(CircuitProvingKey, CircuitVerifyingKey), SNARKError> {
        let circuit = AHPForR1CS::index(c, mode)?;
        let srs = Self::universal_setup(circuit.max_degree())?;
        Self::circuit_setup(&srs, c, mode)
    }

    /// Generates the circuit proving and verifying keys.
    /// This is a deterministic algorithm that anyone can rerun.
    #[allow(clippy::type_complexity)]
    pub fn circuit_setup<C: ConstraintSynthesizer<Scalar>>(
        universal_srs: &UniversalParams,
        circuit: &C,
        mode: bool,
    ) -> Result<(CircuitProvingKey, CircuitVerifyingKey), SNARKError> {
        // TODO: Add check that c is in the correct mode.
        // Increase the universal SRS size to support the circuit size.
        let index = AHPForR1CS::index(circuit, mode)?;
        if universal_srs.max_degree() < index.max_degree() {
            universal_srs
                .increase_degree(index.max_degree())
                .map_err(|_| {
                    MarlinError::IndexTooLarge(universal_srs.max_degree(), index.max_degree())
                })?;
        }

        let coefficient_support = AHPForR1CS::get_degree_bounds(&index.index_info);

        // Marlin only needs degree 2 random polynomials.
        let supported_hiding_bound = 1;
        let (committer_key, verifier_key) = SonicKZG10::trim(
            universal_srs,
            index.max_degree(),
            [index.constraint_domain_size()],
            supported_hiding_bound,
            Some(&coefficient_support),
        )?;

        let (mut circuit_commitments, circuit_commitment_randomness): (_, _) =
            SonicKZG10::commit(&committer_key, index.iter().map(Into::into))?;

        circuit_commitments.sort_by(|c1, c2| c1.label().cmp(c2.label()));
        let circuit_commitments = circuit_commitments
            .into_iter()
            .map(|c| *c.commitment())
            .collect();
        let circuit_verifying_key = CircuitVerifyingKey {
            circuit_info: index.index_info,
            circuit_commitments,
            verifier_key,
        };

        let circuit_proving_key = CircuitProvingKey {
            circuit: Arc::new(index),
            circuit_commitment_randomness,
            circuit_verifying_key: circuit_verifying_key.clone(),
            committer_key: Arc::new(committer_key),
        };

        Ok((circuit_proving_key, circuit_verifying_key))
    }

    fn terminate(terminator: &AtomicBool) -> Result<(), MarlinError> {
        if terminator.load(Ordering::Relaxed) {
            Err(MarlinError::Terminated)
        } else {
            Ok(())
        }
    }

    fn init_sponge(
        fs_parameters: &PoseidonParameters,
        batch_size: usize,
        circuit_commitments: &[crate::polycommit::sonic_pc::Commitment],
        inputs: &[Vec<Scalar>],
    ) -> PoseidonSponge {
        let mut sponge = PoseidonSponge::new(Arc::new(fs_parameters.clone()));
        sponge.absorb_bytes(&b"MARLIN-2019"[..]);
        sponge.absorb_bytes(&batch_size.to_le_bytes());
        sponge.absorb_native_field_elements(
            &circuit_commitments
                .iter()
                .flat_map(|comm| [comm.0.x, comm.0.y])
                .collect::<Vec<_>>(),
        );
        for input in inputs {
            sponge.absorb_nonnative_field_elements(input.iter().copied());
        }
        sponge
    }

    fn init_sponge_for_certificate(
        fs_parameters: &PoseidonParameters,
        circuit_commitments: &[crate::polycommit::sonic_pc::Commitment],
    ) -> PoseidonSponge {
        let mut sponge = PoseidonSponge::new(Arc::new(fs_parameters.clone()));
        sponge.absorb_bytes(&b"MARLIN-2019"[..]);
        sponge.absorb_native_field_elements(
            &circuit_commitments
                .iter()
                .flat_map(|comm| [comm.0.x, comm.0.y])
                .collect::<Vec<_>>(),
        );
        sponge
    }

    fn absorb_labeled_with_msg(
        comms: &[LabeledCommitment],
        message: &prover::ThirdMessage,
        sponge: &mut PoseidonSponge,
    ) {
        let commitments: Vec<_> = comms.iter().map(|c| *c.commitment()).collect();
        Self::absorb_with_msg(&commitments, message, sponge)
    }

    fn absorb_labeled(comms: &[LabeledCommitment], sponge: &mut PoseidonSponge) {
        let commitments: Vec<_> = comms.iter().map(|c| *c.commitment()).collect();
        Self::absorb(&commitments, sponge);
    }

    fn absorb(commitments: &[Commitment], sponge: &mut PoseidonSponge) {
        sponge.absorb_native_field_elements(
            &commitments
                .iter()
                .flat_map(|comm| [comm.0.x, comm.0.y])
                .collect::<Vec<_>>(),
        );
    }

    fn absorb_with_msg(
        commitments: &[Commitment],
        msg: &prover::ThirdMessage,
        sponge: &mut PoseidonSponge,
    ) {
        Self::absorb(commitments, sponge);
        sponge.absorb_nonnative_field_elements([msg.sum_a, msg.sum_b, msg.sum_c]);
    }
}

impl SNARK for MarlinSNARK {
    fn universal_setup(max_degree: usize) -> Result<UniversalParams, SNARKError> {
        SonicKZG10::setup(max_degree).map_err(Into::into)
    }

    fn setup<C: ConstraintSynthesizer<Scalar>>(
        circuit: &C,
        srs: &mut SRS<UniversalParams>,
        mode: bool,
    ) -> Result<(CircuitProvingKey, CircuitVerifyingKey), SNARKError> {
        match srs {
            SRS::CircuitSpecific => Self::circuit_specific_setup(circuit, mode),
            SRS::Universal(srs) => Self::circuit_setup(srs, circuit, mode),
        }
        .map_err(SNARKError::from)
    }

    fn prove_vk(
        fs_parameters: &PoseidonParameters,
        verifying_key: &CircuitVerifyingKey,
        proving_key: &CircuitProvingKey,
    ) -> Result<Certificate, SNARKError> {
        // Initialize sponge
        let mut sponge =
            Self::init_sponge_for_certificate(fs_parameters, &verifying_key.circuit_commitments);
        // Compute challenges for linear combination, and the point to evaluate the polynomials at.
        // The linear combination requires `num_polynomials - 1` coefficients
        // (since the first coeff is 1), and so we squeeze out `num_polynomials` points.
        let mut challenges =
            sponge.squeeze_nonnative_field_elements(verifying_key.circuit_commitments.len());
        let point = challenges.pop().unwrap();
        let one = Scalar::ONE;
        let linear_combination_challenges = core::iter::once(&one).chain(challenges.iter());

        // We will construct a linear combination and provide a proof of evaluation of the lc at `point`.
        let mut lc = crate::polycommit::sonic_pc::LinearCombination::empty("circuit_check");
        for (poly, &c) in proving_key
            .circuit
            .iter()
            .zip(linear_combination_challenges)
        {
            lc.add(c, poly.label());
        }

        let query_set =
            QuerySet::from_iter([("circuit_check".into(), ("challenge".into(), point))]);
        let commitments = verifying_key
            .iter()
            .cloned()
            .zip_eq(AHPForR1CS::index_polynomial_info().values())
            .map(|(c, info)| LabeledCommitment::new_with_info(info, c))
            .collect::<Vec<_>>();

        let certificate = SonicKZG10::open_combinations(
            &proving_key.committer_key,
            &[lc],
            proving_key.circuit.iter(),
            &commitments,
            &query_set,
            &proving_key.circuit_commitment_randomness.clone(),
            &mut sponge,
        )?;

        Ok(Certificate::new(certificate))
    }

    fn verify_vk<C: ConstraintSynthesizer<Scalar>>(
        fs_parameters: &PoseidonParameters,
        circuit: &C,
        verifying_key: &CircuitVerifyingKey,
        certificate: &Certificate,
    ) -> Result<bool, SNARKError> {
        let info = AHPForR1CS::index_polynomial_info();
        // Initialize sponge.
        let mut sponge =
            Self::init_sponge_for_certificate(fs_parameters, &verifying_key.circuit_commitments);
        // Compute challenges for linear combination, and the point to evaluate the polynomials at.
        // The linear combination requires `num_polynomials - 1` coefficients
        // (since the first coeff is 1), and so we squeeze out `num_polynomials` points.
        let mut challenges =
            sponge.squeeze_nonnative_field_elements(verifying_key.circuit_commitments.len());
        let point = challenges.pop().unwrap();

        let evaluations_at_point = AHPForR1CS::evaluate_index_polynomials(circuit, point)?;
        let one = Scalar::ONE;
        let linear_combination_challenges = core::iter::once(&one).chain(challenges.iter());

        // We will construct a linear combination and provide a proof of evaluation of the lc at `point`.
        let mut lc = crate::polycommit::sonic_pc::LinearCombination::empty("circuit_check");
        let mut evaluation = Scalar::ZERO;
        for ((label, &c), eval) in info
            .keys()
            .zip_eq(linear_combination_challenges)
            .zip_eq(evaluations_at_point)
        {
            lc.add(c, label.as_str());
            evaluation += c * eval;
        }

        let query_set =
            QuerySet::from_iter([("circuit_check".into(), ("challenge".into(), point))]);
        let commitments = verifying_key
            .iter()
            .cloned()
            .zip_eq(info.values())
            .map(|(c, info)| LabeledCommitment::new_with_info(info, c))
            .collect::<Vec<_>>();
        let evaluations = Evaluations::from_iter([(("circuit_check".into(), point), evaluation)]);

        SonicKZG10::check_combinations(
            &verifying_key.verifier_key,
            &[lc],
            &commitments,
            &query_set,
            &evaluations,
            &certificate.pc_proof,
            &mut sponge,
        )
        .map_err(Into::into)
    }

    #[allow(clippy::only_used_in_recursion)]
    fn prove_batch_with_terminator<C: ConstraintSynthesizer<Scalar>, R: Rng + CryptoRng>(
        &self,
        fs_parameters: &PoseidonParameters,
        circuit_proving_key: &CircuitProvingKey,
        circuits: &[C],
        terminator: &AtomicBool,
        zk_rng: &mut R,
    ) -> Result<Proof, SNARKError> {
        let batch_size = circuits.len();
        if batch_size == 0 {
            return Err(SNARKError::EmptyBatch);
        }

        Self::terminate(terminator)?;

        let (ahp, prover_state) =
            AHPForR1CS::init_prover(&circuit_proving_key.circuit, circuits, self.mode)?;
        let public_input = prover_state.public_inputs();
        let padded_public_input = prover_state.padded_public_inputs();
        assert_eq!(prover_state.batch_size, batch_size);

        let mut sponge = Self::init_sponge(
            fs_parameters,
            batch_size,
            &circuit_proving_key
                .circuit_verifying_key
                .circuit_commitments,
            &padded_public_input,
        );

        // --------------------------------------------------------------------
        // First round

        Self::terminate(terminator)?;
        let mut prover_state = ahp.prover_first_round(prover_state)?;
        Self::terminate(terminator)?;

        let (first_commitments, first_commitment_randomnesses) = {
            let first_round_oracles =
                Arc::get_mut(prover_state.first_round_oracles.as_mut().unwrap()).unwrap();
            SonicKZG10::commit(
                &circuit_proving_key.committer_key,
                first_round_oracles.iter_for_commit(),
            )?
        };

        Self::absorb_labeled(&first_commitments, &mut sponge);
        Self::terminate(terminator)?;

        let (verifier_first_message, verifier_state) = AHPForR1CS::verifier_first_round(
            circuit_proving_key.circuit_verifying_key.circuit_info,
            batch_size,
            &mut sponge,
        )?;
        // --------------------------------------------------------------------

        // --------------------------------------------------------------------
        // Second round

        Self::terminate(terminator)?;
        let (second_oracles, prover_state) =
            ahp.prover_second_round(&verifier_first_message, prover_state, zk_rng);
        Self::terminate(terminator)?;

        let (second_commitments, second_commitment_randomnesses) =
            SonicKZG10::commit_with_terminator(
                &circuit_proving_key.committer_key,
                second_oracles.iter().map(Into::into),
                terminator,
            )?;

        Self::absorb_labeled(&second_commitments, &mut sponge);
        Self::terminate(terminator)?;

        let (verifier_second_msg, verifier_state) =
            AHPForR1CS::verifier_second_round(verifier_state, &mut sponge)?;
        // --------------------------------------------------------------------

        // --------------------------------------------------------------------
        // Third round

        Self::terminate(terminator)?;

        let (prover_third_message, third_oracles, prover_state) =
            AHPForR1CS::prover_third_round(&verifier_second_msg, prover_state, zk_rng)?;
        Self::terminate(terminator)?;

        let (third_commitments, third_commitment_randomnesses) =
            SonicKZG10::commit_with_terminator(
                &circuit_proving_key.committer_key,
                third_oracles.iter().map(Into::into),
                terminator,
            )?;

        Self::absorb_labeled_with_msg(&third_commitments, &prover_third_message, &mut sponge);

        let (verifier_third_msg, verifier_state) =
            AHPForR1CS::verifier_third_round(verifier_state, &mut sponge)?;
        // --------------------------------------------------------------------

        // --------------------------------------------------------------------
        // Fourth round

        Self::terminate(terminator)?;

        let first_round_oracles = Arc::clone(prover_state.first_round_oracles.as_ref().unwrap());
        let fourth_oracles =
            AHPForR1CS::prover_fourth_round(&verifier_third_msg, prover_state, zk_rng)?;
        Self::terminate(terminator)?;

        let (fourth_commitments, fourth_commitment_randomnesses) =
            SonicKZG10::commit_with_terminator(
                &circuit_proving_key.committer_key,
                fourth_oracles.iter().map(Into::into),
                terminator,
            )?;

        Self::absorb_labeled(&fourth_commitments, &mut sponge);

        let verifier_state = AHPForR1CS::verifier_fourth_round(verifier_state, &mut sponge)?;
        // --------------------------------------------------------------------

        Self::terminate(terminator)?;

        // Gather prover polynomials in one vector.
        let polynomials: Vec<_> = circuit_proving_key
            .circuit
            .iter() // 12 items
            .chain(first_round_oracles.iter_for_open()) // 3 * batch_size + (self.mode as usize) items
            .chain(second_oracles.iter()) // 2 items
            .chain(third_oracles.iter()) // 3 items
            .chain(fourth_oracles.iter()) // 1 item
            .collect();

        Self::terminate(terminator)?;

        // Gather commitments in one vector.
        let witness_commitments = first_commitments.chunks_exact(3);
        let mask_poly = self
            .mode
            .then(|| *witness_commitments.remainder()[0].commitment());
        let witness_commitments = witness_commitments
            .map(|c| proof::WitnessCommitments {
                w: *c[0].commitment(),
                z_a: *c[1].commitment(),
                z_b: *c[2].commitment(),
            })
            .collect();
        #[rustfmt::skip]
        let commitments = proof::Commitments {
            witness_commitments,
            mask_poly,

            g_1: *second_commitments[0].commitment(),
            h_1: *second_commitments[1].commitment(),


            g_a: *third_commitments[0].commitment(),
            g_b: *third_commitments[1].commitment(),
            g_c: *third_commitments[2].commitment(),

            h_2: *fourth_commitments[0].commitment(),
        };

        let labeled_commitments: Vec<_> = circuit_proving_key
            .circuit_verifying_key
            .iter()
            .cloned()
            .zip_eq(AHPForR1CS::index_polynomial_info().values())
            .map(|(c, info)| LabeledCommitment::new_with_info(info, c))
            .chain(first_commitments.into_iter())
            .chain(second_commitments.into_iter())
            .chain(third_commitments.into_iter())
            .chain(fourth_commitments.into_iter())
            .collect();

        // Gather commitment randomness together.
        let commitment_randomnesses: Vec<Randomness> = circuit_proving_key
            .circuit_commitment_randomness
            .clone()
            .into_iter()
            .chain(first_commitment_randomnesses)
            .chain(second_commitment_randomnesses)
            .chain(third_commitment_randomnesses)
            .chain(fourth_commitment_randomnesses)
            .collect();

        if !self.mode {
            let empty_randomness = Randomness::empty();
            assert!(commitment_randomnesses
                .iter()
                .all(|r| r == &empty_randomness));
        }

        // Compute the AHP verifier's query set.
        let (query_set, verifier_state) = AHPForR1CS::verifier_query_set(verifier_state);
        let lc_s = ahp.construct_linear_combinations(
            &public_input,
            &polynomials,
            &prover_third_message,
            &verifier_state,
        )?;

        Self::terminate(terminator)?;

        let mut evaluations = std::collections::BTreeMap::new();
        for (label, (_, point)) in query_set.to_set() {
            if !AHPForR1CS::LC_WITH_ZERO_EVAL.contains(&label.as_str()) {
                let lc = lc_s
                    .get(&label)
                    .ok_or_else(|| AHPError::MissingEval(label.to_string()))?;
                let evaluation = polynomials.get_lc_eval(lc, point)?;
                evaluations.insert(label, evaluation);
            }
        }

        let evaluations = proof::Evaluations::from_map(&evaluations, batch_size);

        Self::terminate(terminator)?;

        sponge.absorb_nonnative_field_elements(evaluations.to_field_elements());

        let pc_proof = SonicKZG10::open_combinations(
            &circuit_proving_key.committer_key,
            lc_s.values(),
            polynomials,
            &labeled_commitments,
            &query_set.to_set(),
            &commitment_randomnesses,
            &mut sponge,
        )?;

        Self::terminate(terminator)?;

        let proof = Proof::new(
            batch_size,
            commitments,
            evaluations,
            prover_third_message,
            pc_proof,
        );
        assert_eq!(proof.pc_proof.is_hiding(), self.mode);

        Ok(proof)
    }

    fn verify_batch_prepared<TS: ToScalar + ?Sized, B: Borrow<TS>>(
        &self,
        fs_parameters: &PoseidonParameters,
        prepared_verifying_key: &<CircuitVerifyingKey as Prepare>::Prepared,
        public_inputs: &[B],
        proof: &Proof,
    ) -> Result<bool, SNARKError> {
        let ahp = AHPForR1CS { mode: self.mode };
        let circuit_verifying_key = &prepared_verifying_key.orig_vk;
        if public_inputs.is_empty() {
            return Err(SNARKError::EmptyBatch);
        }

        let comms = &proof.commitments;
        let proof_has_correct_zk_mode = if self.mode {
            proof.pc_proof.is_hiding() & comms.mask_poly.is_some()
        } else {
            !proof.pc_proof.is_hiding() & comms.mask_poly.is_none()
        };
        if !proof_has_correct_zk_mode {
            eprintln!(
                "Found `mask_poly` in the first round when not expected, or proof has incorrect hiding mode ({})",
                proof.pc_proof.is_hiding()
            );
            return Ok(false);
        }

        let batch_size = public_inputs.len();

        let first_round_info = ahp.first_round_polynomial_info(batch_size);
        let mut first_commitments = comms
            .witness_commitments
            .iter()
            .enumerate()
            .flat_map(|(i, c)| {
                [
                    LabeledCommitment::new_with_info(
                        &first_round_info[&witness_label("w", i)],
                        c.w,
                    ),
                    LabeledCommitment::new_with_info(
                        &first_round_info[&witness_label("z_a", i)],
                        c.z_a,
                    ),
                    LabeledCommitment::new_with_info(
                        &first_round_info[&witness_label("z_b", i)],
                        c.z_b,
                    ),
                ]
            })
            .collect::<Vec<_>>();
        if self.mode {
            first_commitments.push(LabeledCommitment::new_with_info(
                first_round_info.get("mask_poly").unwrap(),
                comms.mask_poly.unwrap(),
            ));
        }

        let second_round_info =
            ahp.second_round_polynomial_info(&circuit_verifying_key.circuit_info);
        let second_commitments = [
            LabeledCommitment::new_with_info(&second_round_info["g_1"], comms.g_1),
            LabeledCommitment::new_with_info(&second_round_info["h_1"], comms.h_1),
        ];

        let third_round_info =
            AHPForR1CS::third_round_polynomial_info(&circuit_verifying_key.circuit_info);
        let third_commitments = [
            LabeledCommitment::new_with_info(&third_round_info["g_a"], comms.g_a),
            LabeledCommitment::new_with_info(&third_round_info["g_b"], comms.g_b),
            LabeledCommitment::new_with_info(&third_round_info["g_c"], comms.g_c),
        ];

        let fourth_round_info = AHPForR1CS::fourth_round_polynomial_info();
        let fourth_commitments = [LabeledCommitment::new_with_info(
            &fourth_round_info["h_2"],
            comms.h_2,
        )];

        let input_domain =
            EvaluationDomain::new(circuit_verifying_key.circuit_info.num_public_inputs).unwrap();

        let (padded_public_inputs, public_inputs): (Vec<_>, Vec<_>) = {
            public_inputs
                .iter()
                .map(|input| {
                    let input = input.borrow().to_scalar().unwrap();
                    let mut new_input = vec![Scalar::ONE];
                    new_input.extend_from_slice(input.as_slice());
                    new_input.resize(input.len().max(input_domain.size()), Scalar::ZERO);
                    if cfg!(debug_assertions) {
                        println!("Number of padded public variables: {}", new_input.len());
                    }
                    let unformatted =
                        prover::ConstraintSystem::unformat_public_input(new_input.as_slice());
                    (new_input, unformatted)
                })
                .unzip()
        };

        let mut sponge = Self::init_sponge(
            fs_parameters,
            batch_size,
            &circuit_verifying_key.circuit_commitments,
            &padded_public_inputs,
        );

        // --------------------------------------------------------------------
        // First round
        Self::absorb_labeled(&first_commitments, &mut sponge);
        let (_, verifier_state) = AHPForR1CS::verifier_first_round(
            circuit_verifying_key.circuit_info,
            batch_size,
            &mut sponge,
        )?;
        // --------------------------------------------------------------------

        // --------------------------------------------------------------------
        // Second round
        Self::absorb_labeled(&second_commitments, &mut sponge);
        let (_, verifier_state) = AHPForR1CS::verifier_second_round(verifier_state, &mut sponge)?;
        // --------------------------------------------------------------------

        // --------------------------------------------------------------------
        // Third round

        Self::absorb_labeled_with_msg(&third_commitments, &proof.msg, &mut sponge);
        let (_, verifier_state) = AHPForR1CS::verifier_third_round(verifier_state, &mut sponge)?;
        // --------------------------------------------------------------------

        // --------------------------------------------------------------------
        // Fourth round

        Self::absorb_labeled(&fourth_commitments, &mut sponge);
        let verifier_state = AHPForR1CS::verifier_fourth_round(verifier_state, &mut sponge)?;
        // --------------------------------------------------------------------

        // Collect degree bounds for commitments. Indexed polynomials have *no*
        // degree bounds because we know the committed index polynomial has the
        // correct degree.

        // Gather commitments in one vector.
        let commitments: Vec<_> = circuit_verifying_key
            .iter()
            .cloned()
            .zip_eq(AHPForR1CS::index_polynomial_info().values())
            .map(|(c, info)| LabeledCommitment::new_with_info(info, c))
            .chain(first_commitments)
            .chain(second_commitments)
            .chain(third_commitments)
            .chain(fourth_commitments)
            .collect();

        let (query_set, verifier_state) = AHPForR1CS::verifier_query_set(verifier_state);

        sponge.absorb_nonnative_field_elements(proof.evaluations.to_field_elements());

        let mut evaluations = Evaluations::new();

        for (label, (_point_name, q)) in query_set.to_set() {
            if AHPForR1CS::LC_WITH_ZERO_EVAL.contains(&label.as_ref()) {
                evaluations.insert((label, q), Scalar::ZERO);
            } else {
                let eval = proof
                    .evaluations
                    .get(&label)
                    .ok_or_else(|| AHPError::MissingEval(label.clone()))?;
                evaluations.insert((label, q), eval);
            }
        }

        let lc_s = ahp.construct_linear_combinations(
            &public_inputs,
            &evaluations,
            &proof.msg,
            &verifier_state,
        )?;

        let evaluations_are_correct = SonicKZG10::check_combinations(
            &circuit_verifying_key.verifier_key,
            lc_s.values(),
            &commitments,
            &query_set.to_set(),
            &evaluations,
            &proof.pc_proof,
            &mut sponge,
        )?;

        if !evaluations_are_correct {
            #[cfg(debug_assertions)]
            eprintln!("SonicKZG10::Check failed");
        }
        Ok(evaluations_are_correct & proof_has_correct_zk_mode)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::{bls12_377::Scalar, marlin::MarlinSNARK, r1cs::ConstraintSystem, SRS};
    use anyhow::{anyhow, Result};
    use core::ops::MulAssign;

    #[derive(Copy, Clone)]
    pub struct Circuit {
        pub a: Option<Scalar>,
        pub b: Option<Scalar>,
        pub num_constraints: usize,
        pub num_variables: usize,
    }

    impl ConstraintSynthesizer<Scalar> for Circuit {
        fn generate_constraints<CS: ConstraintSystem<Field = Scalar>>(
            &self,
            cs: &mut CS,
        ) -> Result<()> {
            let a = cs.alloc(
                || "a",
                || self.a.ok_or_else(|| anyhow!("assignment missing")),
            )?;
            let b = cs.alloc(
                || "b",
                || self.b.ok_or_else(|| anyhow!("assignment missing")),
            )?;
            let c = cs.alloc_input(
                || "c",
                || {
                    let mut a = self.a.ok_or_else(|| anyhow!("assignment missing"))?;
                    let b = self.b.ok_or_else(|| anyhow!("assignment missing"))?;

                    a.mul_assign(&b);
                    Ok(a)
                },
            )?;

            for i in 0..(self.num_variables - 3) {
                let _ = cs.alloc(
                    || format!("var {}", i),
                    || self.a.ok_or_else(|| anyhow!("assignment missing")),
                )?;
            }

            for i in 0..(self.num_constraints - 1) {
                cs.enforce(
                    || format!("constraint {}", i),
                    |lc| lc + a,
                    |lc| lc + b,
                    |lc| lc + c,
                );
            }

            Ok(())
        }
    }

    type TestSNARK = MarlinSNARK;

    #[test]
    fn marlin_snark_test() {
        let mut rng = rand::thread_rng();

        // Construct the circuit.
        let a = Scalar::rand();
        let b = Scalar::rand();
        let mut c = a;
        c.mul_assign(&b);

        let circ = Circuit {
            a: Some(a),
            b: Some(b),
            num_constraints: 100,
            num_variables: 25,
        };

        // Generate the circuit parameters.

        let (pk, vk) = TestSNARK::setup(&circ, &mut SRS::CircuitSpecific, true).unwrap();

        // Test native proof and verification.
        let fs_parameters = PoseidonParameters::default();

        let snark = TestSNARK { mode: true };
        let proof = snark.prove(&fs_parameters, &pk, &circ, &mut rng).unwrap();

        assert!(
            snark
                .verify::<[Scalar], _>(&fs_parameters, &vk, [c], &proof)
                .unwrap(),
            "The native verification check fails."
        );
    }
}
