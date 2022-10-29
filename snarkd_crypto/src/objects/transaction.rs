use super::{Deployment, Execution, Transition};

type TransactionID = [u8; 32];

#[derive(Clone, PartialEq, Eq)]
pub enum Transaction {
    Deploy(TransactionID, Deployment, Transition),
    Execute(TransactionID, Execution, Option<Transition>),
}
