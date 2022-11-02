use error_stack::Report;
use thiserror::Error;

mod create_error_type;
mod error_msg;
pub use error_msg::*;
mod ir;
pub use ir::*;
mod network;
pub use network::*;
mod suggestion;
pub use suggestion::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0:?}")]
    IRError(Report<IRError>),

    #[error("{0:?}")]
    NetworkError(Report<NetworkError>),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub trait IntoSnarkdError<Err>: Sized {
    type Ok;
    type InitErr;

    fn to_error<O>(self, op: O) -> core::result::Result<Self::Ok, Err>
    where
        O: FnOnce(Self) -> core::result::Result<Self::Ok, Err>;
}

impl<Ok, InitErr, Err> IntoSnarkdError<Err> for core::result::Result<Ok, InitErr> {
    type Ok = Ok;
    type InitErr = InitErr;

    fn to_error<O>(self, op: O) -> core::result::Result<Ok, Err>
    where
        O: FnOnce(Self) -> core::result::Result<Ok, Err>,
    {
        op(self)
    }
}

#[cfg(test)]
mod test {
    fn test() {
        todo!("write tests")
    }
}
