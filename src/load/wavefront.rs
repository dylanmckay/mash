//! Loader for the Wavefront `.obj` file format.

use {Model, TriangularMesh, BuildModel, Vector, Color, Index, Error};
use tobj;

use std::io::BufRead;
use std::path::Path;

/// A wavefront model.
pub struct Wavefront {
    models: Vec<tobj::Model>,
    materials: Vec<tobj::Material>,
}

/// A vertex.
pub struct Vertex {
    pub position: Vector,
    pub normal: Option<Vector>,
    pub texture_coords: Option<Vector>,
}

/// A material.
pub struct Material<'a> {
    material: &'a tobj::Material,
}

/// A named object in a Wavefront file.
pub struct Object<'a> {
    wavefront: &'a Wavefront,
    model: &'a tobj::Model,
}

/// An iterator over all objects in a file.
pub struct Objects<'a> {
    wavefront: &'a Wavefront,
    models: ::std::slice::Iter<'a, tobj::Model>,
}

/// Loads a Wavefront `.obj` file from disk.
///
/// Material files will be automatically loaded.
pub fn from_path<S>(path: S) -> Result<Wavefront, Error>
    where S: AsRef<Path> {
    let (models, materials) = tobj::load_obj(path.as_ref())?;

    Ok(Wavefront {
        models: models,
        materials: materials,
    })
}

/// Loads a Wavefront `.obj` file from memory.
///
/// You must provide a closure that maps each material file path
/// to its corresponding byte stream.
pub fn from_memory<BO, BM>(reader: &mut BO,
                           material_loader: impl Fn(&Path) -> BM)
    -> Result<Wavefront, Error>
    where BO: BufRead, BM: BufRead {
    let (models, materials) = tobj::load_obj_buf(reader, |mtl_path| {
        let mut mtl_reader = material_loader(mtl_path);
        tobj::load_mtl_buf(&mut mtl_reader)
    })?;

    Ok(Wavefront {
        models: models,
        materials: materials,
    })
}

impl Wavefront {
    /// All of the objects contained within the wavefront.
    pub fn objects(&self) -> Objects {
        Objects { wavefront: self, models: self.models.iter() }
    }
}

impl<'a> Object<'a> {
    /// Gets the name of the object.
    pub fn name(&self) -> &str { &self.model.name }

    /// Gets the material associated with the object.
    pub fn material(&self) -> Option<Material> {
        self.model.mesh.material_id.map(|id| {
            Material { material: &self.wavefront.materials[id] }
        })
    }
}

impl<'a> Material<'a> {
    /// Gets the name of the material.
    pub fn name(&self) -> &str { &self.material.name }

    /// Gets the ambient color.
    pub fn ambient_color(&self) -> Color {
        Color(self.material.ambient[0], self.material.ambient[1], self.material.ambient[2])
    }

    /// Gets the diffuse color.
    pub fn diffuse_color(&self) -> Color {
        Color(self.material.diffuse[0], self.material.diffuse[1], self.material.diffuse[2])
    }

    /// Gets the specular color.
    pub fn specular_color(&self) -> Color {
        Color(self.material.specular[0], self.material.specular[1], self.material.specular[2])
    }

    /// Gets the shininess factor.
    pub fn shininess(&self) -> f32 { self.material.shininess }

    /// Gets the opacity factor.
    pub fn alpha(&self) -> f32 { self.material.dissolve }

    /// Gets the optical density.
    pub fn optical_density(&self) -> f32 { self.material.optical_density }

    /// Gets the ambient texture image file.
    pub fn ambient_texture(&self) -> &str { &self.material.ambient_texture }

    /// Gets the diffuse texture image file.
    pub fn diffuse_texture(&self) -> &str { &self.material.diffuse_texture }

    /// Gets the specular texture image file.
    pub fn specular_texture(&self) -> &str { &self.material.specular_texture }

    /// Gets the normal texture image file.
    pub fn normal_texture(&self) -> &str { &self.material.normal_texture }

    /// Gets the dissolve texture image file.
    pub fn dissolve_texture(&self) -> &str { &self.material.dissolve_texture }
}

impl BuildModel for Wavefront
{
    type Vertex = Vertex;

    fn build_model<V,I>(self) -> Result<Model<V,I>, Error>
        where V: ::Vertex, I: Index, V: From<Vertex> {
        let mut vertices: Vec<V> = Vec::new();
        let mut indices = Vec::new();

        for model in self.models {
            for &index in model.mesh.indices.iter() {
                // The different objects have indices relative to theirselves.
                // Adjust the index so that we have the absolute index across all objects.
                let abs_index = I::from_u64(vertices.len() as u64 + index as u64)?;
                indices.push(abs_index);
            }

            vertices.extend(build_vertices(&model.mesh).into_iter());
        }

        Ok(Model {
            mesh: TriangularMesh {
                vertices: vertices,
                indices: indices,
            }
        })
    }
}

fn build_vertices<V>(mesh: &tobj::Mesh) -> Vec<V>
    where V: From<Vertex> {

    let vertex_count = mesh.positions.len() / 3;
    (0..vertex_count).map(|i| {
        let base_idx = i * 3;
        let position = build_vector(&mesh.positions, base_idx).unwrap();
        let normal = build_vector(&mesh.normals, i);
        let texture_coords = build_vector(&mesh.texcoords, i);

        let wave_vertex = Vertex {
            position: position,
            normal: normal,
            texture_coords: texture_coords,
        };

        V::from(wave_vertex)
    }).collect()
}

impl<'a> BuildModel for Object<'a> {
    type Vertex = Vertex;

    fn build_model<V,I>(self) -> Result<Model<V,I>, Error>
        where V: ::Vertex, I: Index, V: From<Vertex> {
        let indices: Result<_,_> = self.model.mesh.indices.iter().map(|&index| I::from_u64(index as u64)).collect();
        let indices = indices?;
        let vertices = build_vertices(&self.model.mesh);

        Ok(Model {
            mesh: TriangularMesh {
                vertices: vertices,
                indices: indices,
            }
        })
    }
}

impl<'a> Iterator for Objects<'a> {
    type Item = Object<'a>;

    fn next(&mut self) -> Option<Object<'a>> {
        self.models.next().map(|m| Object { wavefront: self.wavefront, model: m })
    }
}

fn build_vector(elems: &Vec<f32>, base_idx: usize) -> Option<Vector> {
    if !elems.is_empty() {
        Some(Vector(elems[base_idx], elems[base_idx+1], elems[base_idx+2]))
    } else {
        None
    }
}

impl From<Vertex> for Vector {
    fn from(v: Vertex) -> Vector {
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
        let cube: Model<Vertex, u64> = Model::new(cube()).unwrap();

        assert_eq!(cube.mesh.vertices.len(), 24);
        assert_eq!(cube.mesh.indices.len(), 36);
    }

    #[test]
    fn can_enumerate_objects() {
        let cube = cube();
        assert_eq!(cube.objects().count(), 1);
        assert_eq!(cube.objects().next().unwrap().name(), "Cube");
    }

    #[test]
    fn can_build_object() {
        let cube: Model<Vertex, u64> = Model::new(cube().objects().next().unwrap()).unwrap();
        assert_eq!(cube.mesh.vertices.len(), 24);
        assert_eq!(cube.mesh.indices.len(), 36);
    }
}

