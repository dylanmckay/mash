use {Model, Vertex, Vector, Index, Error};
use load::Format;
use tobj;

use std::path::Path;

/// A wavefront model.
pub struct Wavefront {
    models: Vec<tobj::Model>,
    materials: Vec<tobj::Material>,
}

/// A vertex.
pub struct WaveVertex {
    pub position: Vector,
    pub normal: Option<Vector>,
    pub texture_coords: Option<Vector>,
}

pub fn from_path(path: &Path) -> Result<Wavefront, Error> {
    let (models, materials) = tobj::load_obj(path).unwrap();

    Ok(Wavefront {
        models: models,
        materials: materials,
    })
}

impl Format for Wavefront
{
    type Vertex = WaveVertex;

    fn build_model<V,I>(self) -> Model<V,I>
        where V: Vertex, I: Index, V: From<WaveVertex> {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for model in self.models {
            for index in model.mesh.indices {
                // The different objects have indices relative to theirselves.
                // Adjust the index so that we have the absolute index across all objects.
                let abs_index = I::from(vertices.len() as u64 + index as u64);
                indices.push(abs_index);
            }

            let vertex_count = model.mesh.positions.len() / 3;
            for i in 0..vertex_count {
                let base_idx = i * 3;
                let position = build_vector(&model.mesh.positions, base_idx).unwrap();
                let normal = build_vector(&model.mesh.normals, i);
                let texture_coords = build_vector(&model.mesh.texcoords, i);

                let wave_vertex = WaveVertex {
                    position: position,
                    normal: normal,
                    texture_coords: texture_coords,
                };

                vertices.push(V::from(wave_vertex));
            }
        }

        Model {
            vertices: vertices,
            indices: indices,
        }
    }
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
    fn it_works() {
        let cube: Model<Vertex, u64> = Model::new(cube());

        assert_eq!(cube.vertices.len(), 24);
        assert_eq!(cube.indices.len(), 36);
    }
}

