// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use crate::circuit::traits::Adder;

use super::*;

impl Adder for Boolean {
    /// Returns the sum of `self` and `other` as a sum bit and carry bit.
    fn adder(&self, other: &Self, carry: &Self) -> (Self, Self) {
        // Compute the sum bit.
        let c0 = self ^ other;
        let sum = &c0 ^ carry;

        // Compute the carry bit.
        let c1 = self & other;
        let c2 = carry & c0;
        let carry = c1 | c2;

        (sum, carry)
    }
}
