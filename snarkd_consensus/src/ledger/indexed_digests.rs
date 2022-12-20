use anyhow::*;
use indexmap::IndexSet;
use snarkd_common::Digest;

#[derive(Clone)]
pub struct IndexedDigests {
    indexed_digests: IndexSet<Digest>,
}

impl IndexedDigests {
    pub fn new(leaves: &[Digest]) -> Self {
        Self {
            indexed_digests: leaves.iter().cloned().collect(),
        }
    }

    pub fn extend(&mut self, new_leaves: &[Digest]) {
        self.indexed_digests.extend(new_leaves.iter().cloned());
    }

    /// pop leafs from the interior merkle tree, and assert they are equal to `to_remove`.
    pub fn pop(&mut self, to_remove: &[Digest]) -> Result<()> {
        if to_remove.len() > self.indexed_digests.len() {
            return Err(anyhow!(
                "attempted to remove more items from indexed digests set than present"
            ));
        }
        let old_length = self.indexed_digests.len() - to_remove.len();
        for i in old_length..self.indexed_digests.len() {
            if self.indexed_digests[i] != to_remove[i - old_length] {
                return Err(anyhow!(
                    "mismatch in attempted pop of indexed digests @ {}: {} != {}",
                    i,
                    self.indexed_digests[i],
                    to_remove[i - old_length]
                ));
            }
        }
        self.indexed_digests.truncate(old_length);

        Ok(())
    }

    pub fn clear(&mut self) {
        self.indexed_digests.clear();
    }

    pub fn len(&self) -> usize {
        self.indexed_digests.len()
    }

    pub fn is_empty(&self) -> bool {
        self.indexed_digests.is_empty()
    }

    pub fn contains(&self, leaf: &Digest) -> bool {
        self.indexed_digests.contains(leaf)
    }

    pub fn index(&self, leaf: &Digest) -> Option<usize> {
        self.indexed_digests.get_index_of(leaf)
    }
}
