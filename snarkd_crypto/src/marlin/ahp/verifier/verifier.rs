use core::marker::PhantomData;

use crate::{
    bls12_377::Scalar,
    fft::EvaluationDomain,
    marlin::ahp::{
        indexer::CircuitInfo,
        verifier::{FirstMessage, QuerySet, SecondMessage, State, ThirdMessage},
        AHPError, AHPForR1CS,
    },
    utils::*,
};

impl AHPForR1CS {
    /// Output the first message and next round state.
    pub fn verifier_first_round(
        index_info: CircuitInfo,
        batch_size: usize,
        fs_rng: &mut PoseidonSponge,
    ) -> Result<(FirstMessage, State), AHPError> {
        // Check that the R1CS is a square matrix.
        if index_info.num_constraints != index_info.num_variables {
            return Err(AHPError::NonSquareMatrix);
        }

        let constraint_domain = EvaluationDomain::new(index_info.num_constraints)
            .ok_or(AHPError::PolynomialDegreeTooLarge)?;

        let non_zero_a_domain = EvaluationDomain::new(index_info.num_non_zero_a)
            .ok_or(AHPError::PolynomialDegreeTooLarge)?;

        let non_zero_b_domain = EvaluationDomain::new(index_info.num_non_zero_b)
            .ok_or(AHPError::PolynomialDegreeTooLarge)?;

        let non_zero_c_domain = EvaluationDomain::new(index_info.num_non_zero_c)
            .ok_or(AHPError::PolynomialDegreeTooLarge)?;

        let input_domain = EvaluationDomain::new(index_info.num_public_inputs)
            .ok_or(AHPError::PolynomialDegreeTooLarge)?;

        let elems = fs_rng.squeeze_nonnative_field_elements(3 + batch_size - 1);
        let (first, rest) = elems.split_at(3);
        let [alpha, eta_b, eta_c]: [_; 3] = first.try_into().unwrap();
        let mut batch_combiners = vec![Scalar::ONE];
        batch_combiners.extend_from_slice(rest);

        assert!(!constraint_domain
            .evaluate_vanishing_polynomial(alpha)
            .is_zero());

        let message = FirstMessage {
            alpha,
            eta_b,
            eta_c,
            batch_combiners,
        };

        let new_state = State {
            batch_size,
            input_domain,
            constraint_domain,
            non_zero_a_domain,
            non_zero_b_domain,
            non_zero_c_domain,
            first_round_message: Some(message.clone()),
            second_round_message: None,
            third_round_message: None,
            gamma: None,
            mode: PhantomData,
        };

        Ok((message, new_state))
    }

    /// Output the second message and next round state.
    pub fn verifier_second_round(
        mut state: State,
        fs_rng: &mut PoseidonSponge,
    ) -> Result<(SecondMessage, State), AHPError> {
        let elems = fs_rng.squeeze_nonnative_field_elements(1);
        let beta = elems[0];
        assert!(!state
            .constraint_domain
            .evaluate_vanishing_polynomial(beta)
            .is_zero());

        let message = SecondMessage { beta };
        state.second_round_message = Some(message);

        Ok((message, state))
    }

    /// Output the third message and next round state.
    pub fn verifier_third_round(
        mut state: State,
        fs_rng: &mut PoseidonSponge,
    ) -> Result<(ThirdMessage, State), AHPError> {
        let elems = fs_rng.squeeze_nonnative_field_elements(2);
        let r_b = elems[0];
        let r_c = elems[1];
        let message = ThirdMessage { r_b, r_c };

        state.third_round_message = Some(message);
        Ok((message, state))
    }

    /// Output the third message and next round state.
    pub fn verifier_fourth_round(
        mut state: State,
        fs_rng: &mut PoseidonSponge,
    ) -> Result<State, AHPError> {
        let elems = fs_rng.squeeze_nonnative_field_elements(1);
        let gamma = elems[0];

        state.gamma = Some(gamma);
        Ok(state)
    }

    /// Output the query state and next round state.
    pub fn verifier_query_set(state: State) -> (QuerySet, State) {
        (QuerySet::new(&state), state)
    }
}
