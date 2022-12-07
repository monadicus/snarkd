use crate::circuit::traits::Subtractor;

use super::*;

impl Subtractor for Boolean {
    /// Returns the difference of `self` and `other` as a difference bit and borrow bit.
    fn subtractor(&self, other: &Self, borrow: &Self) -> (Self, Self) {
        // Compute the difference bit.
        let c0 = self ^ other;
        let difference = &c0 ^ borrow;

        // Compute the borrow bit.
        let c1 = !self & other;
        let c2 = borrow & !c0;
        let borrow = c1 | c2;

        (difference, borrow)
    }
}
