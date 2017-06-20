use {Vertex, Index, Triangle, Error};
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

/// All of the triangles in a mesh.
pub struct Triangles<'a, V: Vertex+'a, I: Index+'a>
{
    mesh: &'a TriangularMesh<V,I>,
    indices: ::std::slice::Iter<'a, I>,
}

impl<V: Vertex, I: Index> Model<V,I> {
    /// Creates a new model.
    pub fn new<F>(format: F) -> Result<Self, Error>
        where F: load::Format, V: From<F::Vertex> {
        format.build_model()
    }
}

impl<V: Vertex, I: Index> TriangularMesh<V,I> {
    /// Gets all of the triangles in a mesh.
    pub fn triangles(&self) -> Triangles<V,I> {
        Triangles { mesh: self, indices: self.indices.iter() }
    }
}

impl<'a, V: Vertex+'a, I: Index+'a> Iterator for Triangles<'a, V, I> {
    type Item = Triangle<V>;

    fn next(&mut self) -> Option<Triangle<V>> {
        if let Some(i1) = self.indices.next() {
            let i2 = self.indices.next().expect("expected at least two more indices");
            let i3 = self.indices.next().expect("expected at least one more index");

            let vertices: Vec<_> = [i1,i2,i3].into_iter().map(|&&idx| {
                let idx: u64 = idx.into();
                self.mesh.vertices[idx as usize].clone()
            }).collect();

            Some(Triangle {
                vertices: [vertices[0].clone(), vertices[1].clone(), vertices[2].clone()],
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use {TriangularMesh, Vector};
    use build;

    #[test]
    fn can_enumerate_triangles() {
        let cube: TriangularMesh<Vector,u64> = build::unit_cube();
        assert_eq!(cube.triangles().count(), 12);
    }
}

