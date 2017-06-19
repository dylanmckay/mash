#[macro_use] extern crate error_chain;
extern crate tobj;

pub use self::errors::{Error, ErrorKind, ResultExt, Result};
pub use self::vertex::{Vertex, Vector};

pub mod errors;
pub mod vertex;
pub mod load;

pub trait Index : From<u64> { }

impl Index for u64 { }

pub struct Model<V: Vertex, I: Index> {
    pub vertices: Vec<V>,
    pub indices: Vec<I>,
}

