use std::fmt;

#[derive(Clone, Debug)]
pub struct ConstrainedString(String);

impl fmt::Display for ConstrainedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "scalar")
    }
}
