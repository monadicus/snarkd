use crate::circuit::traits::Equal;

use super::Boolean;

impl Equal<Self> for Boolean {
    /// Returns `true` if `self` and `other` are equal.
    fn is_equal(&self, other: &Self) -> Boolean {
        !self.is_not_equal(other)
    }

    /// Returns `true` if `self` and `other` are *not* equal.
    fn is_not_equal(&self, other: &Self) -> Boolean {
        self ^ other
    }
}
