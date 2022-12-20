
use anyhow::bail;
use rayon::prelude::*;
use snarkd_crypto::{cfg_chunks, POSEIDON, bls12_377::Fp};

use super::MerklePath;

#[derive(Default)]
pub struct MerkleTree {
    depth: usize,

    /// The computed root of the full Merkle tree.
    root: Option<Fp>,

    /// The internal hashes, from root to hashed leaves, of the full Merkle tree.
    tree: Vec<Fp>,

    /// The index from which hashes of each non-empty leaf in the Merkle tree can be obtained.
    hashed_leaves_index: usize,

    /// For each level after a full tree has been built from the leaves,
    /// keeps both the roots the siblings that are used to get to the desired depth.
    padding_tree: Vec<(Fp, Fp)>,
}

/// Returns the hash of a given leaf.
pub(super) fn hash_leaf(leaf: Fp) -> Fp {
    POSEIDON.evaluate(&[leaf])
}

/// Returns the output hash, given a left and right hash value.
pub(super) fn hash_inner_node(
    left: Fp,
    right: Fp,
) -> Fp {
    POSEIDON.evaluate(&[left, right])
}

lazy_static::lazy_static! {
    static ref HASH_EMPTY: Fp = POSEIDON.evaluate(&[]);
}

impl MerkleTree {
    fn hash_row(
        leaves: impl IntoIterator<Item=Fp>,
    ) -> Result<Vec<Vec<Fp>>> {
        cfg_chunks!(leaves, 500) // arbitrary, experimentally derived
            .map(|chunk| -> Result<Vec<_>> {
                let mut out = Vec::with_capacity(chunk.len());
                for leaf in chunk.into_iter() {
                    out.push(hash_leaf(leaf));
                }
                Ok(out)
            })
            .collect::<Result<Vec<_>>>()
    }

    pub fn new(depth: usize, leaves: impl IntoIterator<Item=Fp>) -> Result<Self> {
        let last_level_size = leaves.len().next_power_of_two();
        let tree_size = 2 * last_level_size - 1;
        let tree_depth = tree_depth(tree_size);

        if tree_depth > depth as usize {
            bail!("invalid tree depth, {tree_depth} > {depth}");
        }

        // Initialize the Merkle tree.
        let mut tree = vec![*HASH_EMPTY; tree_size];

        // Compute the starting index (on the left) for each level of the tree.
        let mut index = 0;
        let mut level_indices = Vec::with_capacity(tree_depth);
        for _ in 0..=tree_depth {
            level_indices.push(index);
            index = left_child(index);
        }

        // Compute and store the hash values for each leaf.
        let last_level_index = level_indices.pop().unwrap_or(0);

        let subsections = Self::hash_row(leaves)?;

        let mut subsection_index = 0;
        for subsection in subsections.into_iter() {
            tree[last_level_index + subsection_index..last_level_index + subsection_index + subsection.len()]
                .copy_from_slice(&subsection[..]);
            subsection_index += subsection.len();
        }

        // Compute the hash values for every node in the tree.
        let mut upper_bound = last_level_index;
        level_indices.reverse();
        for &start_index in &level_indices {
            // Iterate over the current level.
            let hashings = (start_index..upper_bound)
                .map(|i| (&tree[left_child(i)], &tree[right_child(i)]))
                .collect::<Vec<_>>();

            let hashes = Self::hash_row(&hashings[..])?;

            let mut subsection_index = 0;
            for subsection in hashes.into_iter() {
                tree[start_index + subsection_index..start_index + subsection_index + subsection.len()]
                    .copy_from_slice(&subsection[..]);
                subsection_index += subsection.len();
            }

            upper_bound = start_index;
        }

        // Finished computing actual tree.
        // Now, we compute the dummy nodes until we hit our DEPTH goal.
        let mut current_depth = tree_depth;
        let mut padding_tree = Vec::with_capacity((depth as usize).saturating_sub(current_depth + 1));
        let mut current_hash = tree[0].clone();
        while current_depth < depth as usize {
            current_hash = hash_inner_node(&current_hash, *HASH_EMPTY)?;

            // do not pad at the top-level of the tree
            if current_depth < depth as usize - 1 {
                padding_tree.push((current_hash.clone(), *HASH_EMPTY));
            }
            current_depth += 1;
        }
        let root_hash = current_hash;

        Ok(MerkleTree {
            tree,
            padding_tree,
            hashed_leaves_index: last_level_index,
            root: Some(root_hash),
            depth,
        })
    }

    pub fn rebuild(&self, start_index: usize, new_leaves: &[&[u8]]) -> Result<Self> {
        let last_level_size = (start_index + new_leaves.len()).next_power_of_two();
        let tree_size = 2 * last_level_size - 1;
        let tree_depth = tree_depth(tree_size);

        if tree_depth > self.depth as usize {
            bail!("invalid tree depth, {tree_depth} > {}", self.depth);
        }

        // Initialize the Merkle tree.
        let mut tree = vec![*HASH_EMPTY; tree_size];

        // Compute the starting index (on the left) for each level of the tree.
        let mut index = 0;
        let mut level_indices = Vec::with_capacity(tree_depth);
        for _ in 0..=tree_depth {
            level_indices.push(index);
            index = left_child(index);
        }

        // Track the indices of newly added leaves.
        let new_indices = (start_index..start_index + new_leaves.len()).collect::<Vec<_>>();

        // Compute and store the hash values for each leaf.
        let last_level_index = level_indices.pop().unwrap_or(0);

        // The beginning of the tree can be reconstructed from pre-existing hashed leaves.
        tree[last_level_index..][..start_index].clone_from_slice(&self.hashed_leaves()[..start_index]);

        // The new leaves require hashing.
        let subsections = Self::hash_row(&*self.parameters, new_leaves)?;

        for (i, subsection) in subsections.into_iter().enumerate() {
            tree[last_level_index + start_index + i..last_level_index + start_index + i + subsection.len()]
                .copy_from_slice(&subsection[..]);
        }

        // Compute the hash values for every node in the tree.
        let mut upper_bound = last_level_index;
        level_indices.reverse();
        for &start_index in &level_indices {
            // Iterate over the current level.
            for current_index in start_index..upper_bound {
                let left_index = left_child(current_index);
                let right_index = right_child(current_index);

                // Hash only the tree paths that are altered by the addition of new leaves or are brand new.
                if new_indices.contains(&current_index)
                    || self.tree.get(left_index) != tree.get(left_index)
                    || self.tree.get(right_index) != tree.get(right_index)
                    || new_indices
                        .iter()
                        .any(|&idx| Ancestors(idx).into_iter().find(|&i| i == current_index).is_some())
                {
                    // Compute Hash(left || right).
                    tree[current_index] = hash_inner_node(&tree[left_index], &tree[right_index])?;
                } else {
                    tree[current_index] = self.tree[current_index].clone();
                }
            }
            upper_bound = start_index;
        }

        // Finished computing actual tree.
        // Now, we compute the dummy nodes until we hit our DEPTH goal.
        let mut current_depth = tree_depth;
        let mut current_hash = tree[0].clone();

        // The whole padding tree can be reused if the current hash matches the previous one.
        let new_padding_tree = if current_hash == self.tree[0] {
            current_hash =
                hash_inner_node(&self.padding_tree.last().unwrap().0, *HASH_EMPTY)?;

            None
        } else {
            let mut padding_tree = Vec::with_capacity((Self::DEPTH as usize).saturating_sub(current_depth + 1));

            while current_depth < Self::DEPTH as usize {
                hash_inner_node(&current_hash, *HASH_EMPTY)?;

                // do not pad at the top-level of the tree
                if current_depth < Self::DEPTH as usize - 1 {
                    padding_tree.push((current_hash.clone(), *HASH_EMPTY));
                }
                current_depth += 1;
            }

            Some(padding_tree)
        };
        let root_hash = current_hash;

        // update the values at the very end so the original tree is not altered in case of failure
        Ok(MerkleTree {
            root: Some(root_hash),
            tree,
            hashed_leaves_index: last_level_index,
            padding_tree: if let Some(padding_tree) = new_padding_tree {
                padding_tree
            } else {
                self.padding_tree.clone()
            },
            depth: self.depth,
        })
    }

    pub fn root(&self) -> Fp {
        self.root.clone().unwrap()
    }

    pub fn tree(&self) -> &[Fp] {
        &self.tree
    }

    pub fn hashed_leaves(&self) -> &[Fp] {
        &self.tree[self.hashed_leaves_index..]
    }

    pub fn generate_proof(&self, index: usize, leaf: Fp) -> Result<MerklePath> {
        let mut path = vec![];

        let leaf_hash = hash_leaf(leaf);

        let tree_depth = tree_depth(self.tree.len());
        let tree_index = convert_index_to_last_level(index, tree_depth);

        // Check that the given index corresponds to the correct leaf.
        if leaf_hash != self.tree[tree_index] {
            bail!("incorrect leaf index, {leaf_hash} != {}", self.tree[tree_index]);
        }

        // Iterate from the leaf up to the root, storing all intermediate hash values.
        let mut current_node = tree_index;
        while !is_root(current_node) {
            let sibling_node = sibling(current_node).unwrap();
            let (curr_hash, sibling_hash) = (self.tree[current_node].clone(), self.tree[sibling_node].clone());
            if is_left_child(current_node) {
                path.push((curr_hash, sibling_hash));
            } else {
                path.push((sibling_hash, curr_hash));
            }
            current_node = parent(current_node).unwrap();
        }

        // Store the root node. Set boolean as true for consistency with digest location.
        if path.len() > self.depth as usize {
            bail!("invalid path length, {} > {}", path.len(), self.depth);
        }

        if path.len() != self.depth as usize {
            let empty_hash = *HASH_EMPTY;
            path.push((self.tree[0].clone(), empty_hash));

            for &(ref hash, ref sibling_hash) in &self.padding_tree {
                path.push((hash.clone(), sibling_hash.clone()));
            }
        }

        if path.len() != self.depth as usize {
            bail!("incorrect path length, {} != {}", path.len(), self.depth);
        } else {
            Ok(MerklePath {
                path,
            })
        }
    }
}

/// Returns the depth of the tree, given the size of the tree.
#[inline]
fn tree_depth(tree_size: usize) -> usize {
    // Returns the log2 value of the given number.
    fn log2(number: usize) -> usize {
        (number as f64).log2() as usize
    }

    log2(tree_size)
}

/// Returns true iff the index represents the root.
#[inline]
fn is_root(index: usize) -> bool {
    index == 0
}

/// Returns the index of the left child, given an index.
#[inline]
fn left_child(index: usize) -> usize {
    2 * index + 1
}

/// Returns the index of the right child, given an index.
#[inline]
fn right_child(index: usize) -> usize {
    2 * index + 2
}

/// Returns the index of the sibling, given an index.
#[inline]
fn sibling(index: usize) -> Option<usize> {
    if index == 0 {
        None
    } else if is_left_child(index) {
        Some(index + 1)
    } else {
        Some(index - 1)
    }
}

/// Returns true iff the given index represents a left child.
#[inline]
fn is_left_child(index: usize) -> bool {
    index % 2 == 1
}

/// Returns the index of the parent, given an index.
#[inline]
fn parent(index: usize) -> Option<usize> {
    if index > 0 { Some((index - 1) >> 1) } else { None }
}

#[inline]
fn convert_index_to_last_level(index: usize, tree_depth: usize) -> usize {
    index + (1 << tree_depth) - 1
}

pub struct Ancestors(usize);

impl Iterator for Ancestors {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if let Some(parent) = parent(self.0) {
            self.0 = parent;
            Some(parent)
        } else {
            None
        }
    }
}
