use sha2::{Digest, Sha256};

pub fn sha256(data: &[u8]) -> [u8; 32] {
    let digest = Sha256::digest(data);
    let mut ret = [0u8; 32];
    ret.copy_from_slice(&digest);
    ret
}
