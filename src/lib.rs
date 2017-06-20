#[macro_use] extern crate error_chain;

#[cfg(feature = "wavefront")]
extern crate tobj;

pub use self::errors::{Error, ErrorKind, ResultExt, Result};
pub use self::geometry::{Vertex, Vector, Triangle, Color};
pub use self::index::Index;
pub use self::model::{Model, TriangularMesh};

pub mod errors;
pub mod geometry;
pub mod index;
pub mod model;
pub mod load;
pub mod build;

