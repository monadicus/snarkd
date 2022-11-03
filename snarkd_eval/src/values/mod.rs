mod address;
mod boolean;
mod field;
mod group;
mod integers;
mod record;
mod scalar;
mod string;
mod structure;
mod value;

pub use address::*;
pub use boolean::*;
pub use field::*;
pub use group::*;
pub use integers::*;
pub use record::*;
pub use scalar::*;
pub use string::*;
pub use structure::*;
pub use value::*;

use snarkd_crypto::*;
use snarkd_ir::*;

/*
Address(Address),
Boolean(bool),
Field(Field),
Group(Group),
Integer(Integer),
Struct(Vec<Value>),
Str(String),
Ref(u32), // reference to a variable
Scalar(Vec<u64>),
Record(Record),
*/
