use snarkd_common::Digest;
use anyhow::Result;
use snarkd_crypto::bls12_377::Fp;

use super::{hash_leaf, hash_inner_node};

/// Stores the hashes of a particular path (in order) from leaf to root.
/// Our path `is_left_child()` if the boolean in `path` is true.
#[derive(Clone, Debug)]
pub struct MerklePath {
    pub path: Vec<(Digest, Digest)>,
}

impl MerklePath {
    pub fn verify(&self, depth: usize, root_hash: &Digest, leaf: Fp) -> Result<bool> {
        if self.path.len() != depth {
            return Ok(false);
        }

        // Check that the given leaf matches the leaf in the membership proof.
        if !self.path.is_empty() {
            let claimed_leaf_hash = hash_leaf(leaf)?;

            // Check if leaf is one of the bottom-most siblings.
            if claimed_leaf_hash != self.path[0].0 && claimed_leaf_hash != self.path[0].1 {
                return Ok(false);
            };

            // Check levels between leaf level and root.
            let mut previous_hash = claimed_leaf_hash;
            for &(ref hash, ref sibling_hash) in &self.path {
                // Check if the previous hash matches the correct current hash.
                if &previous_hash != hash && &previous_hash != sibling_hash {
                    return Ok(false);
                };
                previous_hash = hash_inner_node(hash, sibling_hash)?;
            }

            if root_hash != &previous_hash {
                return Ok(false);
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn with_depth(depth: usize) -> Self {
        let mut path = Vec::with_capacity(depth);
        for _ in 0..depth {
            path.push((Digest::default(), Digest::default()));
        }
        Self {
            path,
        }

    }
}
