pub mod proto {
    #![allow(clippy::derive_partial_eq_without_eq)]
    include!(concat!(env!("OUT_DIR"), "/snarkd.rs"));
}

mod handler;
pub use handler::*;

mod connection;
pub use connection::*;

mod util;
