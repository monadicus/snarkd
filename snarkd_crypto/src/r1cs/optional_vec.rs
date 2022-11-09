// TODO rewrite
// A helper object containing a list of values that, when removed, leave a "hole" in their
// place; this allows all the following indices to remain unperturbed; the holes take priority
// when inserting new objects.
pub struct OptionalVec<T> {
    // a list of optional values
    values: Vec<Option<T>>,
    // a list of indices of the Nones in the values vector
    holes: Vec<usize>,
}

impl<T> Default for OptionalVec<T> {
    fn default() -> Self {
        Self {
            values: Default::default(),
            holes: Default::default(),
        }
    }
}

impl<T> OptionalVec<T> {
    /// Creates a new `OptionalVec` with the given underlying capacity.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            values: Vec::with_capacity(cap),
            holes: Default::default(),
        }
    }

    /// Inserts a new value either into the first existing hole or extending the vector
    /// of values, i.e. pushing it to its end.
    pub fn insert(&mut self, elem: T) -> usize {
        panic!("unhit");
        let idx = self.holes.pop().unwrap_or(self.values.len());
        if idx < self.values.len() {
            self.values[idx] = Some(elem);
        } else {
            self.values.push(Some(elem));
        }
        idx
    }

    /// Returns the index of the next value inserted into the `OptionalVec`.
    pub fn next_idx(&self) -> usize {
        panic!("unhit");
        self.holes.last().copied().unwrap_or(self.values.len())
    }

    /// Removes a value at the specified index; assumes that the index points to
    /// an existing value that is a `Some(T)` (i.e. not a hole).
    pub fn remove(&mut self, idx: usize) -> T {
        panic!("unhit");
        let val = self.values[idx].take();
        self.holes.push(idx);
        val.unwrap()
    }

    /// Iterates over all the `Some(T)` values in the list.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        panic!("unhit");
        self.values.iter().filter_map(|v| v.as_ref())
    }

    /// Returns the number of `Some(T)` values.
    pub fn len(&self) -> usize {
        panic!("unhit");
        self.values.len() - self.holes.len()
    }

    /// Returns `true` if there are no `Some(T)` values
    pub fn is_empty(&self) -> bool {
        panic!("unhit");
        self.len() == 0
    }
}

impl<T> std::ops::Index<usize> for OptionalVec<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        panic!("unhit");
        self.values[idx].as_ref().unwrap()
    }
}

impl<T> std::ops::IndexMut<usize> for OptionalVec<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        panic!("unhit");
        self.values[idx].as_mut().unwrap()
    }
}
