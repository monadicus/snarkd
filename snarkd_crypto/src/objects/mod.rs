// NOTE: placeholder
type Scalar = Digest32;
type Group = Digest48;
type Field = Digest32;
type VerifyingKey = Digest32;
type Certificate = Digest32;

type Identifier = (Field, u8);

type Instruction = ();
type LiteralType = ();

mod address;
mod block;
use block::*;
mod block_header;
use block_header::*;
mod closure;
use closure::*;
mod compute_key;
use compute_key::*;
mod deployment;
use deployment::*;
mod entry;
use entry::*;
mod entry_type;
use entry_type::*;
mod execution;
use execution::*;
mod finalize_type;
use finalize_type::*;
mod function;
use function::*;
mod graph_key;
mod input;
use input::*;
mod interface;
use interface::*;
mod locator;
use locator::*;
mod map_object;
use map_object::*;
mod mapping;
use mapping::*;
mod metadata;
use metadata::*;
mod origin;
use origin::*;
mod output;
use output::*;
mod plaintext_type;
use plaintext_type::*;
mod private_key;
mod program;
use program::*;
mod program_id;
use program_id::*;
mod record;
use record::*;
mod record_type;
use record_type::*;
mod register;
use register::*;
mod register_type;
use register_type::*;
mod signature;
use signature::*;
mod transaction;
use snarkd_common::{Digest32, Digest48};
use transaction::*;
mod transition;
use transition::*;
mod value;
mod value_entry;
use value_entry::*;
mod value_type;
use value_type::*;
mod view_key;
use value::*;