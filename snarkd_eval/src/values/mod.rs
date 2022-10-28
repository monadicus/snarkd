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

use address::*;
use boolean::*;
use field::*;
use group::*;
use integers::*;
use record::*;
use scalar::*;
use string::*;
use structure::*;
use value::*;

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
