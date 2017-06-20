//! 3d mesh library.
//!
//! # Loading meshes
//!
//! ```
//! use mash::{load, Model, Vector};
//! use mash::load::wavefront;
//! use std::cmp::PartialOrd;
//! use std::path::Path;
//!
//! type Index = u16;
//!
//! #[derive(Clone, Debug, PartialEq, PartialOrd)]
//! pub struct Vertex {
//!     pub position: Vector,
//!     // .. more fields
//! }
//!
//! impl mash::Vertex for Vertex {
//!     fn position(&self) -> Vector { self.position }
//! }
//!
//! impl From<wavefront::Vertex> for Vertex {
//!     fn from(v: wavefront::Vertex) -> Self {
//!         Vertex { position: v.position }
//!     }
//! }
//!
//! let model: Model<Vertex, Index> = Model::new(load::wavefront::from_path("res/cube.obj").unwrap()).unwrap();
//! ```
//!
//! # Preprocessing meshes
//!
//! ```
//! use mash::{load, Vector};
//!
//! type Model = mash::Model<Vector, u32>;
//!
//! let wavefront = load::wavefront::from_path("res/lighthouse.obj").unwrap();
//!
//! let pylon_obj = wavefront.objects().find(|o| o.name() == "pylon_rectangle").unwrap();
//! let light_obj = wavefront.objects().find(|o| o.name() == "rotating_lights_cylinder").unwrap();
//!
//! // Can render the two models separately, with different transforms if necessary.
//! let pylon: Model = Model::new(pylon_obj).unwrap();
//! let lights: Model = Model::new(light_obj).unwrap();
//! ```

#[macro_use] extern crate error_chain;

#[cfg(feature = "wavefront")]
extern crate tobj;

pub use self::errors::{Error, ErrorKind, ResultExt, Result};
pub use self::geometry::{Vertex, Vector, Triangle, Color};
pub use self::index::Index;
pub use self::model::{Model, TriangularMesh, BuildModel};

pub mod errors;
pub mod geometry;
pub mod index;
pub mod model;
pub mod load;
pub mod build;

