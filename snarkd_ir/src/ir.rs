use anyhow::Result;
use std::fmt;

include!(concat!(env!("OUT_DIR"), "/snarkd.ir.rs"));

pub trait ProtoBuf: fmt::Display {
    type Target;

    fn encode(&self) -> Self::Target;
    fn decode(target: Self::Target) -> Result<Self>
    where
        Self: Sized;
}
