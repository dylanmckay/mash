#[macro_use] extern crate error_chain;

#[cfg(feature = "wavefront")]
extern crate tobj;

pub use self::errors::{Error, ErrorKind, ResultExt, Result};
pub use self::vertex::{Vertex, Vector, Color};
pub use self::model::{Model, TriangularMesh};

pub mod errors;
pub mod vertex;
pub mod model;
pub mod load;

pub trait Index : From<u64> { }

impl Index for u64 { }

