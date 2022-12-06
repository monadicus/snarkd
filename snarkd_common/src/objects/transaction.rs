use crate::Digest32;

use super::{Certificate, Identifier, Program, Transition, VerifyingKey};

type TransactionID = Digest32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Transaction {
    Deploy(DeployTransaction),
    Execute(ExecuteTransaction),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeployTransaction {
    pub id: TransactionID,
    pub deployment: Deployment,
    /// Additional fee, used to pay for bytecode storage.
    pub transition: Transition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecuteTransaction {
    pub id: TransactionID,
    pub execution: Execution,
    /// Additional fee, used for executions which require some extra value to be added to the
    /// transaction.
    pub transition: Option<Transition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deployment {
    pub edition: u16,
    pub program: Program,
    pub verifying_key_id: Identifier,
    pub verifying_key: VerifyingKey,
    pub certificate: Certificate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Execution {
    pub edition: u16,
    pub transitions: Vec<Transition>,
}
