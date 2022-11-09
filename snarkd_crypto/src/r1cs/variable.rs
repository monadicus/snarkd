use std::cmp::Ordering;

/// Represents the index of either a public variable (input) or a private variable (auxiliary).
#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
pub enum Index {
    /// Index of an public variable.
    Public(usize),
    /// Index of an private variable.
    Private(usize),
}

impl PartialOrd for Index {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Index {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Index::Public(ref idx1), Index::Public(ref idx2))
            | (Index::Private(ref idx1), Index::Private(ref idx2)) => idx1.cmp(idx2),
            (Index::Public(_), Index::Private(_)) => Ordering::Less,
            (Index::Private(_), Index::Public(_)) => Ordering::Greater,
        }
    }
}

/// Represents a variable in a constraint system.
#[derive(PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct Variable(Index);

impl Variable {
    /// This constructs a variable with an arbitrary index.
    /// Circuit implementations are not recommended to use this.
    pub fn new_unchecked(idx: Index) -> Variable {
        Variable(idx)
    }

    /// This returns the index underlying the variable.
    /// Circuit implementations are not recommended to use this.
    pub fn get_unchecked(&self) -> Index {
        self.0
    }
}
