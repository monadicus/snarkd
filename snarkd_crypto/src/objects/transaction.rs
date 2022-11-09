use snarkd_common::Digest32;

use super::{Deployment, Execution, Transition};

type TransactionID = Digest32;

#[derive(Clone, PartialEq, Eq)]
pub enum Transaction {
    Deploy(DeployTransaction),
    Execute(ExecuteTransaction),
}

#[derive(Clone, PartialEq, Eq)]
pub struct DeployTransaction {
    pub id: TransactionID,
    pub deployment: Box<Deployment>,
    /// Additional fee, used to pay for bytecode storage.
    pub transition: Transition,
}

#[derive(Clone, PartialEq, Eq)]
pub struct ExecuteTransaction {
    pub id: TransactionID,
    pub execution: Execution,
    /// Additional fee, used for executions which require some extra value to be added to the
    /// transaction.
    pub transition: Option<Transition>,
}
