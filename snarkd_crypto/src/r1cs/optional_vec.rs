// A helper object containing a list of values that, when removed, leave a "hole" in their
// place; this allows all the following indices to remain unperturbed; the holes take priority
// when inserting new objects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionalVec<T> {
    // a list of optional values
    values: Vec<Option<T>>,
    // a list of indices of the Nones in the values vector
    holes: Vec<usize>,
}

impl<T> OptionalVec<T> {
    /// Creates a new `OptionalVec` with the given underlying capacity.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            values: Vec::with_capacity(cap),
            holes: Vec::with_capacity(cap),
        }
    }

    /// Inserts a new value either into the first existing hole or extending the vector
    /// of values, i.e. pushing it to its end.
    pub fn insert(&mut self, elem: T) -> usize {
        if let Some(idx) = self.holes.pop() {
            self.values[idx] = Some(elem);
            idx
        } else {
            self.values.push(Some(elem));
            self.values.len() - 1
        }
    }

    /// Returns the index that the next value inserted into the `OptionalVec` would have
    pub fn next_idx(&self) -> usize {
        self.holes.last().copied().unwrap_or(self.values.len())
    }

    /// Removes a value at the specified index; assumes that the index points to
    /// an existing value that is a `Some(T)` (i.e. not a hole).
    pub fn remove(&mut self, idx: usize) -> T {
        let val = self.values[idx].take();
        self.holes.push(idx);
        val.unwrap()
    }

    /// Iterates over all the `Some(T)` values in the list.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.values.iter().filter_map(|v| v.as_ref())
    }

    /// Returns the number of `Some(T)` values.
    pub fn len(&self) -> usize {
        self.values.len() - self.holes.len()
    }

    /// Returns `true` if there are no `Some(T)` values
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> std::ops::Index<usize> for OptionalVec<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        self.values[idx].as_ref().unwrap()
    }
}

impl<T> std::ops::IndexMut<usize> for OptionalVec<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        self.values[idx].as_mut().unwrap()
    }
}

impl<T> Default for OptionalVec<T> {
    fn default() -> Self {
        Self {
            values: Default::default(),
            holes: Default::default(),
        }
    }
}

#[macro_export]
macro_rules! optional_vec {
    ($($item:expr), *) => {{
        let mut _internal = OptionalVec::default();
        $(_internal.insert($item);)*
        _internal
    }};
}
