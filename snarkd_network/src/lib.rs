pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/snarkd.rs"));
}

mod handler;
pub use handler::*;

mod connection;
pub use connection::*;

mod util;
