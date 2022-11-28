use crate::{
    bls12_377::Scalar,
    fft::EvaluationDomain,
    marlin::ahp::verifier::{FirstMessage, SecondMessage, ThirdMessage},
};

/// State of the AHP verifier.
#[derive(Debug)]
pub struct State {
    pub(in crate::marlin) batch_size: usize,
    pub(crate) input_domain: EvaluationDomain,
    pub(crate) constraint_domain: EvaluationDomain,
    pub(crate) non_zero_a_domain: EvaluationDomain,
    pub(crate) non_zero_b_domain: EvaluationDomain,
    pub(crate) non_zero_c_domain: EvaluationDomain,

    pub(crate) first_round_message: Option<FirstMessage>,
    pub(crate) second_round_message: Option<SecondMessage>,
    pub(crate) third_round_message: Option<ThirdMessage>,

    pub(crate) gamma: Option<Scalar>,
}
