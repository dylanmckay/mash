use {Model, Vertex, Vector, Index, Error};
use load::Format;
use tobj;

use std::path::Path;

/// A wavefront model.
pub struct Wavefront {
    models: Vec<tobj::Model>,
    #[allow(dead_code)]
    materials: Vec<tobj::Material>,
}

/// A vertex.
pub struct WaveVertex {
    pub position: Vector,
    pub normal: Option<Vector>,
    pub texture_coords: Option<Vector>,
}

/// A named object in a Wavefront file.
pub struct Object<'a> {
    model: &'a tobj::Model,
}

/// An iterator over all objects in a file.
pub struct Objects<'a> {
    models: ::std::slice::Iter<'a, tobj::Model>,
}

/// Loads a Wavefront `.obj` file from disk.
pub fn from_path(path: &Path) -> Result<Wavefront, Error> {
    let (models, materials) = tobj::load_obj(path).unwrap();

    Ok(Wavefront {
        models: models,
        materials: materials,
    })
}

impl Wavefront {
    pub fn objects(&self) -> Objects {
        Objects { models: self.models.iter() }
    }
}

impl Format for Wavefront
{
    type Vertex = WaveVertex;

    fn build_model<V,I>(self) -> Model<V,I>
        where V: Vertex, I: Index, V: From<WaveVertex> {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for model in self.models {
            for &index in model.mesh.indices.iter() {
                // The different objects have indices relative to theirselves.
                // Adjust the index so that we have the absolute index across all objects.
                let abs_index = I::from(vertices.len() as u64 + index as u64);
                indices.push(abs_index);
            }

            vertices.extend(build_vertices(&model.mesh));
        }

        Model {
            vertices: vertices,
            indices: indices,
        }
    }
}

fn build_vertices<V>(mesh: &tobj::Mesh) -> Vec<V>
    where V: From<WaveVertex> {

    let vertex_count = mesh.positions.len() / 3;
    (0..vertex_count).map(|i| {
        let base_idx = i * 3;
        let position = build_vector(&mesh.positions, base_idx).unwrap();
        let normal = build_vector(&mesh.normals, i);
        let texture_coords = build_vector(&mesh.texcoords, i);

        let wave_vertex = WaveVertex {
            position: position,
            normal: normal,
            texture_coords: texture_coords,
        };

        V::from(wave_vertex)
    }).collect()
}

impl<'a> Format for Object<'a> {
    type Vertex = WaveVertex;

    fn build_model<V,I>(self) -> Model<V,I>
        where V: Vertex, I: Index, V: From<WaveVertex> {
        let indices = self.model.mesh.indices.iter().map(|&index| I::from(index as u64)).collect();
        let vertices = build_vertices(&self.model.mesh);

        Model {
            vertices: vertices,
            indices: indices,
        }
    }
}

impl<'a> Iterator for Objects<'a> {
    type Item = Object<'a>;

    fn next(&mut self) -> Option<Object<'a>> {
        self.models.next().map(|m| Object { model: m })
    }
}

impl<'a> Object<'a> {
    pub fn name(&self) -> &str { &self.model.name }
}

fn build_vector(elems: &Vec<f32>, base_idx: usize) -> Option<Vector> {
    if !elems.is_empty() {
        Some(Vector(elems[base_idx], elems[base_idx+1], elems[base_idx+2]))
    } else {
        None
    }
}

impl From<WaveVertex> for Vector {
    fn from(v: WaveVertex) -> Vector {
        v.position
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use {Model, Vector};
    use std::path::Path;

    fn cube() -> Wavefront {
        from_path(Path::new("res/cube.obj")).unwrap()
    }

    pub type Vertex = Vector;

    #[test]
    fn can_build_file() {
        let cube: Model<Vertex, u64> = Model::new(cube());

        assert_eq!(cube.vertices.len(), 24);
        assert_eq!(cube.indices.len(), 36);
    }

    #[test]
    fn can_enumerate_objects() {
        let cube = cube();
        assert_eq!(cube.objects().count(), 1);
        assert_eq!(cube.objects().next().unwrap().name(), "Cube");
    }

    #[test]
    fn can_build_object() {
        let cube: Model<Vertex, u64> = Model::new(cube().objects().next().unwrap());
        assert_eq!(cube.vertices.len(), 24);
        assert_eq!(cube.indices.len(), 36);
    }
}

