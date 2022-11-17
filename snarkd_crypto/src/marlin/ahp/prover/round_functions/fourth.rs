use std::collections::BTreeMap;

use crate::{
    polycommit::sonic_pc::{LabeledPolynomial, PolynomialInfo, PolynomialLabel},
    snark::marlin::{
        ahp::{verifier, AHPError, AHPForR1CS},
        prover, MarlinMode,
    },
};

use rand_core::RngCore;
use snarkvm_fields::PrimeField;

impl AHPForR1CS {
    /// Output the fourth round message and the next state.
    pub fn prover_fourth_round<'a, R: RngCore>(
        verifier_message: &verifier::ThirdMessage,
        state: prover::State<'a>,
        _r: &mut R,
    ) -> Result<prover::FourthOracles, AHPError> {
        let verifier::ThirdMessage { r_b, r_c, .. } = verifier_message;
        let [mut lhs_a, mut lhs_b, mut lhs_c] = state.lhs_polynomials.unwrap();
        lhs_b *= *r_b;
        lhs_c *= *r_c;

        lhs_a += &lhs_b;
        lhs_a += &lhs_c;
        let h_2 = LabeledPolynomial::new("h_2".into(), lhs_a, None, None);
        let oracles = prover::FourthOracles { h_2 };
        assert!(oracles.matches_info(&Self::fourth_round_polynomial_info()));
        Ok(oracles)
    }

    /// Output the number of oracles sent by the prover in the third round.
    pub fn num_fourth_round_oracles() -> usize {
        1
    }

    /// Output the degree bounds of oracles in the third round.
    pub fn fourth_round_polynomial_info() -> BTreeMap<PolynomialLabel, PolynomialInfo> {
        [("h_2".into(), PolynomialInfo::new("h_2".into(), None, None))].into()
    }
}
