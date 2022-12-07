use crate::circuit::types::Boolean;

/// Unary operator for converting to bits.
pub trait ToBits {
    /// Returns the little-endian bits of the circuit.
    fn to_bits_le(&self) -> Vec<Boolean>;

    /// Returns the big-endian bits of the circuit.
    fn to_bits_be(&self) -> Vec<Boolean>;
}

/********************/
/****** Arrays ******/
/********************/

impl<C: ToBits> ToBits for Vec<C> {
    /// A helper method to return a concatenated list of little-endian bits from the circuits.
    #[inline]
    fn to_bits_le(&self) -> Vec<Boolean> {
        // The vector is order-preserving, meaning the first circuit in is the first circuit bits out.
        self.as_slice().to_bits_le()
    }

    /// A helper method to return a concatenated list of big-endian bits from the circuits.
    #[inline]
    fn to_bits_be(&self) -> Vec<Boolean> {
        // The vector is order-preserving, meaning the first circuit in is the first circuit bits out.
        self.as_slice().to_bits_be()
    }
}

impl<C: ToBits, const N: usize> ToBits for [C; N] {
    /// A helper method to return a concatenated list of little-endian bits from the circuits.
    #[inline]
    fn to_bits_le(&self) -> Vec<Boolean> {
        // The slice is order-preserving, meaning the first circuit in is the first circuit bits out.
        self.as_slice().to_bits_le()
    }

    /// A helper method to return a concatenated list of big-endian bits from the circuits.
    #[inline]
    fn to_bits_be(&self) -> Vec<Boolean> {
        // The slice is order-preserving, meaning the first circuit in is the first circuit bits out.
        self.as_slice().to_bits_be()
    }
}

impl<C: ToBits> ToBits for &[C] {
    /// A helper method to return a concatenated list of little-endian bits from the circuits.
    #[inline]
    fn to_bits_le(&self) -> Vec<Boolean> {
        // The slice is order-preserving, meaning the first circuit in is the first circuit bits out.
        self.iter().flat_map(|c| c.to_bits_le()).collect()
    }

    /// A helper method to return a concatenated list of big-endian bits from the circuits.
    #[inline]
    fn to_bits_be(&self) -> Vec<Boolean> {
        // The slice is order-preserving, meaning the first circuit in is the first circuit bits out.
        self.iter().flat_map(|c| c.to_bits_be()).collect()
    }
}

/********************/
/****** Tuples ******/
/********************/

/// A helper macro to implement `ToBits` for a tuple of `ToBits` circuits.
macro_rules! to_bits_tuple {
    (($t0:ident, $i0:tt), $(($ty:ident, $idx:tt)),+) => {
        impl<$t0: ToBits, $($ty: ToBits),+> ToBits for ($t0, $($ty),+) {
            /// A helper method to return a concatenated list of little-endian bits from the circuits.
            #[inline]
            fn to_bits_le(&self) -> Vec<Boolean> {
                // The tuple is order-preserving, meaning the first circuit in is the first circuit bits out.
                self.$i0.to_bits_le().into_iter()
                    $(.chain(self.$idx.to_bits_le().into_iter()))+
                    .collect()
            }

            /// A helper method to return a concatenated list of big-endian bits from the circuits.
            #[inline]
            fn to_bits_be(&self) -> Vec<Boolean> {
                // The tuple is order-preserving, meaning the first circuit in is the first circuit bits out.
                self.$i0.to_bits_be().into_iter()
                    $(.chain(self.$idx.to_bits_be().into_iter()))+
                    .collect()
            }
        }

        impl<'a, $t0: ToBits, $($ty: ToBits),+> ToBits for &'a ($t0, $($ty),+) {
            /// A helper method to return a concatenated list of little-endian bits from the circuits.
            #[inline]
            fn to_bits_le(&self) -> Vec<Boolean> {
                // The tuple is order-preserving, meaning the first circuit in is the first circuit bits out.
                self.$i0.to_bits_le().into_iter()
                    $(.chain(self.$idx.to_bits_le().into_iter()))+
                    .collect()
            }

            /// A helper method to return a concatenated list of big-endian bits from the circuits.
            #[inline]
            fn to_bits_be(&self) -> Vec<Boolean> {
                // The tuple is order-preserving, meaning the first circuit in is the first circuit bits out.
                self.$i0.to_bits_be().into_iter()
                    $(.chain(self.$idx.to_bits_be().into_iter()))+
                    .collect()
            }
        }
    }
}

to_bits_tuple!((C0, 0), (C1, 1));
to_bits_tuple!((C0, 0), (C1, 1), (C2, 2));
to_bits_tuple!((C0, 0), (C1, 1), (C2, 2), (C3, 3));
to_bits_tuple!((C0, 0), (C1, 1), (C2, 2), (C3, 3), (C4, 4));
to_bits_tuple!((C0, 0), (C1, 1), (C2, 2), (C3, 3), (C4, 4), (C5, 5));
to_bits_tuple!(
    (C0, 0),
    (C1, 1),
    (C2, 2),
    (C3, 3),
    (C4, 4),
    (C5, 5),
    (C6, 6)
);
to_bits_tuple!(
    (C0, 0),
    (C1, 1),
    (C2, 2),
    (C3, 3),
    (C4, 4),
    (C5, 5),
    (C6, 6),
    (C7, 7)
);
to_bits_tuple!(
    (C0, 0),
    (C1, 1),
    (C2, 2),
    (C3, 3),
    (C4, 4),
    (C5, 5),
    (C6, 6),
    (C7, 7),
    (C8, 8)
);
to_bits_tuple!(
    (C0, 0),
    (C1, 1),
    (C2, 2),
    (C3, 3),
    (C4, 4),
    (C5, 5),
    (C6, 6),
    (C7, 7),
    (C8, 8),
    (C9, 9)
);
to_bits_tuple!(
    (C0, 0),
    (C1, 1),
    (C2, 2),
    (C3, 3),
    (C4, 4),
    (C5, 5),
    (C6, 6),
    (C7, 7),
    (C8, 8),
    (C9, 9),
    (C10, 10)
);
