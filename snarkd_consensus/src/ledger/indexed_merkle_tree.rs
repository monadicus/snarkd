
use crate::ledger::IndexedDigests;
use anyhow::Result;
use snarkd_common::Digest;
use snarkd_crypto::bls12_377::Fp;

use super::merkle_tree::MerkleTree;

pub struct IndexedMerkleTree {
    tree: MerkleTree,
    indexed_digests: IndexedDigests,
}

fn to_digest<B: ToBytes>(input: &B) -> Result<Digest> {
    let mut data = vec![];
    input.write_le(&mut data)?;
    Ok((&data[..]).into())
}

impl IndexedMerkleTree {
    pub fn new(depth: usize, leaves: impl IntoIterator<Item=Fp>) -> Result<Self> {
        Ok(Self {
            tree: MerkleTree::new(depth, leaves)?,
            indexed_digests: IndexedDigests::new(leaves),
        })
    }

    pub fn extend(&mut self, new_leaves: &[Digest]) -> Result<()> {
        let tree = self.tree.rebuild(self.indexed_digests.len(), new_leaves)?;
        self.tree = tree;
        self.indexed_digests.extend(new_leaves);
        Ok(())
    }

    /// pop leafs from the interior merkle tree, and assert they are equal to `to_remove`.
    pub fn pop(&mut self, to_remove: &[Digest]) -> Result<()> {
        if to_remove.len() > self.indexed_digests.len() {
            return Err(anyhow!(
                "attempted to remove more items from indexed merkle tree than present"
            ));
        }
        self.indexed_digests.pop(to_remove)?;
        let tree = self.tree.rebuild::<[u8; 32]>(self.indexed_digests.len(), &[])?;
        self.tree = tree;

        Ok(())
    }

    pub fn clear(&mut self) {
        self.indexed_digests.clear();
        self.tree = self.tree.rebuild::<[u8; 32]>(0, &[]).unwrap();
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
        self.indexed_digests.index(leaf)
    }

    pub fn digest(&self) -> Digest {
        let mut out = vec![];
        self.tree.root().write_le(&mut out).expect("failed to digest root");
        (&out[..]).into()
    }

    pub fn generate_proof(&self, commitment: &Digest, index: usize) -> Result<Vec<(Digest, Digest)>> {
        let path = self.tree.generate_proof(index, commitment)?;
        path.path
            .into_iter()
            .map(|(p1, p2)| Ok((to_digest(&p1)?, to_digest(&p2)?)))
            .collect::<Result<Vec<_>>>()
    }
}
