use crate::Digest;

// do not implement clone: this structure can be too deep, resulting in stack overflow
#[derive(Debug)]
pub enum DigestTree {
    // digest of leaf node
    Leaf(Digest),
    // digest and subtree of node, length of longest chain not including node
    Node(Digest, Vec<DigestTree>, usize),
}

impl DigestTree {
    pub fn root(&self) -> &Digest {
        match self {
            DigestTree::Leaf(root) => root,
            DigestTree::Node(root, _, _) => root,
        }
    }

    pub fn longest_length(&self) -> usize {
        match self {
            DigestTree::Leaf(_) => 1,
            DigestTree::Node(_, _, n) => *n + 1,
        }
    }

    pub fn unified_chain(&self) -> Option<Vec<&Digest>> {
        let mut out = vec![];
        let mut current_node = self;
        loop {
            match current_node {
                DigestTree::Leaf(hash) => {
                    out.push(hash);
                    break;
                }
                DigestTree::Node(hash, children, _) => {
                    if children.len() != 1 {
                        return None;
                    }
                    out.push(hash);
                    current_node = &children[0];
                }
            }
        }
        Some(out)
    }

    pub fn children(&self) -> &[DigestTree] {
        match self {
            DigestTree::Leaf(_) => &[],
            DigestTree::Node(_, children, _) => &children[..],
        }
    }

    pub fn take_children(self) -> Vec<DigestTree> {
        match self {
            DigestTree::Leaf(_) => vec![],
            DigestTree::Node(_, children, _) => children,
        }
    }
}
