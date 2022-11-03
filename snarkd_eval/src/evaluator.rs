use snarkd_crypto::{Field, Group};
use snarkd_ir::{InputData, Program};

/// A trait describing a mechanism for producing output from a given program and input
pub trait Evaluator<F: Field, G: Group> {
    type Output;
    type Error;

    fn evaluate(
        &mut self,
        program: &Program,
        input: &InputData,
    ) -> Result<Self::Output, Self::Error>;
}
