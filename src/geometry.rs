//! Geometric type definitions.

use std::fmt::Debug;

/// A 3-dimensional vector.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Vector(pub f32, pub f32, pub f32);

/// A color.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Color(pub f32, pub f32, pub f32);

/// A vertex.
pub trait Vertex : Clone + Debug + PartialEq + PartialOrd {
    /// Get the position of the vertex.
    fn position(&self) -> Vector;
}

/// A triangle.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Triangle<V: Vertex> {
    /// The vertices that make up the triangle.
    pub vertices: [V; 3],
}

// Allow (x,y,z) pairs to work as vectors.
impl Vertex for Vector {
    fn position(&self) -> Vector { *self }
}

