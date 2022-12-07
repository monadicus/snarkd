use crate::circuit::types::Boolean;

pub trait Equal<Rhs: ?Sized = Self> {
    /// Returns `true` if `self` and `other` are equal.
    fn is_equal(&self, other: &Rhs) -> Boolean;

    /// Returns `true` if `self` and `other` are *not* equal.
    fn is_not_equal(&self, other: &Rhs) -> Boolean;
}

/// Trait for comparator operations.
pub trait Compare<Rhs: ?Sized = Self> {
    /// Returns `true` if `self` is less than `other`.
    fn is_less_than(&self, other: &Rhs) -> Boolean;

    /// Returns `true` if `self` is greater than `other`.
    fn is_greater_than(&self, other: &Rhs) -> Boolean;

    /// Returns `true` if `self` is less than or equal to `other`.
    fn is_less_than_or_equal(&self, other: &Rhs) -> Boolean;

    /// Returns `true` if `self` is greater than or equal to `other`.
    fn is_greater_than_or_equal(&self, other: &Rhs) -> Boolean;
}

/// Trait for ternary operations.
pub trait Ternary {
    /// Returns `first` if `condition` is `true`, otherwise returns `second`.
    fn ternary(condition: &Boolean, first: &Self, second: &Self) -> Self
    where
        Self: Sized;
}
