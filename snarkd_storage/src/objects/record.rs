use snarkd_common::Digest;

use super::Address;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerialRecord {
    pub owner: Address,
    pub is_dummy: bool,
    pub value: i64,
    pub payload: Digest,
    pub birth_program_id: Digest,
    pub death_program_id: Digest,
    pub serial_number_nonce: Digest,
    pub commitment: Digest,
    pub commitment_randomness: Digest,
}
