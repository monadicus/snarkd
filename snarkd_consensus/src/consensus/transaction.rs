
use snarkd_common::objects::ExecuteTransaction;

use super::*;

impl Consensus {
    pub async fn receive_transaction(&mut self, transaction: Transaction) -> bool {
        return false;
    }

    /// Check if the transactions are valid. (coinbase only)
    pub async fn verify_transactions(
        &mut self,
        transactions: &[Transaction],
    ) -> Result<bool> {
        if transactions.len() != 1 {
            return Ok(false);
        }
        for transaction in transactions {
            match transaction {
                Transaction::Deploy(_) => return Ok(false),
                Transaction::Execute(ExecuteTransaction { id, execution, transition }) => {
                    if !execution.transitions.is_empty() {
                        return Ok(false);
                    }
                    match transition {
                        None => return Ok(false),
                        Some(transition) => {
                            if transition.fee < 0 {
                                return Ok(false);
                            }
                        }
                    }
                },
            }
        }
        Ok(true)
    }

}
