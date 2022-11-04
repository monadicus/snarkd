use crate::{
    fft::EvaluationDomain,
    snark::marlin::{
        ahp::verifier::{FirstMessage, SecondMessage, ThirdMessage},
        MarlinMode,
    },
};
use core::marker::PhantomData;

/// State of the AHP verifier.
#[derive(Debug)]
pub struct State<F: PrimeField, MM: MarlinMode> {
    pub(in crate::snark::marlin) batch_size: usize,
    pub(crate) input_domain: EvaluationDomain<F>,
    pub(crate) constraint_domain: EvaluationDomain<F>,
    pub(crate) non_zero_a_domain: EvaluationDomain<F>,
    pub(crate) non_zero_b_domain: EvaluationDomain<F>,
    pub(crate) non_zero_c_domain: EvaluationDomain<F>,

    pub(crate) first_round_message: Option<FirstMessage<F>>,
    pub(crate) second_round_message: Option<SecondMessage<F>>,
    pub(crate) third_round_message: Option<ThirdMessage<F>>,

    pub(crate) gamma: Option<F>,
    pub(crate) mode: PhantomData<MM>,
}
