use crate::ledger::Ledger;

use anyhow::Result;
use snarkd_common::Digest;

/// This object only serves as a temporary replacement for the regular Ledger so that it can be sent to a blocking task.
pub(crate) struct DummyLedger;

impl Ledger for DummyLedger {
    fn extend(&mut self, _new_cms: &[Digest], _new_sns: &[Digest], _new_memos: &[Digest]) -> Result<Digest> {
        unimplemented!()
    }

    fn push_interim_digests(&mut self, _new_ledger_digests: &[Digest]) -> Result<()> {
        unimplemented!()
    }

    fn rollback(&mut self, _commitments: &[Digest], _serial_numbers: &[Digest], _memos: &[Digest]) -> Result<()> {
        unimplemented!()
    }

    fn clear(&mut self) {
        unimplemented!()
    }

    fn commitment_len(&self) -> usize {
        unimplemented!()
    }

    fn contains_commitment(&self, _commitment: &Digest) -> bool {
        unimplemented!()
    }

    fn commitment_index(&self, _commitment: &Digest) -> Option<usize> {
        unimplemented!()
    }

    fn contains_serial(&self, _serial: &Digest) -> bool {
        unimplemented!()
    }

    fn contains_memo(&self, _memo: &Digest) -> bool {
        unimplemented!()
    }

    fn validate_digest(&self, _digest: &Digest) -> bool {
        unimplemented!()
    }

    fn digest(&self) -> Digest {
        unimplemented!()
    }

    fn generate_proof(&self, _commitment: &Digest, _index: usize) -> Result<Vec<(Digest, Digest)>> {
        unimplemented!()
    }

    fn validate_ledger(&self) -> bool {
        unimplemented!()
    }

    fn requires_async_task(&self, _new_commitments_len: usize, _new_serial_numbers_len: usize) -> bool {
        unimplemented!()
    }
}
