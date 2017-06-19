#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector(pub f32, pub f32, pub f32);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color(pub f32, pub f32, pub f32);

pub trait Vertex {
    fn position(&self) -> Vector;
}

// Allow (x,y,z) pairs to work as vectors.
impl Vertex for Vector {
    fn position(&self) -> Vector { *self }
}

