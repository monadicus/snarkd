mod assignment;
pub use assignment::*;

mod constraint_counter;
pub use constraint_counter::*;

mod constraint_system;
pub use constraint_system::*;

mod constraint_variable;
pub use constraint_variable::*;

mod linear_combination;
pub use linear_combination::*;

mod namespace;
pub use namespace::*;

mod optional_vec;
pub use optional_vec::*;

mod variable;
pub use variable::*;

#[cfg(test)]
#[path = ""]
mod test {
    use super::*;

    mod linear_combination_test;
    mod optional_vec_test;
    // pub mod test_constraint_system;
}
