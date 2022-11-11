use crate::circuit::helpers::{Count, Mode};

/// Trait for determining the number of constants, public input, private inputs, and constraints for an operation.
pub trait Metrics<Op: ?Sized> {
    type Case: ?Sized;

    /// Returns the number of constants, public inputs, private inputs, and constraints.
    fn count(parameter: &Self::Case) -> Count;
}

/// Trait for determining the mode of the output of an operation.
pub trait OutputMode<Op: ?Sized> {
    type Case: ?Sized;

    /// Returns the mode of the output.
    fn output_mode(input: &Self::Case) -> Mode;
}
