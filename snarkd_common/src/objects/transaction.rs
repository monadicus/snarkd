use crate::Digest32;

use super::{Certificate, Identifier, Program, Transition, VerifyingKey};

pub type TransactionID = Digest32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Transaction {
    Deploy(DeployTransaction),
    Execute(ExecuteTransaction),
}

impl Transaction {
    pub fn id(&self) -> &TransactionID {
        match self {
            Transaction::Deploy(DeployTransaction { id, .. }) => id,
            Transaction::Execute(ExecuteTransaction { id, .. }) => id,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Transaction::Deploy(x) => x.size(),
            Transaction::Execute(x) => x.size(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeployTransaction {
    pub id: TransactionID,
    pub deployment: Deployment,
    /// Additional fee, used to pay for bytecode storage.
    pub transition: Transition,
}

impl DeployTransaction {
    pub fn size(&self) -> usize {
        self.id.len() + std::mem::size_of::<Deployment>() + self.deployment.program.len() + self.transition.size()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecuteTransaction {
    pub id: TransactionID,
    pub execution: Execution,
    /// Additional fee, used for executions which require some extra value to be added to the
    /// transaction.
    pub transition: Option<Transition>,
}

impl ExecuteTransaction {
    pub fn size(&self) -> usize {
        self.id.len() + std::mem::size_of::<Execution>() + self.execution.transitions.iter().map(Transition::size).sum::<usize>() + self.transition.as_ref().map(|x| x.size()).unwrap_or_default()
    }
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
