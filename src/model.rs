use {Vertex, Index};
use load;

/// A 3D model.
pub struct Model<V: Vertex, I: Index> {
    /// The mesh that makes up the model.
    pub mesh: TriangularMesh<V, I>,
}

/// A triangular mesh.
pub struct TriangularMesh<V: Vertex, I: Index> {
    /// The vertex list.
    pub vertices: Vec<V>,
    /// The index list.
    pub indices: Vec<I>,
}

impl<V: Vertex, I: Index> Model<V,I> {
    /// Creates a new model.
    pub fn new<F>(format: F) -> Self
        where F: load::Format, V: From<F::Vertex> {
        format.build_model()
    }
}

