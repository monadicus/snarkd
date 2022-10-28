use snarkd_ir::{Header, Program};

/// A trait describing a mechanism for producing output from a given program and input
pub trait Evaluator {
    type Output;
    type Error;

    fn evaluate(&mut self, program: &Program, header: &Header)
        -> Result<Self::Output, Self::Error>;
}
