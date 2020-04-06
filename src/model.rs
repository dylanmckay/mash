//! Format-independent mesh representation.
use {Vertex, Index, Triangle, Error};

use std::iter::FromIterator;
use std::fmt;

/// A 3D model.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Model<V: Vertex, I: Index> {
    /// The mesh that makes up the model.
    pub mesh: TriangularMesh<V, I>,
}

/// Something which we can build a model out of.
pub trait BuildModel {
    /// The vertex type that we need to convert from.
    type Vertex;

    fn build_model<V,I>(self) -> Result<Model<V,I>, Error>
        where V: Vertex, I: Index, V: From<Self::Vertex>;
}

/// A triangular mesh.
#[derive(Clone, PartialEq, Eq)]
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
    /// Creates an empty mesh.
    pub fn empty() -> Self {
        Model { mesh: TriangularMesh::empty() }
    }

    /// Creates a new model.
    pub fn new<F>(builder: F) -> Result<Self, Error>
        where F: BuildModel, V: From<F::Vertex> {
        builder.build_model()
    }
}

impl<V: Vertex, I: Index> fmt::Debug for TriangularMesh<V,I> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        // Create a trait for debug printing.
        #[derive(Debug)]
        struct TriangularMesh {
            pub vertex_count: usize,
            pub index_count: usize,
        }

        TriangularMesh {
            vertex_count: self.vertices.len(),
            index_count: self.indices.len(),
        }.fmt(fmt)
    }
}

impl<V: Vertex, I: Index> TriangularMesh<V,I> {
    /// Creates an empty triangular mesh.
    pub fn empty() -> Self {
        TriangularMesh { vertices: Vec::new(), indices: Vec::new() }
    }

    /// Gets all of the triangles in a mesh.
    pub fn triangles(&self) -> Triangles<V,I> {
        Triangles { mesh: self, indices: self.indices.iter() }
    }
}

impl<V,I> FromIterator<Triangle<V>> for TriangularMesh<V,I>
    where V: Vertex, I: Index
{
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item=Triangle<V>> {
        let triangles: Vec<_> = iter.into_iter().collect();

        let mut vertices: Vec<_> = triangles.iter().flat_map(|tri| tri.vertices.iter().cloned()).collect();
        vertices.sort_by(|a, b| a.partial_cmp(b).unwrap());
        vertices.dedup();

        let indices = triangles.iter().flat_map(|tri| {
            tri.vertices.iter().map(|vert| {
                let index: u64 = vertices.binary_search_by(|a| a.partial_cmp(vert).unwrap()).unwrap() as u64;
                I::from_u64(index).expect("index type too small for mesh")
            })
        }).collect();

        TriangularMesh { vertices: vertices, indices: indices }
    }
}

impl<'a, V: Vertex+'a, I: Index+'a> Iterator for Triangles<'a, V, I> {
    type Item = Triangle<V>;

    fn next(&mut self) -> Option<Triangle<V>> {
        if let Some(i1) = self.indices.next() {
            let i2 = self.indices.next().expect("expected at least two more indices");
            let i3 = self.indices.next().expect("expected at least one more index");

            let vertices: Vec<_> = [i1,i2,i3].iter().map(|&&idx| {
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
    use {TriangularMesh, Vector, Triangle};
    use build;

    #[test]
    fn can_enumerate_triangles() {
        let cube: TriangularMesh<Vector,u64> = build::unit_cube();
        assert_eq!(cube.triangles().count(), 12);
    }

    #[test]
    fn can_build_out_of_triangles() {
        let triangles = vec![
            Triangle { vertices: [Vector(1.0, 1.0, 1.0), Vector(2.0, 2.0, 2.0), Vector(3.0,3.0,3.0)] },
            Triangle { vertices: [Vector(5.0, 5.0, 5.0), Vector(6.0, 6.0, 6.0), Vector(7.0,7.0,7.0)] },
        ];

        let mesh: TriangularMesh<_, u16> = triangles.into_iter().collect();
        let processed_triangles: Vec<_> = mesh.triangles().collect();

        assert_eq!(processed_triangles, vec![
            Triangle { vertices: [Vector(1.0, 1.0, 1.0), Vector(2.0, 2.0, 2.0), Vector(3.0,3.0,3.0)] },
            Triangle { vertices: [Vector(5.0, 5.0, 5.0), Vector(6.0, 6.0, 6.0), Vector(7.0,7.0,7.0)] },
        ]);
    }
}

