use crate::{Digest32, Digest48};

// NOTE: placeholder
type Scalar = Digest32;
type Group = Digest48;
type Field = Digest32;
type VerifyingKey = Digest32;
type Certificate = Digest32;

type Instruction = ();
type LiteralType = ();

pub mod keys;
pub use keys::*;

mod block;
pub use block::*;
mod identifier;
pub use identifier::*;

mod closure;
pub use closure::*;
mod entry;
pub use entry::*;
mod entry_type;
pub use entry_type::*;
mod finalize_type;
pub use finalize_type::*;
mod function;
pub use function::*;
mod input;
pub use input::*;
mod interface;
pub use interface::*;
mod locator;
pub use locator::*;
mod map_object;
pub use map_object::*;
mod mapping;
pub use mapping::*;
mod origin;
pub use origin::*;
mod output;
pub use output::*;
mod plaintext_type;
pub use plaintext_type::*;
mod program;
pub use program::*;
mod program_id;
pub use program_id::*;
mod record;
pub use record::*;
mod record_type;
pub use record_type::*;
mod register;
pub use register::*;
mod register_type;
pub use register_type::*;
mod transaction;
pub use transaction::*;
mod transition;
pub use transition::*;
mod value;
pub use value::*;
mod value_entry;
pub use value_entry::*;
mod value_type;
pub use value_type::*;
