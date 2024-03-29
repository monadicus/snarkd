pub mod fixed_base;
pub use fixed_base::*;

#[cfg(test)]
pub mod tests;

pub mod variable_base;
pub use variable_base::*;

/// The result of this function is only approximately `ln(a)`
/// [`Explanation of usage`]
///
/// [`Explanation of usage`]: https://github.com/scipr-lab/zexe/issues/79#issue-556220473
fn ln_without_floats(a: usize) -> usize {
    // log2(a) * ln(2)
    (crate::fft::domain::log2(a) * 69 / 100) as usize
}
