use snarkd_common::Digest32;

use super::{Deployment, Execution, Transition};

type TransactionID = Digest32;

#[derive(Clone, PartialEq, Eq)]
pub enum Transaction {
    Deploy(TransactionID, Deployment, Transition),
    Execute(TransactionID, Execution, Option<Transition>),
}
